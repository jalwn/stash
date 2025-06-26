use std::env;
use std::fs::File;
use std::io::{self, BufReader, BufWriter};
use flate2::Compression;
use flate2::write::ZlibEncoder;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let file_path = &args[1];

    println!("file to compress: {file_path}");

    let source_file = File::open(file_path).expect("unable to open src file.");
    let mut reader = BufReader::new(source_file);

    let compressed_file_path: String = format!("{file_path}.zlib");
    let dest_file = File::create(compressed_file_path).expect("unable to create compressed file.");
    let writer = BufWriter::new(dest_file);

    let mut encoder = ZlibEncoder::new(writer, Compression::default());
    io::copy(&mut reader, &mut encoder).expect("compress failed at io-copy");
    let _writer = encoder.finish().expect("compress failed at flush");
    println!("File compressed successfully!");

}