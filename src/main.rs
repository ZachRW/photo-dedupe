mod group;

use anyhow::Result;
use group::{get_duplicate_groups, Group};

fn main() -> Result<()> {
    // Read duplicate sets
    let groups = get_duplicate_groups(
        &r"D:\Google Takeout Processing\shared-and-zach.dupeguru".to_string(),
        99,
    )?;
    // dbg!(groups.len());
    // let file_count: usize = groups.iter().map(|group| group.files_by_size.len()).sum();
    // dbg!(file_count);
    for group in groups {
        if group.files_by_size.iter().any(|f| {
            let mut ext1 = f.split('.').last().unwrap().to_lowercase();
            let mut ext2 = group.files_by_size[0]
                .split('.')
                .last()
                .unwrap()
                .to_lowercase();
            if ext1 == "jpeg" {
                ext1 = "jpg".to_string();
            }
            if ext2 == "jpeg" {
                ext2 = "jpg".to_string();
            }

            ext1 != ext2
        }) {
            dbg!(group);
        }
    }

    // Merge metadata into largest file copy
    // Read .lnk albums
    // Redirect file locations to largest
    // Store albums in text files

    Ok(())
}

fn merge_metadata(group: &Group) {}
