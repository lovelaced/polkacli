use crate::error::Result;
use serde_json::Value;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::error::Error;

/// Load a JSON file from the given path and deserialize it into a `serde_json::Value`.
/// Provides detailed error messages if the file cannot be opened or if the JSON is invalid.
pub fn load_json_from_file(path: &Path) -> Result<Value> {
    // Attempt to open the file, providing a specific error message if it fails.
    let file = File::open(path).map_err(|e| {
        format!(
            "Failed to open JSON file at {}: {}",
            path.display(),
            e.description()
        )
    })?;
    
    // Attempt to parse the JSON from the file, providing a specific error message if it fails.
    serde_json::from_reader(file).map_err(|e| {
        format!(
            "Failed to parse JSON from file at {}: {}",
            path.display(),
            e.description()
        )
        .into()
    })
}

/// Find an image file that matches the given JSON file's name.
/// Looks for .jpg, .jpeg, and .png extensions in the same directory as the JSON file.
/// Provides detailed error messages if the parent directory or file stem cannot be determined,
/// or if no matching image is found.
pub fn find_image_for_json(json_path: &Path) -> Result<PathBuf> {
    // Ensure the JSON file has a parent directory.
    let parent_dir = json_path.parent().ok_or_else(|| {
        format!(
            "Failed to determine parent directory for JSON file at {}",
            json_path.display()
        )
    })?;
    
    // Ensure the JSON file has a valid stem (filename without extension).
    let json_stem = json_path.file_stem().ok_or_else(|| {
        format!(
            "Failed to extract file stem from JSON file name at {}",
            json_path.display()
        )
    })?.to_string_lossy();
    
    // Check for image files with the same stem and various extensions.
    for extension in &["jpg", "jpeg", "png"] {
        let image_path = parent_dir.join(format!("{}.{}", json_stem, extension));
        if image_path.exists() {
            return Ok(image_path);
        }
    }

    // If no image is found, return an error with a detailed message.
    Err(format!(
        "No matching image found for JSON file at {}. Looked for .jpg, .jpeg, and .png files with the same name.",
        json_path.display()
    ).into())
}

