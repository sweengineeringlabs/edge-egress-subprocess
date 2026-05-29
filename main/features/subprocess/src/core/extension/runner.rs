//! `ExtensionRunner` — adapts a [`ProcessRunnerExtension`] into [`ProcessRunner`].

use futures::future::BoxFuture;

use crate::api::traits::runner::ProcessRunner;
use crate::api::traits::runner_extension::ProcessRunnerExtension;
use crate::api::types::process::args::ProcessArgs;
use crate::api::types::process::outcome::ProcessResult;

/// Bridges a downstream [`ProcessRunnerExtension`] into the [`ProcessRunner`] contract.
///
/// Created by [`ProcessSvc::with_runner`]; callers receive `impl ProcessRunner`
/// and never name this type directly.
pub(crate) struct ExtensionRunner<E: ProcessRunnerExtension> {
    ext: E,
}

impl<E: ProcessRunnerExtension> ExtensionRunner<E> {
    /// Wrap `ext` as a [`ProcessRunner`].
    pub(crate) fn new(ext: E) -> Self {
        Self { ext }
    }
}

impl<E: ProcessRunnerExtension> ProcessRunner for ExtensionRunner<E> {
    fn run(&self, args: ProcessArgs) -> BoxFuture<'_, ProcessResult> {
        self.ext.run_extended(args)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[derive(Debug)]
    struct ExtensionRunnerStub;
    impl ProcessRunnerExtension for ExtensionRunnerStub {
        fn run_extended(&self, args: ProcessArgs) -> BoxFuture<'_, ProcessResult> {
            let cmd = args.argv.first().cloned().unwrap_or_default();
            Box::pin(async move { ProcessResult::Denied { command: cmd } })
        }
    }

    /// @covers: new
    #[test]
    fn test_extension_runner_new_is_object_safe() {
        let _: Arc<dyn ProcessRunner> = Arc::new(ExtensionRunner::new(ExtensionRunnerStub));
    }

    /// @covers: run
    #[tokio::test]
    async fn test_extension_runner_delegates_to_ext_run_extended() {
        let runner = ExtensionRunner::new(ExtensionRunnerStub);
        let args = ProcessArgs::builder().argv(vec!["echo".into()]).build();
        let result = runner.run(args).await;
        assert!(matches!(result, ProcessResult::Denied { .. }));
    }
}
