use std::fs;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::{Path, PathBuf};
use std::collections::HashSet;

use crate::models::Subdomain;
use crate::errors::Result;

pub fn create_output_dir() -> Result<()> {
    let dir_name = "output";
    match fs::metadata(dir_name) {
        Ok(metadata) => {
            if metadata.is_dir() {
                Ok(())
            } else {
                Err(std::io::Error::new(
                    std::io::ErrorKind::AlreadyExists,
                    "A file with the name 'output' already exists",
                ).into())
            }
        }
        Err(_error) => {
            fs::create_dir(dir_name)?;
            Ok(())
        }
    }
}

pub fn create_output_file(output_file: &str, subdomains: &[Subdomain]) -> Result<()> {
    let dir_name = "output";
    let dir_path = Path::new(dir_name);
    if !dir_path.exists() {
        fs::create_dir(dir_name)?;
    }

    let mut new_file_path = PathBuf::from(dir_name);
    new_file_path.push(output_file);

    let mut file = File::create(&new_file_path)?;
    for subdomain in subdomains.iter() {
        writeln!(file, "{}", subdomain.url)?;
    }

    Ok(())
}

pub fn read_wordlists(paths: &[String]) -> Result<Vec<String>> {
    let mut words = HashSet::new();

    for path_str in paths {
        let path = Path::new(path_str);
        if path.is_dir() {
            for entry in fs::read_dir(path)? {
                let entry = entry?;
                if entry.path().is_file() {
                    read_file_to_set(&entry.path(), &mut words)?;
                }
            }
        } else if path.is_file() {
            read_file_to_set(path, &mut words)?;
        }
    }

    Ok(words.into_iter().collect())
}

fn read_file_to_set(path: &Path, set: &mut HashSet<String>) -> Result<()> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let trimmed = line.trim();
        if !trimmed.is_empty() {
            set.insert(trimmed.to_string());
        }
    }

    Ok(())
}
