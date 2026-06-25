# Continuous Integration

Ryver runs GitHub Actions checks on pull requests and pushes to `main`.

## Rust Workflow

The Rust workflow enforces the baseline local development checks:

- `cargo fmt --all -- --check`;
- `cargo clippy --all-targets --all-features -- -D warnings`;
- `cargo test --all-targets --all-features`.

## Security Workflow

The security workflow runs:

- Semgrep with Rust, security-audit, and secrets rulesets;
- RustSec dependency auditing through `cargo audit`.

SonarQube or SonarCloud can be added later if the project is connected to a Sonar organization and the required GitHub secrets are available. Until then, Clippy, Semgrep, and RustSec provide checks that run without external service setup.
