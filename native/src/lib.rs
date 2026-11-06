extern crate neon;

#[cfg(windows)]
extern crate clipboard_win;

#[cfg(target_os="macos")]
#[macro_use]
extern crate objc;
#[cfg(target_os="macos")]
extern crate objc_id;
#[cfg(target_os="macos")]
extern crate objc_foundation;

#[cfg(windows)]
pub mod windows_clipboard;

#[cfg(target_os="macos")]
pub mod osx_clipboard;
