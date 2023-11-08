use clap::{Parser, Subcommand};

mod daemon;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Starts energy measurement
    Start,
    /// Stops energy measurement
    Stop,
}

fn main() {
    let args = Args::parse();

    match args.command {
        Commands::Start => {
            if daemon::is_started() {
                println!("Already started measurement, please stop the previous measurement to start a new one!");
                std::process::exit(-1)
            }
            println!("Starting measurement");
            daemon::start_daemon();
        }
        Commands::Stop => {
            if !daemon::is_started() {
                println!("Measurement has not yet been started, please start a measurement before stopping it");
                std::process::exit(-1);
            }
            daemon::stop_daemon();
            println!("Stopped measurement");
        }
    }
}
