use anyhow::{Context, Result};
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

const PRE_COMMIT_HOOK: &str = r#"#!/bin/sh
# agentmap pre-commit hook
# Regenerates docs for changed files and stages .agentmap/

if [ -n "$AGENTMAP_SKIP" ]; then
    exit 0
fi

if command -v agentmap >/dev/null 2>&1; then
    agentmap --quiet
    git add .agentmap/ 2>/dev/null || true
fi
"#;

const POST_CHECKOUT_HOOK: &str = r#"#!/bin/sh
# agentmap post-checkout hook
# Regenerates docs after branch switch

if [ -n "$AGENTMAP_SKIP" ]; then
    exit 0
fi

# Only run on branch checkout (not file checkout)
if [ "$3" = "1" ]; then
    if command -v agentmap >/dev/null 2>&1; then
        agentmap --quiet &
    fi
fi
"#;

const POST_MERGE_HOOK: &str = r#"#!/bin/sh
# agentmap post-merge hook
# Regenerates docs after pull/merge

if [ -n "$AGENTMAP_SKIP" ]; then
    exit 0
fi

if command -v agentmap >/dev/null 2>&1; then
    agentmap --quiet &
fi
"#;

pub fn install_hooks(path: &Path) -> Result<()> {
    let git_dir = find_git_dir(path)?;
    let hooks_dir = git_dir.join("hooks");

    fs::create_dir_all(&hooks_dir).context("Failed to create hooks directory")?;

    install_hook(&hooks_dir, "pre-commit", PRE_COMMIT_HOOK)?;
    install_hook(&hooks_dir, "post-checkout", POST_CHECKOUT_HOOK)?;
    install_hook(&hooks_dir, "post-merge", POST_MERGE_HOOK)?;

    eprintln!("Installed agentmap git hooks:");
    eprintln!("  - pre-commit: regenerate docs and stage .agentmap/");
    eprintln!("  - post-checkout: regenerate docs after branch switch");
    eprintln!("  - post-merge: regenerate docs after pull/merge");
    eprintln!("\nTo skip hooks, set AGENTMAP_SKIP=1");

    Ok(())
}

pub fn remove_hooks(path: &Path) -> Result<()> {
    let git_dir = find_git_dir(path)?;
    let hooks_dir = git_dir.join("hooks");

    remove_hook(&hooks_dir, "pre-commit")?;
    remove_hook(&hooks_dir, "post-checkout")?;
    remove_hook(&hooks_dir, "post-merge")?;

    eprintln!("Removed agentmap git hooks");

    Ok(())
}

fn find_git_dir(path: &Path) -> Result<std::path::PathBuf> {
    let mut current = path.to_path_buf();
    loop {
        let git_dir = current.join(".git");
        if git_dir.is_dir() {
            return Ok(git_dir);
        }
        if !current.pop() {
            break;
        }
    }
    anyhow::bail!("Not a git repository (or any parent)")
}

fn install_hook(hooks_dir: &Path, name: &str, content: &str) -> Result<()> {
    let hook_path = hooks_dir.join(name);

    if hook_path.exists() {
        let existing = fs::read_to_string(&hook_path).unwrap_or_default();
        if existing.contains("agentmap") {
            eprintln!("  {} hook already contains agentmap, skipping", name);
            return Ok(());
        }

        let combined = format!(
            "{}\n\n# --- agentmap hook ---\n{}",
            existing.trim(),
            content.trim()
        );
        fs::write(&hook_path, combined).context(format!("Failed to update {} hook", name))?;
        eprintln!("  {} hook updated (appended)", name);
    } else {
        fs::write(&hook_path, content).context(format!("Failed to create {} hook", name))?;
        eprintln!("  {} hook created", name);
    }

    let mut perms = fs::metadata(&hook_path)?.permissions();
    perms.set_mode(0o755);
    fs::set_permissions(&hook_path, perms)?;

    Ok(())
}

fn remove_hook(hooks_dir: &Path, name: &str) -> Result<()> {
    let hook_path = hooks_dir.join(name);

    if !hook_path.exists() {
        return Ok(());
    }

    let content = fs::read_to_string(&hook_path).unwrap_or_default();

    if !content.contains("agentmap") {
        return Ok(());
    }

    if content.contains("# --- agentmap hook ---") {
        let cleaned: Vec<&str> = content
            .split("# --- agentmap hook ---")
            .next()
            .unwrap_or("")
            .trim()
            .lines()
            .collect();

        if cleaned.is_empty() || (cleaned.len() == 1 && cleaned[0] == "#!/bin/sh") {
            fs::remove_file(&hook_path).context(format!("Failed to remove {} hook", name))?;
            eprintln!("  {} hook removed", name);
        } else {
            fs::write(&hook_path, cleaned.join("\n"))
                .context(format!("Failed to update {} hook", name))?;
            eprintln!("  {} hook updated (agentmap section removed)", name);
        }
    } else {
        fs::remove_file(&hook_path).context(format!("Failed to remove {} hook", name))?;
        eprintln!("  {} hook removed", name);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_find_git_dir() {
        let temp = TempDir::new().unwrap();
        let git_dir = temp.path().join(".git");
        fs::create_dir(&git_dir).unwrap();

        let result = find_git_dir(temp.path());
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), git_dir);
    }

    #[test]
    fn test_find_git_dir_not_found() {
        let temp = TempDir::new().unwrap();
        let result = find_git_dir(temp.path());
        assert!(result.is_err());
    }
}
