use cli::mago::command::{build_mago_command, MagoCommand};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

const MAGO_BIN: &str = "mago";

#[test]
fn build_mago_command_omits_workspace_when_none_is_provided() {
    let file_path = Path::new("/tmp/example.php");
    let command = build_mago_command(MAGO_BIN, file_path, MagoCommand::Lint, None);

    let args: Vec<_> = command.get_args().map(|arg| arg.to_string_lossy().into_owned()).collect();

    assert_eq!(args, ["lint", "/tmp/example.php"]);
    assert_eq!(command.get_current_dir(), None);
}

#[test]
fn build_mago_command_forwards_workspace_to_mago() {
    let workspace_root = Path::new("/tmp/workspace");
    let file_path = Path::new("/tmp/workspace/src/example.php");
    let command =
        build_mago_command(MAGO_BIN, file_path, MagoCommand::Analyze, Some(workspace_root));

    let args: Vec<_> = command.get_args().map(|arg| arg.to_string_lossy().into_owned()).collect();

    assert_eq!(
        args,
        ["--workspace", "/tmp/workspace", "analyze", "/tmp/workspace/src/example.php",]
    );
    assert_eq!(command.get_current_dir(), Some(workspace_root));
}

fn create_temp_workspace(test_name: &str) -> PathBuf {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time should be after unix epoch")
        .as_nanos();
    let workspace = std::env::temp_dir()
        .join(format!("mago-zed-{test_name}-{}-{timestamp}", std::process::id()));

    fs::create_dir_all(&workspace).expect("should create temp workspace");
    workspace
}

#[test]
fn build_mago_command_attaches_config_when_workspace_contains_mago_toml() {
    let workspace_root = create_temp_workspace("config-present");
    fs::write(workspace_root.join("mago.toml"), "php-version = \"8.3\"\n")
        .expect("should write config file");

    let file_path = workspace_root.join("src/example.php");
    let command =
        build_mago_command(MAGO_BIN, &file_path, MagoCommand::Lint, Some(&workspace_root));

    let args: Vec<_> = command.get_args().map(|arg| arg.to_string_lossy().into_owned()).collect();

    assert_eq!(
        args,
        [
            "--workspace",
            workspace_root.to_string_lossy().as_ref(),
            "--config",
            workspace_root.join("mago.toml").to_string_lossy().as_ref(),
            "lint",
            file_path.to_string_lossy().as_ref(),
        ]
    );

    fs::remove_dir_all(&workspace_root).expect("should clean up temp workspace");
}

#[test]
fn build_mago_command_falls_back_to_mago_dist_with_format_precedence() {
    let workspace_root = create_temp_workspace("dist-fallback");
    fs::write(workspace_root.join("mago.dist.json"), "{}\n").expect("should write dist json");
    fs::write(workspace_root.join("mago.dist.yaml"), "php-version: 8.2\n")
        .expect("should write dist yaml");

    let file_path = workspace_root.join("src/example.php");
    let command =
        build_mago_command(MAGO_BIN, &file_path, MagoCommand::Analyze, Some(&workspace_root));

    let args: Vec<_> = command.get_args().map(|arg| arg.to_string_lossy().into_owned()).collect();

    assert_eq!(args[2], "--config");
    assert_eq!(args[3], workspace_root.join("mago.dist.yaml").to_string_lossy().as_ref());

    fs::remove_dir_all(&workspace_root).expect("should clean up temp workspace");
}
