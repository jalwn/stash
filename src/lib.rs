use suppaftp::{FtpStream, FtpError};
use std::fs::File;
use std::io::{self, BufReader, BufWriter, Write};
use flate2::Compression;
use flate2::write::ZlibEncoder;
use flate2::read::ZlibDecoder;
use std::env;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum StashError {
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),
    #[error("FTP error: {0}")]
    Ftp(#[from] FtpError),
    #[error("Output path does not exist")]
    OutputPathDoesNotExist,
    #[error("Output path is not a directory")]
    OutputPathNotDirectory,
    #[error("Input path does not contain a file name")]
    NoFileName,
}

pub fn welcome_message() -> String {
    "Welcome to Stash's Awesome Project Library!".to_string()
}

pub fn ftp_connect(addr: &str, user: &str, pass: &str, cwd: &str) -> Result<FtpStream, FtpError> {
    let mut ftp_stream = FtpStream::connect(addr)?;
    ftp_stream.login(user, pass)?;
    ftp_stream.cwd(cwd)?;
    Ok(ftp_stream)
}

pub fn compress_to_dir(file_path: &str, output_path: Option<&str>) ->  Result<(), StashError> {

    let source_file = File::open(file_path)?;
    let mut reader = BufReader::new(source_file);
   
    // Compute file_name.zlib once
    let file_name = PathBuf::from(file_path)
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .ok_or(StashError::NoFileName)?;

    let compressed_file_name = format!("{file_name}.zlib");

    let compressed_file_path = match output_path {
        Some(path) => {
            let out_path = PathBuf::from(path);
            if !out_path.exists() {
                return Err(StashError::OutputPathDoesNotExist);
            }
            if !out_path.is_dir() {
            return Err(StashError::OutputPathNotDirectory);
            }
            out_path.join(&compressed_file_name)
        }
        None => {
            let cwd = env::current_dir()?;
            cwd.join(&compressed_file_name)
        }
    };

    let dest_file = File::create(compressed_file_path)?;
    let writer = BufWriter::new(dest_file); 

    let mut encoder = ZlibEncoder::new(writer, Compression::default());
    io::copy(&mut reader, &mut encoder)?;
    let _writer = encoder.finish()?;

    Ok(())
}


pub fn compress_to_ftp(file_path: &str, ftp_stream: &mut FtpStream) -> Result<(), StashError> {
    let source_file = File::open(&file_path)?;
    let mut reader = BufReader::new(source_file);

    let file_name = PathBuf::from(file_path)
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .ok_or(StashError::NoFileName)?;

    let compressed_file_name: String = format!("{file_name}.zlib");
    //let dest_file = File::create(compressed_file_path).expect("unable to create compressed file.");
    let ftp_writer = ftp_stream.put_with_stream(compressed_file_name)?;

    let mut encoder = ZlibEncoder::new(ftp_writer, Compression::default());
    
    io::copy(&mut reader, &mut encoder)?;
    
    let ftp_writer = encoder.finish()?;
    ftp_stream.finalize_put_stream(ftp_writer)?;
    
    Ok(())

}

pub fn decompress_file(input_path: &str, output_path: Option<&str>) -> Result<(), StashError> {
    let compressed_file = File::open(input_path)?;
    let mut decoder = ZlibDecoder::new(BufReader::new(compressed_file));

    // FIX: Create a binding for the PathBuf
    let input_path_buf = PathBuf::from(input_path);
    let file_name = input_path_buf
        .file_name()
        .and_then(|n| n.to_str())
        .ok_or(StashError::NoFileName)?;

    let decompressed_file_name = file_name
        .strip_suffix(".zlib")
        .ok_or(StashError::NoFileName)?; // &str

    // Determine output directory
    let output_dir = match output_path {
        Some(dir) => {
            let dir_path = PathBuf::from(dir);
            if !dir_path.exists() {
                return Err(StashError::OutputPathDoesNotExist);
            }
            if !dir_path.is_dir() {
                return Err(StashError::OutputPathNotDirectory);
            }
            dir_path
        }
        None => env::current_dir()?,
    };

    let output_file_path = output_dir.join(decompressed_file_name);

    let output_file = File::create(output_file_path)?;
    let mut writer = BufWriter::new(output_file);

    io::copy(&mut decoder, &mut writer)?;
    writer.flush()?;
    Ok(())
}