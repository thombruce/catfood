use catfood_bar::{is_bar_running, run_bar, spawn_in_panel};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "catfood")]
#[command(about = "A utility suite for system management")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Run the system bar
    Bar {
        /// Run without spawning in a kitten panel
        #[arg(long = "no-kitten")]
        no_kitten: bool,
    },
    /// Run the menu system
    Menu {
        #[arg(short, long, help = "Show menu categories")]
        categories: bool,
    },
    /// Run notification system
    Notifications {
        #[arg(short, long, help = "Enable do-not-disturb mode")]
        dnd: bool,
    },
}

fn main() -> color_eyre::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Bar { no_kitten } => {
            if no_kitten {
                // Run directly with existing behavior
                run_bar()?;
            } else {
                // Check if bar is already running
                if is_bar_running()? {
                    eprintln!("catfood bar is already running");
                    std::process::exit(1);
                }

                // Spawn in kitten panel and disown
                spawn_in_panel();
            }
        }
        Commands::Menu { categories: _ } => {
            println!("Menu feature coming soon!");
            std::process::exit(0);
        }
        Commands::Notifications { dnd: _ } => {
            println!("Notifications feature coming soon!");
            std::process::exit(0);
        }
    }

    Ok(())
}
