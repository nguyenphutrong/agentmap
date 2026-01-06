use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "agentmap")]
#[command(version = "0.1.0")]
#[command(
    about = "Prepare codebases for AI agents by generating outlines, memory files, and reading rules"
)]
#[command(author = "AgentMap Contributors")]
pub struct Args {
    #[arg(default_value = ".")]
    pub path: PathBuf,

    #[arg(short, long, default_value = ".agentmap")]
    pub output: PathBuf,

    #[arg(short, long, default_value = "500")]
    pub threshold: usize,

    #[arg(short, long, action = clap::ArgAction::Append)]
    pub ignore: Vec<String>,

    #[arg(short, long, action = clap::ArgAction::Append)]
    pub lang: Vec<String>,

    #[arg(long, default_value = "false")]
    pub no_gitignore: bool,

    #[arg(long, default_value = "false")]
    pub dry_run: bool,

    #[arg(short, long, action = clap::ArgAction::Count)]
    pub verbose: u8,

    #[arg(short, long, default_value = "false")]
    pub quiet: bool,

    #[arg(long, value_name = "REF")]
    pub diff: Option<String>,

    #[arg(long, default_value = "false")]
    pub json: bool,
}

impl Args {
    pub fn verbosity(&self) -> u8 {
        if self.quiet {
            0
        } else {
            self.verbose + 1
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        if !self.path.exists() {
            return Err(format!("Path does not exist: {}", self.path.display()));
        }

        if !self.path.is_dir() {
            return Err(format!("Path is not a directory: {}", self.path.display()));
        }

        if self.threshold == 0 {
            return Err("Threshold must be greater than 0".to_string());
        }

        Ok(())
    }
}
