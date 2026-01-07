use anyhow::Result;
use indicatif::{ProgressBar, ProgressStyle};
use inquire::{Confirm, MultiSelect};
use std::io::IsTerminal;
use std::path::Path;
use std::time::Duration;

use super::theme::{agentlens_theme, print_banner, print_error, print_success, print_summary};
use crate::cli::{install_hooks_with_manager, run_templates};
use crate::scan::scan_directory;
use crate::Config;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TemplateChoice {
    Cursor,
    Claude,
    OpenCode,
}

impl TemplateChoice {
    fn as_template_arg(&self) -> String {
        match self {
            TemplateChoice::Cursor => "cursor".to_string(),
            TemplateChoice::Claude => "claude".to_string(),
            TemplateChoice::OpenCode => "opencode".to_string(),
        }
    }
}

#[derive(Debug, Default)]
pub struct InitOptions {
    pub create_config: bool,
    pub install_hooks: bool,
    pub templates: Vec<TemplateChoice>,
}

impl InitOptions {
    pub fn is_empty(&self) -> bool {
        !self.create_config && !self.install_hooks && self.templates.is_empty()
    }
}

pub fn is_interactive() -> bool {
    std::io::stdin().is_terminal() && std::io::stdout().is_terminal()
}

pub fn run_interactive_init(path: &Path) -> Result<InitOptions> {
    inquire::set_global_render_config(agentlens_theme());

    print_banner();

    let project_info = detect_project(path);
    println!("  {} {}", console::style("📁").dim(), path.display());
    if !project_info.languages.is_empty() {
        println!(
            "  {} Languages: {}",
            console::style("📦").dim(),
            project_info.languages.join(", ")
        );
    }
    println!(
        "  {} {} source files",
        console::style("📄").dim(),
        project_info.file_count
    );
    println!();

    let proceed = Confirm::new("Continue with this directory?")
        .with_default(true)
        .with_help_message("Press Enter to confirm, or n to cancel")
        .prompt()?;

    if !proceed {
        println!();
        println!("  {} Setup cancelled.", console::style("ℹ").blue());
        println!();
        return Ok(InitOptions::default());
    }

    println!();

    let options = vec![
        "Generate agentlens.toml config",
        "Install git hooks (auto-regenerate on commit)",
        "Generate .cursorrules (Cursor)",
        "Generate CLAUDE.md (Claude Code)",
        "Generate AGENTS.md (OpenCode)",
    ];

    let defaults = vec![0, 1];

    let selected = MultiSelect::new("What would you like to set up?", options)
        .with_default(&defaults)
        .with_help_message("↑↓ move, Space select, Enter confirm")
        .prompt()?;

    let mut init_options = InitOptions::default();

    for selection in selected {
        match selection {
            "Generate agentlens.toml config" => init_options.create_config = true,
            "Install git hooks (auto-regenerate on commit)" => init_options.install_hooks = true,
            "Generate .cursorrules (Cursor)" => init_options.templates.push(TemplateChoice::Cursor),
            "Generate CLAUDE.md (Claude Code)" => {
                init_options.templates.push(TemplateChoice::Claude)
            }
            "Generate AGENTS.md (OpenCode)" => {
                init_options.templates.push(TemplateChoice::OpenCode)
            }
            _ => {}
        }
    }

    Ok(init_options)
}

pub fn execute_setup(options: &InitOptions, path: &Path, output_dir: &str) -> Result<()> {
    if options.is_empty() {
        println!();
        println!(
            "  {} Nothing selected. Run {} for options.",
            console::style("ℹ").blue(),
            console::style("agentlens init").cyan()
        );
        println!();
        return Ok(());
    }

    println!();
    println!("  {}", console::style("Setting up agentlens...").bold());
    println!();

    if options.create_config {
        let spinner = create_spinner("Creating agentlens.toml...");
        match Config::create_default_file(path) {
            Ok(config_path) => {
                spinner.finish_and_clear();
                print_success(&format!("Created {}", config_path.display()));
            }
            Err(e) => {
                spinner.finish_and_clear();
                print_error(&format!("Failed to create config: {}", e));
            }
        }
    }

    if options.install_hooks {
        let spinner = create_spinner("Installing git hooks...");
        match install_hooks_with_manager(path, false, false, false, false) {
            Ok(()) => {
                spinner.finish_and_clear();
                print_success("Installed git hooks");
            }
            Err(e) => {
                spinner.finish_and_clear();
                print_error(&format!("Failed to install hooks: {}", e));
            }
        }
    }

    for template in &options.templates {
        let template_name = match template {
            TemplateChoice::Cursor => ".cursorrules",
            TemplateChoice::Claude => "CLAUDE.md",
            TemplateChoice::OpenCode => "AGENTS.md",
        };
        let spinner = create_spinner(&format!("Generating {}...", template_name));
        match run_templates(path, Some(template.as_template_arg()), output_dir) {
            Ok(()) => {
                spinner.finish_and_clear();
                print_success(&format!("Generated {}", template_name));
            }
            Err(e) => {
                spinner.finish_and_clear();
                print_error(&format!("Failed to generate {}: {}", template_name, e));
            }
        }
    }

    print_summary();
    Ok(())
}

fn create_spinner(message: &str) -> ProgressBar {
    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏")
            .template("  {spinner} {msg}")
            .unwrap(),
    );
    spinner.set_message(message.to_string());
    spinner.enable_steady_tick(Duration::from_millis(80));
    spinner
}

struct ProjectInfo {
    file_count: usize,
    languages: Vec<String>,
}

fn detect_project(path: &Path) -> ProjectInfo {
    let files = scan_directory(path, 500, true, Some(5)).unwrap_or_default();

    let mut lang_counts: std::collections::HashMap<String, usize> =
        std::collections::HashMap::new();
    for file in &files {
        let lang = format!("{:?}", file.language);
        if lang != "Unknown" {
            *lang_counts.entry(lang).or_insert(0) += 1;
        }
    }

    let mut languages: Vec<_> = lang_counts.into_iter().collect();
    languages.sort_by(|a, b| b.1.cmp(&a.1));
    let languages: Vec<String> = languages.into_iter().take(3).map(|(l, _)| l).collect();

    ProjectInfo {
        file_count: files.len(),
        languages,
    }
}
