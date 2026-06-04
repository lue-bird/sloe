struct SloeZedExtension();

impl zed_extension_api::Extension for SloeZedExtension {
    fn new() -> Self {
        SloeZedExtension()
    }
    fn language_server_command(
        &mut self,
        _: &zed_extension_api::LanguageServerId,
        worktree: &zed_extension_api::Worktree,
    ) -> zed_extension_api::Result<zed_extension_api::Command> {
        if let Some(path) = worktree.which("sloe") {
            Ok(zed_extension_api::Command::new(path).arg("lsp"))
        } else {
            Err("executable sloe not found in the PATH environment".into())
        }
    }
}

zed_extension_api::register_extension!(SloeZedExtension);
