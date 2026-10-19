# Extraction provenance

VUSL began as the RUSL design in [Rangoon](https://github.com/hypler-dev/rangoon/tree/d23b2b73e7d0436325a03264144ac87c0056c10b). This repository separates specification ownership from the consuming application. The initial extraction includes the October 18, 2026 VUSL naming edits and link/ownership corrections; it is not a byte-identical copy of the earlier Markdown.

[The manifest](conformance/manifest.json) records original source paths and SHA-256 identities at the pinned Rangoon commit, alongside destination paths and initial-extraction hashes. Source paths retain the original `docs/rusl.md` and `docs/rusl-conformance.md` names. Destination Markdown was edited during extraction and is not asserted to exist in the source commit. All three JSON vector files are byte-identical to the pinned Rangoon base. Their historical `rusl` filenames, language/profile tokens, digest domain and acceptance IDs remain unchanged. Changing those values is a behavioral migration, not branding.

The experimental Rust parser and Python reference remain in Rangoon. At extraction, the locked/offline D1 Rust suite passed eight tests and the Python reference passed fourteen tests. Those results cover the existing offline subset, not provider integration, deployment readback, semantic-equivalence detection, authorization, verified effects or this repository's broader architecture families. Current source is linked by immutable commit; subsequent code changes need their own evidence.

The logo in `assets/vusl-logo.png` was generated for this project with OpenAI image generation and selected on October 18, 2026. It is a raster PNG on a light background, not an authored vector asset. No uniqueness or trademark clearance is asserted.

No private strategic assessment, credentials, native test logs, customer data, application database or engine runtime is included in this extraction.
