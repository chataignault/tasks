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
                let relative_path = path.strip_prefix(input_folder).unwrap();
                let output_file_path = Path::new(output_folder).join(relative_path);

                if file_is_newer(&path, &output_file_path)? {
                    println!(
                        "Update : {} into {}",
                        &path.display(),
                        &output_file_path.display()
                    );
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use std::thread::sleep;
    use std::time::{Duration, SystemTime};
    use tempfile::TempDir;

    #[test]
    fn test_process_directory() -> io::Result<()> {
        // Create temporary directories for input and output
        let input_dir = TempDir::new()?;
        let output_dir = TempDir::new()?;

        let extension = "txt";

        // Create test file structure in input directory
        let input_paths = [
            (input_dir.path().join("file1.txt"), "content1"),
            (input_dir.path().join("file2.txt"), "content2"),
            (input_dir.path().join("file3.doc"), "content3"), // Different extension, should be ignored
            (
                input_dir.path().join("subdir").join("file4.txt"),
                "content4",
            ),
        ];

        // Create test file structure in output directory
        let output_paths = [
            (output_dir.path().join("file1.txt"), "old_content1"), // Will be updated (older)
            (output_dir.path().join("file2.txt"), "same_content2"), // Won't be updated (newer)
            (
                output_dir.path().join("subdir").join("file4.txt"),
                "old_content4",
            ), // Will be updated (older)
        ];

        // Create input files
        for (path, content) in &input_paths {
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)?;
            }
            let mut file = File::create(path)?;
            file.write_all(content.as_bytes())?;
        }

        // Create output files (with controlled timestamps)
        for (path, content) in &output_paths {
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)?;
            }
            let mut file = File::create(path)?;
            file.write_all(content.as_bytes())?;

            // Make file2.txt in output directory newer than its input counterpart
            if path.file_name().unwrap() == "file2.txt" {
                // Set modification time to current time (which will be newer than input file)
                let now = SystemTime::now();
                filetime::set_file_mtime(path, filetime::FileTime::from_system_time(now))?;

                // Also update input file's time to be older
                let older_time = now - Duration::from_secs(10);
                filetime::set_file_mtime(
                    &input_paths[1].0,
                    filetime::FileTime::from_system_time(older_time),
                )?;
            } else {
                // Make other output files older than input files
                let now = SystemTime::now();
                let older_time = now - Duration::from_secs(100);
                filetime::set_file_mtime(path, filetime::FileTime::from_system_time(older_time))?;
            }
        }

        // Small delay to ensure file timestamps are different
        sleep(Duration::from_millis(10));

        // Run the function we're testing
        process_directory(
            input_dir.path(),
            input_dir.path(),
            output_dir.path(),
            extension,
        )?;

        // Verify results

        // file1.txt should have been updated
        let output_content1 = fs::read_to_string(output_dir.path().join("file1.txt"))?;
        assert_eq!(
            output_content1, "content1",
            "file1.txt should have been updated"
        );

        // file2.txt should NOT have been updated because the output version is newer
        let output_content2 = fs::read_to_string(output_dir.path().join("file2.txt"))?;
        assert_eq!(
            output_content2, "same_content2",
            "file2.txt should not have been updated"
        );

        // file3.doc should not exist in output because it has wrong extension
        assert!(
            !output_dir.path().join("file3.doc").exists(),
            "file3.doc should not exist in output"
        );

        // subdir/file4.txt should have been updated
        let output_content4 =
            fs::read_to_string(output_dir.path().join("subdir").join("file4.txt"))?;
        assert_eq!(
            output_content4, "content4",
            "subdir/file4.txt should have been updated"
        );

        Ok(())
    }
}
