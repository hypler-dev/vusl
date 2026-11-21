//! Inspect caller-supplied native reviews and independently selected policies offline.

mod policy;

use policy::Selection;
use serde::Serialize;
use serde_json::Value;
use std::ffi::OsString;
#[cfg(unix)]
use std::io::Read;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use vusl_core::{inspect, CheckedReview};
use vusl_reference_consumer::exact_mapping::{
    compare_exact_mappings, inspect_exact_mapping_with_relationship,
    try_inspect_exact_chain_with_relationship, ExactChainEdge,
};
use vusl_reference_consumer::MAX_EDGES;

const MAX_REVIEW_BYTES: usize = 65_536;
const MAX_POLICY_BYTES: usize = 4 * 1024 * 1024;
const MAX_REPORT_BYTES: usize = 32 * 1024 * 1024;

#[derive(Serialize)]
pub(crate) struct Diagnostic {
    format: &'static str,
    stage: &'static str,
    code: &'static str,
    input_index: Option<usize>,
    obligation_index: Option<usize>,
}

impl Diagnostic {
    fn new(stage: &'static str, code: &'static str, input_index: Option<usize>) -> Self {
        Self {
            format: "vusl.inspect-reviews-diagnostic.v0.1",
            stage,
            code,
            input_index,
            obligation_index: None,
        }
    }

    fn arguments(code: &'static str) -> Self {
        Self::new("arguments", code, None)
    }

    fn policy(code: &'static str, index: usize, obligation_index: Option<usize>) -> Self {
        Self {
            obligation_index,
            ..Self::new("selected_policy", code, Some(index))
        }
    }
}

enum Mode {
    Edge,
    Comparison,
    Chain,
}

struct Arguments {
    mode: Mode,
    reviews: Vec<PathBuf>,
    policies: Vec<PathBuf>,
}

fn arguments(mut args: impl Iterator<Item = OsString>) -> Result<Arguments, Diagnostic> {
    let mode = match args.next().as_deref().and_then(|arg| arg.to_str()) {
        Some("edge") => Mode::Edge,
        Some("comparison") => Mode::Comparison,
        Some("chain") => Mode::Chain,
        _ => return Err(Diagnostic::arguments("invalid_arguments")),
    };
    let mut reviews = Vec::new();
    let mut policies = Vec::new();
    while let Some(flag) = args.next() {
        let paths = match flag.to_str() {
            Some("--review") => &mut reviews,
            Some("--selected-policy") => &mut policies,
            _ => return Err(Diagnostic::arguments("invalid_arguments")),
        };
        let path = args
            .next()
            .map(PathBuf::from)
            .ok_or_else(|| Diagnostic::arguments("invalid_arguments"))?;
        if !path.is_absolute() {
            return Err(Diagnostic::arguments("invalid_arguments"));
        }
        if paths.len() == MAX_EDGES {
            return Err(Diagnostic::arguments("invalid_input_count"));
        }
        paths.push(path);
    }
    let expected = match mode {
        Mode::Edge if reviews.len() == 1 => 1,
        Mode::Comparison if reviews.len() == 2 => 1,
        Mode::Chain if (2..=MAX_EDGES).contains(&reviews.len()) => reviews.len(),
        _ => return Err(Diagnostic::arguments("invalid_review_count")),
    };
    if policies.len() != expected {
        return Err(Diagnostic::arguments("invalid_policy_count"));
    }
    Ok(Arguments {
        mode,
        reviews,
        policies,
    })
}

#[cfg(unix)]
fn read_file(path: &Path, maximum: usize) -> io::Result<Vec<u8>> {
    use std::fs::OpenOptions;
    use std::os::unix::fs::OpenOptionsExt;

    // Validate and read one descriptor. Nonblocking open prevents a FIFO
    // from stalling the command before its type can be checked.
    let file = OpenOptions::new()
        .read(true)
        .custom_flags(libc::O_NOFOLLOW | libc::O_NONBLOCK)
        .open(path)?;
    if !file.metadata()?.is_file() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "expected regular file",
        ));
    }
    let mut bytes = Vec::new();
    file.take((maximum + 1) as u64).read_to_end(&mut bytes)?;
    Ok(bytes)
}

#[cfg(not(unix))]
fn read_file(_: &Path, _: usize) -> io::Result<Vec<u8>> {
    Err(io::ErrorKind::Unsupported.into())
}

fn read_policy(path: &Path, index: usize) -> Result<Selection, Diagnostic> {
    let bytes = read_file(path, MAX_POLICY_BYTES)
        .map_err(|_| Diagnostic::policy("input_read_failed", index, None))?;
    if bytes.len() > MAX_POLICY_BYTES {
        return Err(Diagnostic::policy("resource_limit", index, None));
    }
    serde_json::from_slice(&bytes)
        .map_err(|_| Diagnostic::policy("invalid_selected_policy_json", index, None))
}

fn read_review(path: &Path, index: usize) -> Result<CheckedReview, Diagnostic> {
    let bytes = read_file(path, MAX_REVIEW_BYTES)
        .map_err(|_| Diagnostic::new("native_review", "input_read_failed", Some(index)))?;
    inspect(&bytes)
        .map_err(|error| Diagnostic::new("native_review", error.code().as_str(), Some(index)))
}

fn evaluate(args: &Arguments) -> Result<Value, Diagnostic> {
    let selections = args
        .policies
        .iter()
        .enumerate()
        .map(|(index, path)| read_policy(path, index))
        .collect::<Result<Vec<_>, _>>()?;
    let prepared = selections
        .iter()
        .enumerate()
        .map(|(index, selection)| selection.prepare(index))
        .collect::<Result<Vec<_>, _>>()?;
    let obligations: Vec<_> = prepared
        .iter()
        .map(policy::PolicyInputs::obligations)
        .collect();
    let policies = prepared
        .iter()
        .zip(&obligations)
        .map(|(input, terms)| input.exact(terms))
        .collect::<Result<Vec<_>, _>>()?;
    let relationships = prepared
        .iter()
        .map(policy::PolicyInputs::relationship)
        .collect::<Result<Vec<_>, _>>()?;
    let reviews = args
        .reviews
        .iter()
        .enumerate()
        .map(|(index, path)| read_review(path, index))
        .collect::<Result<Vec<_>, _>>()?;

    match args.mode {
        Mode::Edge => Ok(inspect_exact_mapping_with_relationship(
            &reviews[0],
            &policies[0],
            &relationships[0],
        )),
        Mode::Comparison => Ok(compare_exact_mappings(
            &reviews[0],
            &reviews[1],
            &policies[0],
            &relationships[0],
        )),
        Mode::Chain => {
            let edges: Vec<_> = reviews
                .iter()
                .zip(&policies)
                .zip(&relationships)
                .map(|((review, policy), relationship)| ExactChainEdge {
                    review,
                    policy,
                    relationship,
                })
                .collect();
            try_inspect_exact_chain_with_relationship(&edges)
                .map_err(|error| Diagnostic::arguments(error.as_str()))
        }
    }
}

fn execute() -> Result<(), Diagnostic> {
    let args = arguments(std::env::args_os().skip(1))?;
    let report = evaluate(&args)?;
    let mut output = serde_json::to_vec(&report)
        .map_err(|_| Diagnostic::new("output", "report_failed", None))?;
    output.push(b'\n');
    if output.len() > MAX_REPORT_BYTES {
        return Err(Diagnostic::new("output", "resource_limit", None));
    }
    let mut stdout = io::stdout().lock();
    stdout
        .write_all(&output)
        .and_then(|_| stdout.flush())
        .map_err(|_| Diagnostic::new("output", "output_write_failed", None))
}

fn main() -> std::process::ExitCode {
    match execute() {
        Ok(()) => std::process::ExitCode::SUCCESS,
        Err(diagnostic) => {
            let mut stderr = io::stderr().lock();
            let _ = serde_json::to_writer(&mut stderr, &diagnostic);
            let _ = stderr.write_all(b"\n");
            std::process::ExitCode::FAILURE
        }
    }
}
