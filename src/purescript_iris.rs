use std::{fs, path::Path};
use zed_extension_api::{self as zed, serde_json, settings::LspSettings, Result};

const BINARY_NAME: &str = "iris";
const GITHUB_REPOSITORY: &str = "purefunctor/purescript-iris";
const SETTINGS_SECTION: &str = "iris.server";

#[derive(Default)]
struct PurescriptIrisExtension {
    cached_binary_path: Option<String>,
}

impl PurescriptIrisExtension {
    fn language_server_binary_path(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<String> {
        if let Some(path) = worktree.which(BINARY_NAME) {
            return Ok(path);
        }

        if let Some(path) = self
            .cached_binary_path
            .as_ref()
            .filter(|path| Path::new(path).is_file())
        {
            return Ok(path.clone());
        }

        // Release assets, see https://iris-lang.com/docs/installation/
        let (os, arch) = zed::current_platform();
        let target = match (os, arch) {
            (zed::Os::Linux, zed::Architecture::X8664) => "x86_64-unknown-linux-musl",
            (zed::Os::Linux, zed::Architecture::Aarch64) => "aarch64-unknown-linux-musl",
            (zed::Os::Mac, _) => "universal-apple-darwin",
            (zed::Os::Windows, zed::Architecture::X8664) => "x86_64-pc-windows-msvc",
            _ => return Err(format!("no `{BINARY_NAME}` release for {os:?} {arch:?}")),
        };
        let (extension, file_type, binary_name) = match os {
            zed::Os::Windows => ("zip", zed::DownloadedFileType::Zip, "iris.exe"),
            _ => ("tar.gz", zed::DownloadedFileType::GzipTar, BINARY_NAME),
        };
        // The archive contains `iris-<target>/iris`.
        let archive_binary_path = format!("{BINARY_NAME}-{target}/{binary_name}");

        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );
        let release = match zed::latest_github_release(
            GITHUB_REPOSITORY,
            zed::GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        ) {
            Ok(release) => release,
            // Offline: use an earlier download.
            Err(error) => {
                let path = find_existing_binary(&archive_binary_path).ok_or(error)?;
                self.cached_binary_path = Some(path.clone());
                return Ok(path);
            }
        };

        let asset_name = format!("{BINARY_NAME}-{target}.{extension}");
        let asset = release
            .assets
            .iter()
            .find(|asset| asset.name == asset_name)
            .ok_or_else(|| format!("no asset found matching {asset_name:?}"))?;

        let version_dir = format!("{BINARY_NAME}-{}", release.version);
        let binary_path = format!("{version_dir}/{archive_binary_path}");

        if !Path::new(&binary_path).is_file() {
            zed::set_language_server_installation_status(
                language_server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );
            zed::download_file(&asset.download_url, &version_dir, file_type)
                .map_err(|error| format!("failed to download {asset_name}: {error}"))?;
            zed::make_file_executable(&binary_path)?;

            // Remove older releases.
            let entries =
                fs::read_dir(".").map_err(|error| format!("failed to list directory: {error}"))?;
            for entry in entries.flatten() {
                let name = entry.file_name();
                let name = name.to_string_lossy();
                if name.starts_with(&format!("{BINARY_NAME}-")) && name != version_dir {
                    fs::remove_dir_all(entry.path()).ok();
                }
            }
        }

        self.cached_binary_path = Some(binary_path.clone());
        Ok(binary_path)
    }
}

/// Path of an earlier download, if any.
/// Similar to `find_existing_binary` of the Elixir extension:
/// https://github.com/zed-extensions/elixir/blob/main/src/language_servers/util.rs
fn find_existing_binary(archive_binary_path: &str) -> Option<String> {
    fs::read_dir(".")
        .ok()?
        .flatten()
        .map(|entry| entry.file_name().to_string_lossy().into_owned())
        .filter(|name| name.starts_with(&format!("{BINARY_NAME}-")))
        .map(|dir| format!("{dir}/{archive_binary_path}"))
        .find(|path| Path::new(path).is_file())
}

impl zed::Extension for PurescriptIrisExtension {
    fn new() -> Self {
        Self::default()
    }

    // Not called when `lsp.iris.binary.path` is set.
    fn language_server_command(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let binary_settings = LspSettings::for_worktree(language_server_id.as_ref(), worktree)
            .ok()
            .and_then(|settings| settings.binary);
        let (binary_args, binary_env) =
            binary_settings.map_or((None, None), |binary| (binary.arguments, binary.env));

        let command = self.language_server_binary_path(language_server_id, worktree)?;

        // `lsp.iris.binary.env` overrides the shell environment.
        let mut env = worktree.shell_env();
        if let Some(binary_env) = binary_env {
            env.retain(|(key, _)| !binary_env.contains_key(key));
            env.extend(binary_env);
        }

        Ok(zed::Command {
            command,
            // See https://iris-lang.com/docs/language-server/#other-editors
            args: binary_args.unwrap_or_else(|| vec!["lsp".to_string(), "--stdio".to_string()]),
            env,
        })
    }

    // `lsp.iris.settings` as the `iris.server` section,
    // see https://iris-lang.com/docs/language-server/#settings
    fn language_server_workspace_configuration(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Option<serde_json::Value>> {
        let settings = LspSettings::for_worktree(language_server_id.as_ref(), worktree)
            .ok()
            .and_then(|settings| settings.settings)
            .unwrap_or_else(|| serde_json::json!({}));

        Ok(Some(serde_json::json!({ SETTINGS_SECTION: settings })))
    }
}

zed::register_extension!(PurescriptIrisExtension);
