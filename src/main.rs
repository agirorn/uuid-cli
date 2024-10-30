use uuid::Uuid;

use clap::Parser;

/// A UUID generator
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Number of UUIDs to generate
    #[arg(short, long, default_value_t = 1)]
    count: u64,
}

fn main() {
    let args = Args::parse();

    for _ in 0..args.count {
        println!("{}", Uuid::new_v4());
    }
}
