mod group;

use anyhow::Result;
use group::get_duplicate_groups;

fn main() -> Result<()> {
    // Read duplicate sets
    let groups = get_duplicate_groups(
        &r"D:\Google Takeout Processing\shared-and-zach.dupeguru".to_string(),
        99,
    )?;
    dbg!(groups.len());
    let file_count: usize = groups.iter().map(|group| group.files_by_size.len()).sum();
    dbg!(file_count);

    // Merge metadata into largest file copy
    // Read .lnk albums
    // Redirect file locations to largest
    // Store albums in text files

    Ok(())
}
