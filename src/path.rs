use std::{io, path::PathBuf};

pub fn bucket_path() -> io::Result<PathBuf> {
    let home = home::home_dir().ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::NotFound,
            "could not determine home directory",
        )
    })?;

    Ok(home.join(".bucketlist"))
}
