//! Cross-platform crate to control system audio volume.
//!
//! This crate re-exports the platform-appropriate `AudioDevice` implementation.
//! The underlying backend is selected automatically at compile time based on
//! the target operating system:
//!
//! | Platform | Backend crate              | Feature flag to enable real impl |
//! |----------|----------------------------|----------------------------------|
//! | Linux    | `volumecontrol-linux`      | `pulseaudio`                     |
//! | Windows  | `volumecontrol-windows`    | `wasapi`                         |
//! | macOS    | `volumecontrol-macos`      | `coreaudio`                      |
//!
//! # Example
//!
//! ```no_run
//! use volumecontrol::AudioDevice;
//! use volumecontrol_core::AudioDevice as AudioDeviceTrait;
//!
//! fn main() -> Result<(), volumecontrol_core::AudioError> {
//!     let device = AudioDevice::default()?;
//!     let vol = device.get_vol()?;
//!     println!("Current volume: {vol}%");
//!     Ok(())
//! }
//! ```

pub use volumecontrol_core::AudioError;

#[cfg(target_os = "linux")]
pub use volumecontrol_linux::AudioDevice;

#[cfg(target_os = "windows")]
pub use volumecontrol_windows::AudioDevice;

#[cfg(target_os = "macos")]
pub use volumecontrol_macos::AudioDevice;

#[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
compile_error!(
    "volumecontrol does not support the current target OS. \
     Supported targets: linux, windows, macos."
);
