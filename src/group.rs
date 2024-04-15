use std::{collections::HashSet, fs};

use anyhow::{anyhow, bail, Result};
use indicatif::ProgressBar;
use roxmltree::{Document, Node};

pub fn get_duplicate_groups(
    dupe_file_path: &String,
    min_match_percentage: u8,
) -> Result<Vec<Group>> {
    let xml = fs::read_to_string(dupe_file_path)?;
    let doc = Document::parse(xml.as_str())?;

    let root_element = doc.root_element();
    if root_element.tag_name().name() != "results" {
        bail!(anyhow!("Root element is not `results`"));
    }

    let mut groups: Vec<Group> = Vec::new();
    let group_nodes: Vec<_> = root_element.children().collect();

    let bar = ProgressBar::new(group_nodes.len() as u64);
    let group_nodes = group_nodes.into_iter();

    for group in bar.wrap_iter(group_nodes) {
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
                "file" => files.push(try_get_attribute(&file_or_match, "path")?),
                "match" => matches.push(Match::new(&file_or_match)?),
                name => return Err(anyhow!("Unexpected tag in `group`: `{}`", name)),
            }
        }

        // Filter out sub-threshold matching files
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

#[derive(Debug)]
pub struct Group {
    pub files_by_size: Vec<String>,
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

#[derive(Debug)]
pub struct Match {
    pub first: usize,
    pub second: usize,
    pub percentage: u8,
}

impl Match {
    pub fn new(node: &Node<'_, '_>) -> Result<Self> {
        Ok(Match {
            first: try_get_attribute(&node, "first")?.parse()?,
            second: try_get_attribute(&node, "second")?.parse()?,
            percentage: try_get_attribute(&node, "percentage")?.parse()?,
        })
    }
}

fn try_get_attribute<'a>(node: &Node<'a, '_>, name: &str) -> Result<&'a str> {
    node.attribute(name)
        .ok_or_else(|| anyhow!("Attribute `{}` not found in node `{:?}`", name, node))
}
