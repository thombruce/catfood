use catfood_bar::{is_bar_running, run_bar, spawn_in_panel};
use clap::Parser;

#[derive(Parser)]
#[command(name = "catfood-bar")]
#[command(about = "A system bar component of the catfood utility suite")]
struct Cli {
    /// Run without spawning in a kitten panel
    #[arg(long = "no-kitten")]
    no_kitten: bool,
}

fn main() -> color_eyre::Result<()> {
    let cli = Cli::parse();

    // If not --no-kitten, check if already running and spawn in panel
    if !cli.no_kitten {
        if is_bar_running()? {
            eprintln!("catfood-bar is already running");
            std::process::exit(1);
        }
        spawn_in_panel();
        Ok(()) // This line will never be reached, but needed for type compatibility
    } else {
        // --no-kitten: run directly with existing behavior
        run_bar()
    }
}
