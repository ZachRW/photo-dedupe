use std::{collections::HashSet, fs};

use anyhow::{anyhow, Result};
use roxmltree::{Document, Node};

fn main() -> Result<()> {
    // Read duplicate sets
    get_duplicate_groups(
        &r"D:\Google Takeout Processing\dupeguru-test.dupeguru".to_string(),
        99,
    )?;

    // Merge metadata into largest file copy
    // Read .lnk albums
    // Redirect file locations to largest
    // Store albums in text files

    Ok(())
}

fn get_duplicate_groups(dupe_file_path: &String, min_match_percentage: u8) -> Result<Vec<Group>> {
    let xml = fs::read_to_string(dupe_file_path)?;
    let doc = Document::parse(xml.as_str())?;

    let root_element = doc.root_element();
    if root_element.tag_name().name() != "results" {
        return Err(anyhow!("Root element is not `results`"));
    }

    let mut groups: Vec<Group> = Vec::new();

    for group in root_element.children() {
        if group.tag_name().name() != "group" {
            return Err(anyhow!(
                "Unexpected tag in `results`: `{}`",
                group.tag_name().name()
            ));
        }

        let mut files = Vec::new();
        let mut matches = Vec::new();

        for file_or_match in group.children() {
            match file_or_match.tag_name().name() {
                "file" => files.push(attribute_or(&file_or_match, "file")?),
                "match" => matches.push(Match::new(&file_or_match)?),
                name => return Err(anyhow!("Unexpected tag in `group`: `{}`", name)),
            }
        }

        let mut matching_threshold_files = HashSet::with_capacity(files.len());
        for file_match in matches {
            if file_match.percentage >= min_match_percentage {
                matching_threshold_files.insert(file_match.first);
                matching_threshold_files.insert(file_match.second);
            }
        }
        let matching_threshold_files: Vec<String> = matching_threshold_files
            .into_iter()
            .map(|file_index| String::from(files[file_index]))
            .collect();

        if !matching_threshold_files.is_empty() {
            groups.push(Group::new(matching_threshold_files)?);
        }
    }

    Ok(groups)
}

struct Group {
    files_by_size: Vec<String>,
}

impl Group {
    pub fn new(files: Vec<String>) -> Result<Self> {
        let mut files_with_sizes: Vec<(String, u64)> = Vec::with_capacity(files.len());
        for file in files.into_iter() {
            let size = fs::metadata(&file)?.len();
            files_with_sizes.push((file, size));
        }

        files_with_sizes.sort_unstable_by_key(|(_, size)| *size);
        let sorted_files: Vec<String> =
            files_with_sizes.into_iter().map(|(file, _)| file).collect();

        Ok(Group {
            files_by_size: sorted_files,
        })
    }
}

struct Match {
    pub first: usize,
    pub second: usize,
    pub percentage: u8,
}

impl Match {
    pub fn new(node: &Node<'_, '_>) -> Result<Self> {
        Ok(Match {
            first: attribute_or(&node, "first")?.parse()?,
            second: attribute_or(&node, "second")?.parse()?,
            percentage: attribute_or(&node, "percentage")?.parse()?,
        })
    }
}

fn attribute_or<'a>(node: &Node<'a, '_>, name: &str) -> Result<&'a str> {
    node.attribute(name)
        .ok_or_else(|| anyhow!("Attribute `{}` not found in node `{:?}`", name, node))
}
