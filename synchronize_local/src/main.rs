use clap::Parser;
use std::fs;
use std::fs::metadata;
use std::io;
use std::path::Path;

#[derive(Parser)]
struct Cli {
    input_folder: std::path::PathBuf,
    output_folder: std::path::PathBuf,
    extension: String,
}

fn main() -> io::Result<()> {
    let args = Cli::parse();
    println!("Extract newer from  : {}", args.input_folder.display());
    println!("To : {}", args.output_folder.display());
    println!("Matching extension : {}", args.extension);

    let input_folder = &args.input_folder;
    let output_folder = &args.output_folder;
    let extension = &args.extension;

    process_directory(input_folder, input_folder, output_folder, extension)?;

    Ok(())
}

fn process_directory(
    path: &Path,
    input_folder: &Path,
    output_folder: &Path,
    extension: &str,
) -> io::Result<()> {
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            process_directory(&path, input_folder, output_folder, extension)?;
        } else if let Some(ext) = path.extension() {
            if ext == extension {
                println!("{}\n{}", input_folder.display(), path.display());
                let relative_path = path.strip_prefix(input_folder).unwrap();
                let output_file_path = Path::new(output_folder).join(relative_path);

                if file_is_newer(&path, &output_file_path)? {
                    copy_file(&path, &output_file_path)?;
                }
            }
        }
    }
    Ok(())
}

fn file_is_newer(path: &Path, other: &Path) -> io::Result<bool> {
    if !other.exists() {
        return Ok(true);
    }
    let input_meta = metadata(path)?;
    let target_meta = metadata(other)?;

    let input_modified = input_meta.modified()?;
    let target_modified = target_meta.modified()?;

    Ok(input_modified > target_modified)
}

fn copy_file(input_file: &Path, output_file: &Path) -> io::Result<()> {
    if let Some(parent) = output_file.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::copy(input_file, output_file)?;
    Ok(())
}
