# swe-edge-egress-subprocess

> **TLDR:** Safe outbound subprocess execution for swe-edge — exhaustive result variants (Completed, Denied, TimedOut, SpawnFailed, IsolationFailed), allow-list enforcement, and configurable isolation profiles. See [Overview](scm/docs/README.md) for details.

Subprocess execution egress port for `swe-edge`. Wraps OS process spawning in an async-safe,
exhaustively-typed API with allow-list enforcement and isolation profiles.

## Quick Start

```rust
use swe_edge_egress_subprocess::{SubprocessSvc, SubprocessArgs, SubprocessResult};

let runner = SubprocessSvc::from_config(&config).await?;
let args = SubprocessArgs::builder().command("jq").args([".key"]).build()?;
match runner.run(args).await {
    SubprocessResult::Completed { exit_code, stdout, .. } => { /* use stdout */ }
    SubprocessResult::Denied { command } => eprintln!("{command} not in allow-list"),
    SubprocessResult::TimedOut { timeout_ms } => eprintln!("timed out after {timeout_ms}ms"),
    SubprocessResult::SpawnFailed { reason } => eprintln!("spawn error: {reason}"),
    SubprocessResult::IsolationFailed { profile, reason } => eprintln!("{profile}: {reason}"),
}
```

## Documentation

| Document | Description |
|----------|-------------|
| [Overview](scm/docs/README.md) | WHAT + WHY — capabilities and design rationale |
