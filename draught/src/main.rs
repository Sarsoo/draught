use std::fs::File;
use clap::Parser;
use log::info;
use simplelog::*;

use draughtlib::{Game, Team};

/// Command-line arguments for configuring the server
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Width of the board
    #[arg(long, default_value_t = 8)]
    width: usize,

    /// Height of the board
    #[arg(long, default_value_t = 8)]
    height: usize,

    /// Height of the board
    #[arg(short, long, default_value_t = 3)]
    piece_rows: usize,

    #[arg(short, long, default_value_t = 4)]
    search: usize,
}

fn main() {
    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Info, Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
            WriteLogger::new(LevelFilter::Info, Config::default(), File::create("draught.log").unwrap()),
        ]
    ).unwrap();

    let args = Args::parse();

    info!("======================");
    info!("       draught");
    info!("======================");

    let game = Game::new(args.width, args.height, args.piece_rows, Team::White, args.search);

    println!("{}", game.current_board())
}
