use std::{
    fs::{File, OpenOptions},
    io,
    path::PathBuf,
};

pub fn touch(path_buf: &PathBuf) -> Result<File, io::Error> {
    let file = OpenOptions::new().create(true).write(true).open(path_buf)?;
    Ok(file)
}
