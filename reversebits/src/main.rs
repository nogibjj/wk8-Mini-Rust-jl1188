use clap::Parser;

#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "Selina Liu",
    about = "Reversing the binary bits of a integer"
)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    #[clap(version = "1.0", author = "Selina Liu")]
    Reverse {
        #[clap(short, long)]
        input: u32,
    },
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::Reverse { input }) => {
            let result = reversebits::reverse_bits(input);
            println!(
                "Reversing the bits in the binary representation of {}  is: {}",
                input, result
            );
        }
        None => println!("No subcommand was used"),
    }
}
