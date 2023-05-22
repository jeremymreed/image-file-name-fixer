#[derive(Debug)]
pub struct ImageData {
    pub absolute_path: String,
    pub final_absolute_path: String,
    pub format: String,
    pub width: u32,
    pub height: u32,
    pub hash: String,
}
