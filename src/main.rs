mod group;

use std::{collections::HashMap, fs, process::Command};

use anyhow::{anyhow, Result};
use group::{get_duplicate_groups, Group};
use indicatif::ProgressBar;

fn main() -> Result<()> {
    // Read duplicate sets
    let groups = get_duplicate_groups(
        &r"D:\Google Takeout Processing\shared-and-zach.dupeguru".to_string(),
        99,
    )?;

    let groups: Vec<&Group> = groups
        .iter()
        .filter(|group| group.files.iter().all(is_jpeg)).collect();
    let mut file_aliases = HashMap::default();

    let bar = ProgressBar::new(groups.len() as u64);
    
    // Merge metadata into largest file copy
    for group in bar.wrap_iter(groups.into_iter()) {
        merge_group(group, &mut file_aliases)?;
    }

    // Read .lnk albums
    // Redirect file locations to largest
    // Store albums in text files

    Ok(())
}

fn merge_group<'a>(
    group: &'a Group,
    file_aliases: &mut HashMap<&'a String, &'a String>,
) -> Result<()> {
    println!("finding largest file");
    let (largest_file_index, largest_file) = group
        .files
        .iter()
        .enumerate()
        .max_by_key(|&(_, path)| get_file_size(path))
        .ok_or(anyhow!("Empty group"))?;

    println!("sorting files by metadata size");
    let mut other_files: Vec<&String> = group.files.iter().collect();
    other_files.remove(largest_file_index);
    other_files.sort_unstable_by_key(|&path| get_metadata_size(path));
    println!("done sorting");

    let largest_file_copy = create_temp_copy(&largest_file)?;
    for file in other_files {
        merge_metadata(file, largest_file);
        file_aliases.insert(file, largest_file);
    }
    merge_metadata(&largest_file_copy, largest_file);

    Ok(())
}

fn merge_metadata(source: &String, destination: &String) {
    println!("merging metadata from `{}` into `{}`", source, destination);
    println!("deleting `{}`", source);
}

fn create_temp_copy(file: &String) -> Result<String> {
    println!("creating temp copy of `{}`", file);
    Ok(format!("{}_temp", file))
}

fn is_jpeg(path: &String) -> bool {
    path.ends_with(".jpeg") || path.ends_with(".jpg")
}

fn get_file_size(path: &String) -> u64 {
    fs::metadata(path).expect("Failed to get metadata").len()
}

fn get_metadata_size(path: &String) -> usize {
    let metadata = Command::new("exiftool")
        .arg("-b")
        .arg(path)
        .output()
        .expect("Failed to get exif data");
    metadata.stdout.len()
}
