use std::path::PathBuf;

#[derive(Debug)]
pub struct Segments {
    pub valid: Vec<PathBuf>,   // Segments that exist
    pub missing: Vec<PathBuf>, // Segments not found
}

/// Validates all segments in an M3U8 file.
impl Segments {
    pub fn validate(segments: &[PathBuf]) -> Self {
        let (valid, missing): (Vec<_>, Vec<_>) = segments.iter().cloned().partition(|p| p.exists());

        Self {
            valid: valid,
            missing: missing,
        }
    }
}
