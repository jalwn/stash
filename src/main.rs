use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Compress and upload a file to FTP
    Upload {
        #[arg(short = 'f', long)]
        file_path: String,
        #[arg(short = 'a', long)]
        ftp_addr: String,
        #[arg(short = 'p', long)]
        ftp_pass: String,
        #[arg(short = 'u', long)]
        ftp_user: String,
        #[arg(short = 'd', long)]
        ftp_cwd: String,
    },
    /// Just compress a file locally
    Compress {
        #[arg(short, long)]
        file_path: String,
        #[arg(short, long)]
        output: Option<String>,
    },
    /// Decompress a .zlib file to a directory
    Decompress {
        #[arg(short, long)]
        input: String,
        #[arg(short, long)]
        output: Option<String>,
    },
}

fn main() {
    let args = Args::parse();


    match args.command {
        Commands::Upload { file_path, ftp_addr, ftp_pass, ftp_user, ftp_cwd } => {
            // Call your upload logic here
            println!("uploading {file_path} to {ftp_addr} as {ftp_user}");
            let mut ftp_stream = stash::ftp_connect(&ftp_addr, &ftp_user, &ftp_pass, &ftp_cwd).unwrap();
            stash::compress_to_ftp(&file_path, &mut ftp_stream).unwrap();
            println!("file compressed successfully!");
            let _ = ftp_stream.quit();
        }
        Commands::Compress { file_path, output } => {
            // Call your compress logic here
            println!("compressing {file_path} to {:?}", output);
            stash::compress_to_dir(&file_path, output.as_deref()).unwrap();
            println!("file compressed successfully!");
        }
        Commands::Decompress { input, output } => {
            println!("decompressing {input} to {:?}", output);
            stash::decompress_file(&input, output.as_deref()).unwrap();
            println!("file decompressed successfully!");
        }
    }
}