use std::io;
use std::path::Path;

/// Generate a Zellij KDL layout for a worktree
///
/// Layout structure:
/// - Left pane (60%): Text editor opening the worktree root
/// - Bottom-left (stacked): Terminal for running commands
/// - Right pane (40%): Claude Code instance
pub fn generate_layout(
    worktree_path: &Path,
    editor_cmd: &str,
    install_cmd: Option<&str>,
    start_cmd: Option<&str>,
) -> String {
    let path_str = worktree_path.to_string_lossy();

    // Build the command sequence for the terminal pane
    let terminal_cmd = build_terminal_command(install_cmd, start_cmd);

    format!(
        r#"layout {{
    default_tab_template {{
        pane size=1 borderless=true {{
            plugin location="zellij:tab-bar"
        }}
        children
        pane size=2 borderless=true {{
            plugin location="zellij:status-bar"
        }}
    }}

    tab name="{tab_name}" cwd="{cwd}" {{
        pane split_direction="vertical" {{
            pane split_direction="horizontal" size="60%" {{
                pane {{
                    command "{editor}"
                    args "{cwd}"
                }}
                pane size="30%" {{
                    {terminal_pane}
                }}
            }}
            pane size="40%" {{
                command "claude"
            }}
        }}
    }}
}}
"#,
        tab_name = worktree_path
            .file_name()
            .map(|n| n.to_string_lossy())
            .unwrap_or_else(|| "worktree".into()),
        cwd = path_str,
        editor = editor_cmd,
        terminal_pane = terminal_cmd,
    )
}

fn build_terminal_command(install_cmd: Option<&str>, start_cmd: Option<&str>) -> String {
    match (install_cmd, start_cmd) {
        (Some(install), Some(start)) => {
            // Run install first, then start
            format!(
                r#"command "bash"
                    args "-c" "{} && {}"
                    start_suspended false"#,
                escape_shell(install),
                escape_shell(start)
            )
        }
        (Some(install), None) => {
            format!(
                r#"command "bash"
                    args "-c" "{}"
                    start_suspended false"#,
                escape_shell(install)
            )
        }
        (None, Some(start)) => {
            format!(
                r#"command "bash"
                    args "-c" "{}"
                    start_suspended false"#,
                escape_shell(start)
            )
        }
        (None, None) => {
            // Just a regular shell
            String::new()
        }
    }
}

fn escape_shell(cmd: &str) -> String {
    cmd.replace('\\', "\\\\").replace('"', "\\\"")
}

/// Write layout to a temporary file and return the path
pub fn write_temp_layout(layout: &str) -> io::Result<std::path::PathBuf> {
    let tmp_dir = std::env::temp_dir().join("maestro-ai");
    std::fs::create_dir_all(&tmp_dir)?;

    let layout_path = tmp_dir.join("layout.kdl");
    std::fs::write(&layout_path, layout)?;
    Ok(layout_path)
}

/// Get the user's preferred editor command
pub fn get_editor_command() -> String {
    std::env::var("VISUAL")
        .or_else(|_| std::env::var("EDITOR"))
        .unwrap_or_else(|_| "hx".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_generate_layout() {
        let path = PathBuf::from("/home/user/project.feature");
        let layout = generate_layout(&path, "hx", Some("bun install"), Some("bun run serve"));

        assert!(layout.contains("tab name=\"project.feature\""));
        assert!(layout.contains("command \"hx\""));
        assert!(layout.contains("command \"claude\""));
        assert!(layout.contains("bun install"));
        assert!(layout.contains("bun run serve"));
    }

    #[test]
    fn test_generate_layout_no_hooks() {
        let path = PathBuf::from("/home/user/project.feature");
        let layout = generate_layout(&path, "code", None, None);

        assert!(layout.contains("tab name=\"project.feature\""));
        assert!(layout.contains("command \"code\""));
        assert!(layout.contains("command \"claude\""));
    }
}
