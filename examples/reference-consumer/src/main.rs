use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;
use vusl_reference_consumer::{Outcome, PolicyLabel, Report};

fn diagnostic(code: &str) {
    let _ = writeln!(io::stderr(), "{{\"format\":\"vusl.reference-consumer.diagnostic.v0.1\",\"layer\":\"example_io\",\"code\":\"{code}\",\"catalog\":\"docs/diagnostics.md\"}}");
}
fn read_input(path: &str) -> Result<Vec<u8>, &'static str> {
    let mut bytes = Vec::with_capacity(65_537);
    if path == "-" {
        io::stdin().take(65_537).read_to_end(&mut bytes)
    } else {
        File::open(path)
            .map_err(|_| "input_read_failed")?
            .take(65_537)
            .read_to_end(&mut bytes)
    }
    .map_err(|_| "input_read_failed")?;
    Ok(bytes)
}
fn write_report(report: &Report, text: bool) -> Result<(), &'static str> {
    // Assemble and bound the entire stream before its first write.
    let json = serde_json::to_string(report).map_err(|_| "report_failed")?;
    let payload = if text {
        format!("scope: edge_only\ncomposition_assessment: not_assessed\nauthority: none\n{json}\n")
    } else {
        format!("{json}\n")
    };
    if payload.len() > 262_144 {
        return Err("report_failed");
    }
    let mut stdout = io::stdout().lock();
    stdout
        .write_all(payload.as_bytes())
        .and_then(|_| stdout.flush())
        .map_err(|_| "output_write_failed")
}
fn execute() -> Result<i32, &'static str> {
    let mut args = std::env::args_os().skip(1);
    let (mut scenario, mut input, mut policy, mut format) = (None, None, None, None);
    while let Some(flag) = args.next() {
        let flag = flag.into_string().map_err(|_| "invalid_arguments")?;
        let value = args
            .next()
            .ok_or("invalid_arguments")?
            .into_string()
            .map_err(|_| "invalid_arguments")?;
        if value.is_empty() {
            return Err("invalid_arguments");
        }
        match flag.as_str() {
            "--scenario" if scenario.is_none() => scenario = Some(value),
            "--input" if input.is_none() => input = Some(value),
            "--policy" if policy.is_none() => policy = Some(value),
            "--format" if format.is_none() && matches!(value.as_str(), "json" | "text") => {
                format = Some(value)
            }
            _ => return Err("invalid_arguments"),
        }
    }
    // Validate every closed option and mode before opening the input.
    let outcome = match (scenario, input, policy) {
        (Some(id), None, None) => {
            let name = vusl_reference_consumer::scenario_name(&id).ok_or("invalid_arguments")?;
            let items = vusl_reference_consumer::scenario(name).ok_or("invalid_arguments")?;
            vusl_reference_consumer::evaluate_edges("scenario", name, &items)
        }
        (None, Some(path), Some(label)) => {
            let label = PolicyLabel::parse(&label).ok_or("invalid_arguments")?;
            if path != "-" && !Path::new(&path).is_absolute() {
                return Err("invalid_arguments");
            }
            let bytes = read_input(&path)?;
            match vusl_reference_consumer::fixed_policy(label) {
                Ok(policy) => {
                    vusl_reference_consumer::evaluate_input_with_policy(&bytes, &policy, label)
                }
                Err(_) => vusl_reference_consumer::evaluate_edges(
                    "input",
                    "single_input",
                    &[(&bytes, label)],
                ),
            }
        }
        _ => return Err("invalid_arguments"),
    };
    match outcome {
        Outcome::Operational(code) => Err(code),
        Outcome::Complete(report) => {
            write_report(&report, format.as_deref() == Some("text"))?;
            Ok(vusl_reference_consumer::exit_code(&report))
        }
    }
}
fn main() {
    let code = match execute() {
        Ok(code) => code,
        Err(code) => {
            diagnostic(code);
            1
        }
    };
    std::process::exit(code);
}
