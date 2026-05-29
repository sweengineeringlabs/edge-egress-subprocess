//! Integration tests for the SubprocessRunner public API.

use std::sync::Arc;

use swe_edge_egress_subprocess::{
    SubprocessArgs, SubprocessResult, SubprocessRunner, SubprocessSvc,
};

// ── platform helpers ──────────────────────────────────────────────────────────

/// Returns `argv` and `allow_commands` to run `echo <msg>` on the current OS.
#[cfg(unix)]
fn echo_invocation(msg: &str) -> (Vec<String>, Vec<String>) {
    (vec!["echo".into(), msg.into()], vec!["echo".into()])
}

#[cfg(windows)]
fn echo_invocation(msg: &str) -> (Vec<String>, Vec<String>) {
    (
        vec!["cmd".into(), "/C".into(), format!("echo {msg}")],
        vec!["cmd".into()],
    )
}

// ── deny (no subprocess spawned) ─────────────────────────────────────────────

/// @covers: subprocess_runner — empty allow-list denies any command.
#[tokio::test]
async fn test_subprocess_runner_denied_when_allow_list_is_empty() {
    let runner = SubprocessSvc::runner();
    let args = SubprocessArgs::builder()
        .argv(vec!["echo".into(), "hello".into()])
        .build();
    let result = runner.run(args).await;
    assert!(
        matches!(result, SubprocessResult::Denied { .. }),
        "empty allow-list must deny all commands; got {result:?}",
    );
}

/// @covers: subprocess_runner — command not in allow-list is denied.
#[tokio::test]
async fn test_subprocess_runner_denied_when_command_not_in_allow_list() {
    let runner = SubprocessSvc::runner();
    let args = SubprocessArgs::builder()
        .argv(vec!["echo".into(), "hello".into()])
        .allow_commands(vec!["cat".into()])
        .build();
    let result = runner.run(args).await;
    let SubprocessResult::Denied { command } = result else {
        panic!("expected Denied; got {result:?}");
    };
    assert_eq!(
        command, "echo",
        "denied command must be the normalised argv[0]"
    );
}

/// @covers: subprocess_runner — argv path prefix is stripped before allow-list check.
#[cfg(unix)]
#[tokio::test]
async fn test_subprocess_runner_allow_list_matches_after_path_strip() {
    let runner = SubprocessSvc::runner();
    let args = SubprocessArgs::builder()
        .argv(vec!["/usr/bin/echo".into(), "hi".into()])
        .allow_commands(vec!["echo".into()]) // basename only in allow-list
        .timeout_ms(5_000)
        .build();
    let result = runner.run(args).await;
    assert!(
        matches!(result, SubprocessResult::Completed { .. }),
        "path-prefixed command must match basename in allow-list; got {result:?}",
    );
}

// ── spawn failed ──────────────────────────────────────────────────────────────

/// @covers: subprocess_runner — nonexistent binary returns SpawnFailed.
#[tokio::test]
async fn test_subprocess_runner_spawn_failed_for_nonexistent_binary() {
    let runner = SubprocessSvc::runner();
    let args = SubprocessArgs::builder()
        .argv(vec!["__swe_nonexistent_binary_edge__".into()])
        .allow_commands(vec!["__swe_nonexistent_binary_edge__".into()])
        .build();
    let result = runner.run(args).await;
    assert!(
        matches!(result, SubprocessResult::SpawnFailed { .. }),
        "nonexistent binary must return SpawnFailed; got {result:?}",
    );
}

// ── happy path ────────────────────────────────────────────────────────────────

/// @covers: subprocess_runner — successful subprocess returns Completed with exit_code 0.
#[tokio::test]
async fn test_subprocess_runner_completed_with_exit_code_zero() {
    let runner = SubprocessSvc::runner();
    let (argv, allow) = echo_invocation("hello");
    let args = SubprocessArgs::builder()
        .argv(argv)
        .allow_commands(allow)
        .timeout_ms(5_000)
        .build();
    let result = runner.run(args).await;
    let SubprocessResult::Completed {
        exit_code, stdout, ..
    } = result
    else {
        panic!("expected Completed; got {result:?}");
    };
    assert_eq!(exit_code, 0, "echo must exit with code 0");
    assert!(
        stdout.contains("hello"),
        "stdout must contain the echoed message; got {stdout:?}"
    );
}

/// @covers: subprocess_runner — child never inherits parent env (fail-closed).
#[cfg(unix)]
#[tokio::test]
async fn test_subprocess_runner_child_does_not_inherit_parent_env() {
    // Set a sentinel in the parent environment; the child must not see it.
    std::env::set_var("__SWE_EDGE_SUBPROCESS_SENTINEL__", "should_not_appear");

    let runner = SubprocessSvc::runner();
    let args = SubprocessArgs::builder()
        .argv(vec![
            "sh".into(),
            "-c".into(),
            "echo ${__SWE_EDGE_SUBPROCESS_SENTINEL__:-absent}".into(),
        ])
        .allow_commands(vec!["sh".into()])
        .timeout_ms(5_000)
        .build();
    let result = runner.run(args).await;

    std::env::remove_var("__SWE_EDGE_SUBPROCESS_SENTINEL__");

    let SubprocessResult::Completed { stdout, .. } = result else {
        panic!("expected Completed; got {result:?}");
    };
    assert!(
        stdout.contains("absent"),
        "child must not inherit parent env var; stdout was {stdout:?}",
    );
}

/// @covers: subprocess_runner — env vars passed explicitly are visible to the child.
#[cfg(unix)]
#[tokio::test]
async fn test_subprocess_runner_explicit_env_vars_visible_to_child() {
    let runner = SubprocessSvc::runner();
    let mut env = std::collections::HashMap::new();
    env.insert("MY_VAR".into(), "visible".into());
    let args = SubprocessArgs::builder()
        .argv(vec!["sh".into(), "-c".into(), "echo $MY_VAR".into()])
        .allow_commands(vec!["sh".into()])
        .env(env)
        .timeout_ms(5_000)
        .build();
    let result = runner.run(args).await;
    let SubprocessResult::Completed { stdout, .. } = result else {
        panic!("expected Completed; got {result:?}");
    };
    assert!(
        stdout.contains("visible"),
        "explicitly passed env var must be visible to child; stdout was {stdout:?}",
    );
}

// ── timeout ───────────────────────────────────────────────────────────────────

/// @covers: subprocess_runner — process killed and TimedOut returned when deadline exceeded.
#[cfg(unix)]
#[tokio::test]
async fn test_subprocess_runner_timed_out_when_deadline_exceeded() {
    let runner = SubprocessSvc::runner();
    let args = SubprocessArgs::builder()
        .argv(vec!["sleep".into(), "60".into()])
        .allow_commands(vec!["sleep".into()])
        .timeout_ms(100) // 100 ms — sleep 60 s will never finish
        .build();
    let result = runner.run(args).await;
    let SubprocessResult::TimedOut { timeout_ms } = result else {
        panic!("expected TimedOut; got {result:?}");
    };
    assert_eq!(timeout_ms, 100);
}

// ── byte-cap truncation ───────────────────────────────────────────────────────

/// @covers: subprocess_runner — stdout truncated to output_bytes_cap.
///
/// `echo` with a 200-byte argument produces >200 bytes of output; with a
/// cap of 50 the returned stdout must be ≤ 50 bytes.
#[cfg(unix)]
#[tokio::test]
async fn test_subprocess_runner_stdout_truncated_to_output_bytes_cap() {
    let long_arg = "A".repeat(200);
    let runner = SubprocessSvc::runner();
    let args = SubprocessArgs::builder()
        .argv(vec!["echo".into(), long_arg])
        .allow_commands(vec!["echo".into()])
        .output_bytes_cap(50)
        .timeout_ms(5_000)
        .build();
    let result = runner.run(args).await;
    let SubprocessResult::Completed { stdout, stderr, .. } = result else {
        panic!("expected Completed; got {result:?}");
    };
    assert!(
        stdout.len() <= 50,
        "stdout must be truncated to cap=50; got {} bytes",
        stdout.len(),
    );
    assert!(stderr.is_empty(), "echo produces no stderr; got {stderr:?}",);
}

/// @covers: subprocess_runner — combined stdout+stderr respects byte cap.
#[cfg(unix)]
#[tokio::test]
async fn test_subprocess_runner_combined_output_respects_byte_cap() {
    // sh -c produces stdout via echo and stderr via >&2.
    let runner = SubprocessSvc::runner();
    let args = SubprocessArgs::builder()
        .argv(vec![
            "sh".into(),
            "-c".into(),
            "printf '%200s' | tr ' ' 'A'; printf '%200s' >&2 | tr ' ' 'B'".into(),
        ])
        .allow_commands(vec!["sh".into()])
        .output_bytes_cap(100)
        .timeout_ms(5_000)
        .build();
    let result = runner.run(args).await;
    let SubprocessResult::Completed { stdout, stderr, .. } = result else {
        panic!("expected Completed; got {result:?}");
    };
    assert!(
        stdout.len() + stderr.len() <= 100,
        "stdout ({}) + stderr ({}) must not exceed cap=100",
        stdout.len(),
        stderr.len(),
    );
}

// ── object safety ─────────────────────────────────────────────────────────────

/// @covers: SubprocessRunner — object-safe; storable as Arc<dyn SubprocessRunner>.
#[test]
fn test_subprocess_runner_can_be_stored_as_arc_dyn_trait() {
    let runner: Arc<dyn SubprocessRunner> = Arc::new(SubprocessSvc::runner());
    drop(runner);
}

/// @covers: subprocess_runner — run is callable through dyn trait.
#[tokio::test]
async fn test_subprocess_runner_run_callable_through_dyn_trait() {
    let runner: Arc<dyn SubprocessRunner> = Arc::new(SubprocessSvc::runner());
    let args = SubprocessArgs::builder().argv(vec!["echo".into()]).build(); // allow-list empty → Denied (no subprocess needed)
    let result = runner.run(args).await;
    assert!(matches!(result, SubprocessResult::Denied { .. }));
}
