# Architecture — edge-egress-subprocess

## Sequence

> A handler builds `SubprocessArgs`, submits them to `SubprocessRunner`; the runner checks the allowlist, enforces the timeout, spawns the process, and returns an exhaustive result.

```mermaid
sequenceDiagram
    participant Handler
    participant SubprocessSvc
    participant SubprocessRunner
    participant AllowList
    participant OS

    Handler->>SubprocessSvc: runner()
    SubprocessSvc-->>Handler: impl SubprocessRunner

    Handler->>SubprocessSvc: config.with_argv(["ffmpeg", "-i", "in.mp4"])
    SubprocessSvc-->>Handler: SubprocessArgs

    Handler->>SubprocessRunner: run(args)
    SubprocessRunner->>AllowList: is_allowed(argv[0])
    AllowList-->>SubprocessRunner: true / false

    alt allowed
        SubprocessRunner->>OS: tokio::process::Command::spawn()
        OS-->>SubprocessRunner: Child process
        SubprocessRunner->>OS: wait with timeout_ms
        OS-->>SubprocessRunner: exit code + stdout + stderr
        SubprocessRunner-->>Handler: SubprocessResult::Completed{exit_code, stdout, stderr}
    else denied
        SubprocessRunner-->>Handler: SubprocessResult::Denied{command}
    else timed out
        SubprocessRunner-->>Handler: SubprocessResult::TimedOut{timeout_ms}
    else spawn failed
        SubprocessRunner-->>Handler: SubprocessResult::SpawnFailed{reason}
    end
```

## Data Flow

> `SubprocessArgs` (argv, timeout, allowlist) enters the runner; the exhaustive `SubprocessResult` exits — no hidden error paths.

```mermaid
flowchart LR
    A["SubprocessArgs\n───────────\nargv: Vec<String>\ntimeout_ms: u64\nallow_commands: Vec<String>\ncwd: Option<PathBuf>\nenv: HashMap"] --> B["AllowList\n::is_allowed(argv0)"]

    B -->|denied| X["SubprocessResult\n::Denied{command}"]
    B -->|allowed| C["tokio::process\n::Command::spawn()"]

    C -->|spawn error| Y["SubprocessResult\n::SpawnFailed{reason}"]
    C -->|spawned| D["wait with\ntimeout_ms budget"]

    D -->|exit before timeout| E["exit_code, stdout, stderr\n(capped at output_bytes_cap)"]
    D -->|timeout elapsed| Z["SubprocessResult\n::TimedOut{timeout_ms}"]

    E --> F["SubprocessResult\n::Completed{exit_code,\nstdout, stderr}"]
```
