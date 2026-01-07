use console::style;
use inquire::ui::{Attributes, Color, RenderConfig, StyleSheet, Styled};

pub fn agentlens_theme() -> RenderConfig<'static> {
    let mut config = RenderConfig::default();

    config.prompt_prefix = Styled::new("?").with_fg(Color::LightCyan);
    config.highlighted_option_prefix = Styled::new("❯").with_fg(Color::LightCyan);
    config.selected_checkbox = Styled::new("◉").with_fg(Color::LightGreen);
    config.unselected_checkbox = Styled::new("○").with_fg(Color::DarkGrey);
    config.answer = StyleSheet::new().with_fg(Color::LightCyan);
    config.help_message = StyleSheet::new()
        .with_fg(Color::DarkGrey)
        .with_attr(Attributes::ITALIC);

    config
}

pub fn print_banner() {
    println!();
    println!(
        "  {}  {}",
        style("🔍").cyan(),
        style("agentlens").cyan().bold()
    );
    println!("  {}", style("Interactive Setup").dim());
    println!();
}

pub fn print_success(message: &str) {
    println!("  {} {}", style("✓").green(), message);
}

pub fn print_error(message: &str) {
    println!("  {} {}", style("✗").red(), message);
}

pub fn print_summary() {
    println!();
    println!("{}", style("─".repeat(50)).dim());
    println!();
    println!(
        "  {} {}",
        style("✅").green(),
        style("Setup complete!").green().bold()
    );
    println!();
    println!("  {}", style("Next steps:").bold());
    println!(
        "    {} Run {} to generate documentation",
        style("1.").dim(),
        style("agentlens").cyan()
    );
    println!(
        "    {} Add {} to .gitignore (or commit for team)",
        style("2.").dim(),
        style(".agentlens/").cyan()
    );
    println!(
        "    {} Read {} for AI navigation",
        style("3.").dim(),
        style(".agentlens/INDEX.md").cyan()
    );
    println!();
    println!("  {} 🚀", style("Happy coding!").dim());
    println!();
}
