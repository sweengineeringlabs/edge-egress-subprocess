# swe-edge-egress-subprocess

## WHAT

Outbound subprocess execution for swe-edge services — safe async binary invocation with exhaustive
error handling, allow-list enforcement, isolation profiles, and configurable timeouts.

Key capabilities:

- **`SubprocessRunner`** — core trait: `run(args: SubprocessArgs) → BoxFuture<SubprocessResult>`; never returns `Err`; all outcomes are encoded as result variants
- **`SubprocessResult`** — enum: `Completed { exit_code, stdout, stderr }`, `Denied { command }`, `TimedOut { timeout_ms }`, `SpawnFailed { reason }`, `IsolationFailed { profile, reason }`
- **`SubprocessArgs`** / **`SubprocessArgsBuilder`** — VO: command, args, env, stdin, timeout per call
- **`SubprocessConfig`** — top-level config from `[subprocess]` TOML; allow-list, isolation profile, timeout defaults
- **`AllowList`** — trait enforcing which commands can be executed; policy pluggable
- **`IsolationProfile`** — trait for OS-level sandboxing (Linux: seccomp, namespaces)
- **`SweEdgeEgressProcess`** — SAF extension hook for custom subprocess handling

## WHY

| Problem | Solution |
|---------|----------|
| External binary invocation returns opaque `io::Error`; callers can't distinguish timeout from spawn failure | `SubprocessResult` enum covers every outcome; callers match exhaustively without losing context |
| Arbitrary commands executed from user input are a command injection risk | `AllowList` trait; commands not in the allow-list produce `Denied { command }` without spawning |
| Subprocess hangs indefinitely in async context | Per-call timeout via `SubprocessArgs`; produces `TimedOut { timeout_ms }` with the configured limit for observability |
| Sandboxing logic tied to business handlers | `IsolationProfile` is a pluggable trait; Linux namespacing/seccomp applied in the runner, invisible to the handler |
| Diamond dep conflicts when subprocess types change | One crate, one tag — all consumers pin the same version; kgraph detects conflicts pre-commit |
