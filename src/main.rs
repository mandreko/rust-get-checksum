use std::env;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;

fn get_file_sha256_hash(file_path: &Path) -> io::Result<String> {
    let bytes = std::fs::read(file_path)?;
    let hash = sha256::digest(&bytes);
    Ok(hash)
}

fn main() -> io::Result<()> {
    let packages_dir = env::var("INPUT_PACKAGES_DIR")
        .expect("INPUT_PACKAGES_DIR environment variable not set");
    let file_path = env::var("INPUT_FILE_PATH")
        .expect("INPUT_FILE_PATH environment variable not set");

    let output_path = if Path::new(&file_path).is_dir() {
        Path::new(&file_path).join("sha256-checksums.txt")
    } else {
        Path::new(&file_path).to_path_buf()
    };

    println!("Packages directory: {}", packages_dir);
    println!("File which will contain SHA256 checksums will be: {:?}", output_path);

    let mut hashes = String::new();
    for entry in fs::read_dir(&packages_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            if let Ok(file_hash) = get_file_sha256_hash(&path) {
                if let Some(file_name) = path.file_name() {
                    hashes.push_str(&format!("{}  {}\n", file_hash, file_name.to_string_lossy()));
                }
            }
        }
    }

    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent)?;
    }
    let mut output_file = File::create(&output_path)?;
    output_file.write_all(hashes.as_bytes())?;

    println!("{}", hashes);
    println!("Saved checksums in file: {:?} >> $GITHUB_OUTPUT", output_path);

    Ok(())
}
