use clap::{Parser, Subcommand};
use sigmaos_blender::BlendMetadata;
use std::process;

#[derive(Parser)]
#[command(name = "sigmaos-blender", about = "Blender project/file manager")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Read metadata from a .blend file
    Metadata {
        /// Path to the .blend file
        path: String,
    },
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Metadata { path } => {
            match BlendMetadata::from_file(&path) {
                Ok(meta) => {
                    println!("Blender File: {}", path);
                    println!("Pointer size: {}", meta.pointer_size);
                    println!("Endian: {}", meta.endian);
                    println!("Version: {}.{}.{}", meta.version.0, meta.version.1, meta.version.2);
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                    process::exit(1);
                }
            }
        }
    }
}
