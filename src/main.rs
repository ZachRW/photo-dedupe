mod group;

use std::{fs, process::Command};

use anyhow::{anyhow, Result};
use group::{get_duplicate_groups, Group};

fn main() -> Result<()> {
    // Read duplicate sets
    let groups = get_duplicate_groups(
        &r"D:\Google Takeout Processing\shared-and-zach.dupeguru".to_string(),
        99,
    )?;

    let groups = groups
        .into_iter()
        .filter(|group| group.files.iter().all(is_jpeg));

    for group in groups {
        let (largest_file_index, largest_file) = group
            .files
            .iter()
            .enumerate()
            .max_by_key(|&(_, path)| get_file_size(path))
            .ok_or(anyhow!("Empty group"))?;

        let mut other_files: Vec<&String> = group.files.iter().collect();
        other_files.remove(largest_file_index);
        other_files.sort_unstable_by_key(|&path| get_metadata_size(path));

        
    }

    // Merge metadata into largest file copy
    // Read .lnk albums
    // Redirect file locations to largest
    // Store albums in text files

    Ok(())
}

fn is_jpeg(path: &String) -> bool {
    path.ends_with(".jpeg") || path.ends_with(".jpg")
}

fn get_file_size(path: &String) -> u64 {
    fs::metadata(path).expect("Failed to get metadata").len()
}

fn get_metadata_size(path: &String) -> usize {
    let metadata = Command::new("exiftool")
        .arg("-v")
        .arg(path)
        .output()
        .expect("Failed to get exif data");
    metadata.stdout.len()
}

fn merge_metadata(group: &Group) {}
