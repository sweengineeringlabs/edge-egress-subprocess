//! `ExtensionRunner` — adapts a [`SubprocessRunnerExtension`] into [`SubprocessRunner`].

use futures::future::BoxFuture;

use crate::api::traits::subprocess::runner::SubprocessRunner;
use crate::api::traits::subprocess::runner_extension::SubprocessRunnerExtension;
use crate::api::types::subprocess::args::SubprocessArgs;
use crate::api::types::subprocess::result::SubprocessResult;

/// Bridges a downstream [`SubprocessRunnerExtension`] into the [`SubprocessRunner`] contract.
///
/// Created by [`SubprocessSvc::with_runner`]; callers receive `impl SubprocessRunner`
/// and never name this type directly.
pub(crate) struct ExtensionRunner<E: SubprocessRunnerExtension> {
    ext: E,
}

impl<E: SubprocessRunnerExtension> ExtensionRunner<E> {
    /// Wrap `ext` as a [`SubprocessRunner`].
    pub(crate) fn new(ext: E) -> Self {
        Self { ext }
    }
}

impl<E: SubprocessRunnerExtension> SubprocessRunner for ExtensionRunner<E> {
    fn run(&self, args: SubprocessArgs) -> BoxFuture<'_, SubprocessResult> {
        self.ext.run_extended(args)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[derive(Debug)]
    struct ExtensionRunnerStub;
    impl SubprocessRunnerExtension for ExtensionRunnerStub {
        fn run_extended(&self, args: SubprocessArgs) -> BoxFuture<'_, SubprocessResult> {
            let cmd = args.argv.first().cloned().unwrap_or_default();
            Box::pin(async move { SubprocessResult::Denied { command: cmd } })
        }
    }

    /// @covers: new
    #[test]
    fn test_extension_runner_new_is_object_safe() {
        let _: Arc<dyn SubprocessRunner> = Arc::new(ExtensionRunner::new(ExtensionRunnerStub));
    }

    /// @covers: run
    #[tokio::test]
    async fn test_extension_runner_delegates_to_ext_run_extended() {
        let runner = ExtensionRunner::new(ExtensionRunnerStub);
        let args = SubprocessArgs::builder().argv(vec!["echo".into()]).build();
        let result = runner.run(args).await;
        assert!(matches!(result, SubprocessResult::Denied { .. }));
    }
}
