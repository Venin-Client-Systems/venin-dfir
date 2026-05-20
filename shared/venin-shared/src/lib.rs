pub mod file_id;
pub mod hashing;
pub mod timestamp;

pub use file_id::{identify_path, ForensicFileKind};
pub use hashing::{sha256_bytes, sha256_file};
pub use timestamp::{
    chromium_webkit_microseconds_to_utc, unix_seconds_to_chromium_webkit_microseconds,
};
