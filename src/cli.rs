use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Path for the certificates directory.
    ///
    /// This directory should contain `cert.pem` and `key.pem`.
    #[arg(short, long)]
    pub cert_path: PathBuf,
}
