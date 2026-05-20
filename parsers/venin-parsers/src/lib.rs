pub mod browser;
pub mod mobile;
pub mod windows;

use venin_core::{ToolRegistry, ToolRegistryError};

pub fn register_builtin_tools(registry: &mut ToolRegistry) -> Result<(), ToolRegistryError> {
    registry.register(browser::chromium_history::ChromiumHistoryParser::metadata())?;
    registry.register(windows::usb_artifacts::metadata())?;
    registry.register(windows::recent_files::metadata())?;
    registry.register(windows::lnk::metadata())?;
    registry.register(windows::prefetch::metadata())?;
    registry.register(mobile::ios_backup::metadata())?;
    registry.register(mobile::android_sqlite::metadata())?;
    Ok(())
}
