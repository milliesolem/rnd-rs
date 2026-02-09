use clap::Parser;
use base64::{engine::general_purpose::STANDARD_NO_PAD, Engine as _};
use std::fs::File;
use std::io::{self, Read, Write};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Encoding format: base64, hex, or raw
    #[arg(default_value = "hex")]
    encoding: String,

    /// Number of bytes to read
    #[arg(short, long)]
    length: usize,

    /// Source file path
    #[arg(short, long, default_value = "/dev/urandom")]
    source: String,
}

fn main() {
    let args = Args::parse();
    let mut file = File::open(&args.source).expect("Failed to open source file");
    let mut buffer = vec![0u8; args.length];
    file.read_exact(&mut buffer).expect("Failed to read the requested number of bytes");

    match args.encoding.as_str() {
        "base64" | "b64" => {
            let encoded = STANDARD_NO_PAD.encode(&buffer);
            println!("{}", encoded);
        }
        "hex" => {
            for byte in &buffer {
                print!("{:02x}", byte);
            }
            println!();
        }
        "raw" => {
            io::stdout().write_all(&buffer).expect("Failed to write to stdout");
        }
        _ => {
            eprintln!("Error: Unknown encoding '{}'. Supported: base64, hex, raw", args.encoding);
            std::process::exit(1);
        }
    };
}