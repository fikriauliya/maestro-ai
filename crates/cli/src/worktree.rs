use crate::WtCommands;
use std::io::{self, Error, ErrorKind};
use std::path::{Path, PathBuf};
use std::process::Command;

pub struct Worktree {
    pub path: PathBuf,
    pub branch: Option<String>,
    pub is_bare: bool,
}

pub fn run(cmd: WtCommands) -> io::Result<()> {
    match cmd {
        WtCommands::List => cmd_list(),
        WtCommands::Switch { branch } => cmd_switch(&branch),
        WtCommands::Remove => cmd_remove(),
        WtCommands::Merge => cmd_merge(),
    }
}

fn cmd_list() -> io::Result<()> {
    let worktrees = list_worktrees()?;
    let current_dir = std::env::current_dir()?;

    if worktrees.is_empty() {
        println!("No worktrees found");
        return Ok(());
    }

    for wt in worktrees {
        let is_current = wt.path == current_dir;
        let marker = if is_current { "*" } else { " " };
        let dirty = if !wt.is_bare && is_dirty(&wt.path) {
            " [dirty]"
        } else {
            ""
        };
        let branch = wt.branch.as_deref().unwrap_or("(bare)");
        println!("{} {} {}{}", marker, branch, wt.path.display(), dirty);
    }

    Ok(())
}

fn cmd_switch(branch: &str) -> io::Result<()> {
    let worktrees = list_worktrees()?;

    // Check if worktree already exists for this branch
    if let Some(wt) = worktrees.iter().find(|w| w.branch.as_deref() == Some(branch)) {
        exec_shell(&wt.path)?;
        return Ok(());
    }

    // Create new worktree
    let repo_root = get_repo_root()?;
    let repo_name = repo_root
        .file_name()
        .and_then(|n| n.to_str())
        .ok_or_else(|| Error::new(ErrorKind::Other, "Cannot determine repo name"))?;

    let parent_dir = repo_root
        .parent()
        .ok_or_else(|| Error::new(ErrorKind::Other, "Cannot determine parent directory"))?;

    let worktree_path = parent_dir.join(format!("{}.{}", repo_name, branch));
    let default_branch = get_default_branch()?;

    // Create worktree with new branch
    let output = Command::new("git")
        .args(["worktree", "add", "-b", branch])
        .arg(&worktree_path)
        .arg(&default_branch)
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(Error::new(ErrorKind::Other, stderr.to_string()));
    }

    println!("Created worktree at {}", worktree_path.display());
    exec_shell(&worktree_path)?;
    Ok(())
}

fn cmd_remove() -> io::Result<()> {
    let current_dir = std::env::current_dir()?;
    let worktrees = list_worktrees()?;

    // Find current worktree
    let current_wt = worktrees
        .iter()
        .find(|w| w.path == current_dir)
        .ok_or_else(|| Error::new(ErrorKind::Other, "Not in a worktree"))?;

    let branch = current_wt
        .branch
        .as_ref()
        .ok_or_else(|| Error::new(ErrorKind::Other, "Cannot determine current branch"))?;

    // Find main worktree (first non-bare one, or the bare one)
    let main_wt = worktrees
        .iter()
        .find(|w| !w.is_bare && w.path != current_dir)
        .or_else(|| worktrees.iter().find(|w| w.is_bare))
        .ok_or_else(|| Error::new(ErrorKind::Other, "Cannot find main worktree"))?;

    // Check if dirty
    if is_dirty(&current_dir) {
        return Err(Error::new(
            ErrorKind::Other,
            "Worktree has uncommitted changes. Commit or stash first.",
        ));
    }

    let main_path = main_wt.path.clone();
    let branch_to_delete = branch.clone();
    let path_to_remove = current_dir.clone();

    // Change to main worktree first
    std::env::set_current_dir(&main_path)?;

    // Remove worktree
    let output = Command::new("git")
        .args(["worktree", "remove"])
        .arg(&path_to_remove)
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(Error::new(ErrorKind::Other, stderr.to_string()));
    }

    // Delete branch
    let output = Command::new("git")
        .args(["branch", "-d", &branch_to_delete])
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("Warning: Could not delete branch: {}", stderr.trim());
    }

    println!("Removed worktree and branch '{}'", branch_to_delete);
    exec_shell(&main_path)?;
    Ok(())
}

fn cmd_merge() -> io::Result<()> {
    let current_dir = std::env::current_dir()?;
    let worktrees = list_worktrees()?;

    // Find current worktree
    let current_wt = worktrees
        .iter()
        .find(|w| w.path == current_dir)
        .ok_or_else(|| Error::new(ErrorKind::Other, "Not in a worktree"))?;

    let branch = current_wt
        .branch
        .as_ref()
        .ok_or_else(|| Error::new(ErrorKind::Other, "Cannot determine current branch"))?
        .clone();

    let default_branch = get_default_branch()?;

    if branch == default_branch {
        return Err(Error::new(
            ErrorKind::Other,
            "Cannot merge main branch into itself",
        ));
    }

    // Check if dirty
    if is_dirty(&current_dir) {
        return Err(Error::new(
            ErrorKind::Other,
            "Worktree has uncommitted changes. Commit or stash first.",
        ));
    }

    // Find main worktree
    let main_wt = worktrees
        .iter()
        .find(|w| w.branch.as_deref() == Some(&default_branch))
        .ok_or_else(|| {
            Error::new(
                ErrorKind::Other,
                format!("Cannot find worktree for branch '{}'", default_branch),
            )
        })?;

    let main_path = main_wt.path.clone();
    let path_to_remove = current_dir.clone();

    // Change to main worktree
    std::env::set_current_dir(&main_path)?;

    // Squash merge
    let output = Command::new("git")
        .args(["merge", "--squash", &branch])
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(Error::new(
            ErrorKind::Other,
            format!("Merge failed: {}", stderr),
        ));
    }

    // Commit the squashed changes
    let output = Command::new("git")
        .args(["commit", "-m", &format!("Merge branch '{}'", branch)])
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        // Check if it's just "nothing to commit"
        if !stderr.contains("nothing to commit") {
            return Err(Error::new(
                ErrorKind::Other,
                format!("Commit failed: {}", stderr),
            ));
        }
    }

    // Remove worktree
    let output = Command::new("git")
        .args(["worktree", "remove"])
        .arg(&path_to_remove)
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("Warning: Could not remove worktree: {}", stderr.trim());
    }

    // Delete branch
    let output = Command::new("git")
        .args(["branch", "-d", &branch])
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("Warning: Could not delete branch: {}", stderr.trim());
    }

    println!("Merged '{}' into '{}' and cleaned up", branch, default_branch);
    exec_shell(&main_path)?;
    Ok(())
}

fn list_worktrees() -> io::Result<Vec<Worktree>> {
    let output = Command::new("git")
        .args(["worktree", "list", "--porcelain"])
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(Error::new(ErrorKind::Other, stderr.to_string()));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut worktrees = Vec::new();
    let mut current_path: Option<PathBuf> = None;
    let mut current_branch: Option<String> = None;
    let mut is_bare = false;

    for line in stdout.lines() {
        if line.starts_with("worktree ") {
            // Save previous worktree if exists
            if let Some(path) = current_path.take() {
                worktrees.push(Worktree {
                    path,
                    branch: current_branch.take(),
                    is_bare,
                });
                is_bare = false;
            }
            current_path = Some(PathBuf::from(&line[9..]));
        } else if line.starts_with("branch refs/heads/") {
            current_branch = Some(line[18..].to_string());
        } else if line == "bare" {
            is_bare = true;
        }
    }

    // Don't forget the last one
    if let Some(path) = current_path {
        worktrees.push(Worktree {
            path,
            branch: current_branch,
            is_bare,
        });
    }

    Ok(worktrees)
}

fn is_dirty(path: &Path) -> bool {
    Command::new("git")
        .args(["-C", &path.to_string_lossy(), "status", "--porcelain"])
        .output()
        .map(|o| !o.stdout.is_empty())
        .unwrap_or(false)
}

fn get_repo_root() -> io::Result<PathBuf> {
    let output = Command::new("git")
        .args(["rev-parse", "--show-toplevel"])
        .output()?;

    if !output.status.success() {
        return Err(Error::new(ErrorKind::Other, "Not in a git repository"));
    }

    let path = String::from_utf8_lossy(&output.stdout).trim().to_string();
    Ok(PathBuf::from(path))
}

fn get_default_branch() -> io::Result<String> {
    // Try to get from remote
    let output = Command::new("git")
        .args(["symbolic-ref", "refs/remotes/origin/HEAD"])
        .output()?;

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        if let Some(branch) = stdout.trim().strip_prefix("refs/remotes/origin/") {
            return Ok(branch.to_string());
        }
    }

    // Fall back to checking for main or master
    for branch in &["main", "master"] {
        let output = Command::new("git")
            .args(["rev-parse", "--verify", &format!("refs/heads/{}", branch)])
            .output()?;

        if output.status.success() {
            return Ok(branch.to_string());
        }
    }

    Err(Error::new(
        ErrorKind::Other,
        "Cannot determine default branch",
    ))
}

fn exec_shell(path: &Path) -> io::Result<()> {
    use std::os::unix::process::CommandExt;

    let shell = std::env::var("SHELL").unwrap_or_else(|_| "/bin/sh".to_string());

    std::env::set_current_dir(path)?;

    let err = Command::new(&shell).exec();

    Err(Error::new(
        ErrorKind::Other,
        format!("Failed to exec shell: {}", err),
    ))
}
