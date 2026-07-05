use crate::model::Project;
use std::fs;
use std::io;
use std::path::PathBuf;

fn data_file_path() -> io::Result<PathBuf> {
    let mut dir = dirs::data_dir().ok_or_else(|| {
        io::Error::new(io::ErrorKind::NotFound, "could not determine data directory")
    })?;
    dir.push("termban");
    fs::create_dir_all(&dir)?;
    dir.push("projects.json");
    Ok(dir)
}

/// Serializes all projects to disk as pretty-printed JSON.
pub fn save_projects(projects: &[Project]) -> io::Result<()> {
    let path = data_file_path()?;
    let json = serde_json::to_string_pretty(projects)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    // Write to a temp file then rename, so a crash mid-write can't corrupt
    // the existing save file.
    let tmp_path = path.with_extension("json.tmp");
    fs::write(&tmp_path, json)?;
    fs::rename(tmp_path, path)
}

/// Loads projects from disk. Returns an empty Vec if no save file exists yet
/// (e.g. first run), rather than treating that as an error.
pub fn load_projects() -> io::Result<Vec<Project>> {
    let path = data_file_path()?;
    if !path.exists() {
        return Ok(Vec::new());
    }
    let json = fs::read_to_string(path)?;
    serde_json::from_str(&json).map_err(|e| io::Error::new(io::ErrorKind::Other, e))
}