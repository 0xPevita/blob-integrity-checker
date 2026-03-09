use std::fs;
use std::path::Path;
use sha2::{Sha256, Digest};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "blob-checker", about = "Fast file integrity checker for Shelby blob storage")]
struct Args {
    #[arg(short, long)] path: String,
    #[arg(short, long)] expected: Option<String>,
    #[arg(short, long)] recursive: bool,
}

fn hash_file(filepath: &str) -> Result<String, Box<dyn std::error::Error>> {
    let data = fs::read(filepath)?;
    let mut h = Sha256::new();
    h.update(&data);
    Ok(format!("{:x}", h.finalize()))
}

fn check(filepath: &str, expected: Option<&str>) {
    match hash_file(filepath) {
        Ok(hash) => match expected {
            Some(exp) if hash == exp => println!("✅ MATCH    {} → {}", filepath, &hash[..16]),
            Some(exp) => println!("❌ MISMATCH {} → got {} expected {}", filepath, &hash[..16], &exp[..16]),
            None => println!("🔐 {} → sha256:{}", filepath, hash),
        },
        Err(e) => println!("❌ ERROR {} → {}", filepath, e),
    }
}

fn scan_dir(dir: &str, recursive: bool) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let p = entry.path();
            if p.is_file() { check(p.to_str().unwrap_or(""), None); }
            else if p.is_dir() && recursive { scan_dir(p.to_str().unwrap_or(""), recursive); }
        }
    }
}

fn main() {
    let args = Args::parse();
    println!("🔍 Blob Integrity Checker — Shelby Protocol\n");
    let path = Path::new(&args.path);
    if path.is_file() { check(&args.path, args.expected.as_deref()); }
    else if path.is_dir() { scan_dir(&args.path, args.recursive); }
    else { eprintln!("❌ Not found: {}", args.path); }
}
