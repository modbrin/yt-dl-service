use std::ffi::OsStr;
use std::path::Path;
use std::fs::read_dir;

pub fn contains_unfinished_downloads<P>(dir_path: P) -> Result<bool, &'static str>
where
    P: AsRef<Path>,
{
    let paths = read_dir(dir_path).map_err(
        |_| "Failed to list directory via indicated path."
    )?;
    let result = paths.into_iter()
        .filter_map(|e| e.map(|v| v.path()).ok())
        .any(|f| f.extension() == Some("part".as_ref()));
    Ok(result)
}

pub fn process_channel(url: String) -> Result<(), String> {
    todo!()
}