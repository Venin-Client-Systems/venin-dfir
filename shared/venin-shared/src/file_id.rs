use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ForensicFileKind {
    SQLiteDatabase,
    WindowsLnk,
    WindowsPrefetch,
    Plist,
    Unknown,
}

pub fn identify_path(path: impl AsRef<Path>) -> io::Result<ForensicFileKind> {
    let path = path.as_ref();
    let mut file = File::open(path)?;
    let mut header = [0_u8; 32];
    let read = file.read(&mut header)?;
    let header = &header[..read];

    if header.starts_with(b"SQLite format 3\0") {
        return Ok(ForensicFileKind::SQLiteDatabase);
    }

    if header.len() >= 4 && header[0..4] == [0x4c, 0x00, 0x00, 0x00] {
        return Ok(ForensicFileKind::WindowsLnk);
    }

    if header.starts_with(b"SCCA") {
        return Ok(ForensicFileKind::WindowsPrefetch);
    }

    if header.starts_with(b"bplist00") || header.starts_with(b"<?xml") {
        return Ok(ForensicFileKind::Plist);
    }

    Ok(ForensicFileKind::Unknown)
}
