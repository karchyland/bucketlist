use std::fs;
use std::fs::OpenOptions;
use std::io::{self, Write};
use std::num::NonZeroUsize;

use crate::path::bucket_path;

pub fn init() -> io::Result<()> {
    println!("init a new bucketlist");
    let path = bucket_path()?;

    match OpenOptions::new().write(true).create_new(true).open(&path) {
        Ok(_) => {
            println!("Created bucket list at {}", path.display());
            Ok(())
        }

        Err(err) if err.kind() == io::ErrorKind::AlreadyExists => {
            println!("Bucket list already exists at {}", path.display());
            Ok(())
        }

        Err(err) => Err(err),
    }
}

pub fn add(item: &str) -> io::Result<()> {
    println!("adding item: {item}");
    let path = bucket_path()?;

    let mut file = OpenOptions::new().append(true).open(path)?;

    writeln!(file, "{item}")?;

    Ok(())
}

pub fn kick(index: NonZeroUsize) -> io::Result<()> {
    let index = index.get();
    let path = bucket_path()?;

    let contents = fs::read_to_string(&path)?;
    let mut items: Vec<String> = contents.lines().map(String::from).collect();

    let Some(item) = items.get_mut(index - 1) else {
        println!("No item #{index} in your bucket.");
        return Ok(());
    };

    if item.ends_with(",x") {
        println!("Already kicked: {}", item.trim_end_matches(",x"));
        return Ok(());
    }

    item.push_str(",x");

    println!("Kicked: {}", item.trim_end_matches(",x"));

    fs::write(&path, format!("{}\n", items.join("\n")))?;

    Ok(())
}

pub fn remove(index: NonZeroUsize) -> io::Result<()> {
    let index = index.get();
    let path = bucket_path()?;

    let contents = fs::read_to_string(&path)?;
    let mut items: Vec<String> = contents.lines().map(String::from).collect();

    if index > items.len() {
        println!("No item #{index} in your bucket.");
        return Ok(());
    }

    let removed = items.remove(index - 1);

    fs::write(&path, format!("{}\n", items.join("\n")))?;

    println!("Removed: {}", removed.trim_end_matches(",x"));

    Ok(())
}

pub fn list() -> io::Result<()> {
    let path = bucket_path()?;
    let contents = fs::read_to_string(path)?;

    for (index, item) in contents.lines().enumerate() {
        let completed = item.ends_with(",x");
        let item = item.trim_end_matches(",x");

        let marker = if completed { "✓" } else { "○" };

        println!("{:>3}  {} {}", index + 1, marker, item);
    }

    Ok(())
}
