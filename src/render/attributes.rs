use chrono::{DateTime, Local};
use std::fs::Metadata;
use std::io::{self, Write};
use std::time::SystemTime;

use super::buffer::Buffer;

// Type definitions
type MetadataFormatter<W> = fn(&mut Buffer<W>, &Metadata) -> io::Result<()>;
pub type FnExtSize<W> = MetadataFormatter<W>;
pub type FnExtAccessTime<W> = MetadataFormatter<W>;
pub type FnExtBTime<W> = MetadataFormatter<W>;
pub type FnExtModTime<W> = MetadataFormatter<W>;
pub type FnExtPermission<W> = MetadataFormatter<W>;

#[cfg(unix)]
const PERMISSION_BITS: [(u32, char); 9] = [
    (0o400, 'r'),
    (0o200, 'w'),
    (0o100, 'x'),
    (0o040, 'r'),
    (0o020, 'w'),
    (0o010, 'x'),
    (0o004, 'r'),
    (0o002, 'w'),
    (0o001, 'x'),
];

const MAX_SUPPORTED_SIZE: u64 = 999_999_999_999; // ~931.32 GB

impl<W: Write> Buffer<W> {
    // Print methods with formatters
    pub fn print_size(&mut self, meta: &Metadata, f: FnExtSize<W>) -> io::Result<()> {
        f(self, meta)
    }

    pub fn print_atime(&mut self, meta: &Metadata, f: FnExtAccessTime<W>) -> io::Result<()> {
        f(self, meta)
    }

    pub fn print_btime(&mut self, meta: &Metadata, f: FnExtBTime<W>) -> io::Result<()> {
        f(self, meta)
    }

    pub fn print_mtime(&mut self, meta: &Metadata, f: FnExtModTime<W>) -> io::Result<()> {
        f(self, meta)
    }

    pub fn print_permission(&mut self, meta: &Metadata, f: FnExtPermission<W>) -> io::Result<()> {
        f(self, meta)
    }

    // Write methods
    pub fn write_size(&mut self, meta: &Metadata) -> io::Result<()> {
        let size = meta.len();
        self.write_padded_number(size)
    }

    pub fn write_atime(&mut self, meta: &Metadata) -> io::Result<()> {
        self.write_metadata_time(meta.accessed())
    }

    pub fn write_btime(&mut self, meta: &Metadata) -> io::Result<()> {
        self.write_metadata_time(meta.created())
    }

    pub fn write_mtime(&mut self, meta: &Metadata) -> io::Result<()> {
        self.write_metadata_time(meta.modified())
    }

    #[cfg(unix)]
    pub fn write_permission(&mut self, meta: &Metadata) -> io::Result<()> {
        use std::os::unix::fs::PermissionsExt;
        let mode = meta.permissions().mode();
        let permission_string = self.format_unix_permissions(meta.is_dir(), mode);
        self.write_surrounded_by_spaces(&permission_string)
    }

    // No-op implementations
    pub fn write_no_size(&mut self, _meta: &Metadata) -> io::Result<()> {
        Ok(())
    }
    pub fn write_no_atime(&mut self, _meta: &Metadata) -> io::Result<()> {
        Ok(())
    }
    pub fn write_no_btime(&mut self, _meta: &Metadata) -> io::Result<()> {
        Ok(())
    }
    pub fn write_no_mtime(&mut self, _meta: &Metadata) -> io::Result<()> {
        Ok(())
    }
    pub fn write_no_permission(&mut self, _meta: &Metadata) -> io::Result<()> {
        Ok(())
    }

    // Helper methods
    fn write_padded_number(&mut self, number: u64) -> io::Result<()> {
        let padded_string = if number > MAX_SUPPORTED_SIZE {
            String::from("  OVERFLOW  ")
        } else {
            format!("{:^12}", number)
        };

        self.bufwr.write_all(padded_string.as_bytes())
    }

    fn write_metadata_time(&mut self, time_result: io::Result<SystemTime>) -> io::Result<()> {
        match time_result {
            Ok(time) => {
                let formatted_time = Self::format_system_time(time);
                self.write_surrounded_by_spaces(&formatted_time)
            }
            Err(_) => self.write_surrounded_by_spaces("─────"),
        }
    }

    fn write_surrounded_by_spaces(&mut self, content: &str) -> io::Result<()> {
        self.write_space()?;
        self.bufwr.write_all(content.as_bytes())?;
        self.write_space()
    }

    // Utility methods
    fn format_system_time(time: SystemTime) -> String {
        let datetime: DateTime<Local> = time.into();
        datetime.format("%d-%m-%Y %H:%M").to_string()
    }

    #[cfg(unix)]
    fn format_unix_permissions(&self, is_dir: bool, mode: u32) -> String {
        let file_type = if is_dir { 'd' } else { '.' };
        let permissions = PERMISSION_BITS
            .iter()
            .map(|&(mask, ch)| if mode & mask != 0 { ch } else { '-' })
            .collect::<String>();

        format!("{}{}", file_type, permissions)
    }
}
