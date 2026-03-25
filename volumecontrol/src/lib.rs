//! Cross-platform crate to control system audio volume.
//!
//! This crate provides a unified [`AudioDevice`] type that works on Linux,
//! Windows, and macOS. The correct backend is selected automatically at
//! compile time; no feature flags or sub-crate imports are required.
//!
//! | Platform | Backend              | System library required |
//! |----------|----------------------|-------------------------|
//! | Linux    | PulseAudio           | `libpulse-dev`          |
//! | Windows  | WASAPI               | built-in                |
//! | macOS    | CoreAudio            | built-in                |
//!
//! # Example
//!
//! ```no_run
//! use volumecontrol::AudioDevice;
//!
//! fn main() -> Result<(), volumecontrol::AudioError> {
//!     let device = AudioDevice::default()?;
//!     println!("Current volume: {}%", device.get_vol()?);
//!     Ok(())
//! }
//! ```
//!
//! Alternatively, use the [`prelude`] module for a glob import:
//!
//! ```no_run
//! use volumecontrol::prelude::*;
//!
//! fn main() -> Result<(), AudioError> {
//!     let device = AudioDevice::default()?;
//!     println!("Volume: {}%", device.get_vol()?);
//!     Ok(())
//! }
//! ```

pub use volumecontrol_core::AudioError;

use volumecontrol_core::AudioDevice as AudioDeviceTrait;

#[cfg(target_os = "linux")]
use volumecontrol_linux::AudioDevice as Inner;

#[cfg(target_os = "windows")]
use volumecontrol_windows::AudioDevice as Inner;

#[cfg(target_os = "macos")]
use volumecontrol_macos::AudioDevice as Inner;

#[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
compile_error!(
    "volumecontrol does not support the current target OS. \
     Supported targets: linux, windows, macos."
);

/// A cross-platform audio output device.
///
/// Wraps the platform-appropriate backend and exposes a uniform API for
/// querying and changing the system volume and mute state.  No trait imports
/// are required to use the methods below.
#[derive(Debug)]
pub struct AudioDevice(Inner);

impl AudioDevice {
    /// Returns the system default audio output device.
    ///
    /// # Errors
    ///
    /// Returns an error if the default device cannot be resolved.
    #[allow(clippy::should_implement_trait)]
    pub fn default() -> Result<Self, AudioError> {
        <Inner as AudioDeviceTrait>::default().map(Self)
    }

    /// Returns the audio device identified by `id`.
    ///
    /// # Errors
    ///
    /// Returns [`AudioError::DeviceNotFound`] if no device with the given
    /// identifier exists, or another error if the lookup fails.
    pub fn from_id(id: &str) -> Result<Self, AudioError> {
        <Inner as AudioDeviceTrait>::from_id(id).map(Self)
    }

    /// Returns the first audio device whose name contains `name`.
    ///
    /// # Errors
    ///
    /// Returns [`AudioError::DeviceNotFound`] if no matching device is found,
    /// or another error if the lookup fails.
    pub fn from_name(name: &str) -> Result<Self, AudioError> {
        <Inner as AudioDeviceTrait>::from_name(name).map(Self)
    }

    /// Lists all available audio devices as `(id, name)` pairs.
    ///
    /// # Errors
    ///
    /// Returns an error if the device list cannot be retrieved.
    pub fn list() -> Result<Vec<(String, String)>, AudioError> {
        <Inner as AudioDeviceTrait>::list()
    }

    /// Returns the current volume level in the range `0..=100`.
    ///
    /// # Errors
    ///
    /// Returns an error if the volume cannot be read.
    pub fn get_vol(&self) -> Result<u8, AudioError> {
        self.0.get_vol()
    }

    /// Sets the volume level.
    ///
    /// `vol` is clamped to `0..=100` before being applied.
    ///
    /// # Errors
    ///
    /// Returns an error if the volume cannot be set.
    pub fn set_vol(&self, vol: u8) -> Result<(), AudioError> {
        self.0.set_vol(vol)
    }

    /// Returns `true` if the device is currently muted.
    ///
    /// # Errors
    ///
    /// Returns an error if the mute state cannot be read.
    pub fn is_mute(&self) -> Result<bool, AudioError> {
        self.0.is_mute()
    }

    /// Mutes or unmutes the device.
    ///
    /// # Errors
    ///
    /// Returns an error if the mute state cannot be changed.
    pub fn set_mute(&self, muted: bool) -> Result<(), AudioError> {
        self.0.set_mute(muted)
    }
}

/// A convenience prelude re-exporting the items needed for typical usage.
///
/// ```no_run
/// use volumecontrol::prelude::*;
///
/// fn main() -> Result<(), AudioError> {
///     let device = AudioDevice::default()?;
///     println!("Volume: {}%", device.get_vol()?);
///     Ok(())
/// }
/// ```
pub mod prelude {
    pub use super::AudioDevice;
    pub use super::AudioError;
}

// ── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── Linux integration tests ──────────────────────────────────────────────
    //
    // These require a running PulseAudio server with at least one sink.
    // In CI a virtual null sink is provisioned before the test suite runs.
    // Run with `--test-threads=1` to avoid races on shared audio state.

    /// The default device must be resolvable when PulseAudio is running.
    #[cfg(target_os = "linux")]
    #[test]
    fn default_returns_ok() {
        let result = AudioDevice::default();
        assert!(result.is_ok(), "expected Ok, got {result:?}");
    }

    /// `list()` must return at least one device with non-empty id and name.
    #[cfg(target_os = "linux")]
    #[test]
    fn list_returns_nonempty() {
        let devices = AudioDevice::list().expect("list()");
        assert!(
            !devices.is_empty(),
            "expected at least one audio device from list()"
        );
        for (id, name) in &devices {
            assert!(!id.is_empty(), "device id must not be empty");
            assert!(!name.is_empty(), "device name must not be empty");
        }
    }

    /// Looking up a device by id obtained from `list()` must succeed.
    #[cfg(target_os = "linux")]
    #[test]
    fn from_id_valid_id_returns_ok() {
        let devices = AudioDevice::list().expect("list()");
        let (id, _name) = devices.first().expect("at least one device in list");
        let found = AudioDevice::from_id(id);
        assert!(
            found.is_ok(),
            "from_id with a valid id should succeed, got {found:?}"
        );
    }

    /// A sink name that does not exist must return `DeviceNotFound`.
    #[cfg(target_os = "linux")]
    #[test]
    fn from_id_nonexistent_returns_not_found() {
        match AudioDevice::from_id("__nonexistent_sink_xyz__") {
            Err(AudioError::DeviceNotFound) => {}
            other => panic!("expected DeviceNotFound, got {other:?}"),
        }
    }

    /// A partial description substring of a listed device must match.
    #[cfg(target_os = "linux")]
    #[test]
    fn from_name_partial_match_returns_ok() {
        let devices = AudioDevice::list().expect("list()");
        let (_id, name) = devices.first().expect("at least one device in list");
        let partial: String = name.chars().take(3).collect();
        let found = AudioDevice::from_name(&partial);
        assert!(
            found.is_ok(),
            "from_name with partial match '{partial}' should succeed"
        );
    }

    /// A description that matches no sink must return `DeviceNotFound`.
    #[cfg(target_os = "linux")]
    #[test]
    fn from_name_no_match_returns_not_found() {
        match AudioDevice::from_name("\x00\x01\x02") {
            Err(AudioError::DeviceNotFound) => {}
            other => panic!("expected DeviceNotFound, got {other:?}"),
        }
    }

    /// The reported volume must always be within the valid `0..=100` range.
    #[cfg(target_os = "linux")]
    #[test]
    fn get_vol_returns_valid_range() {
        let device = AudioDevice::default().expect("default()");
        let vol = device.get_vol().expect("get_vol()");
        assert!(vol <= 100, "volume must be in 0..=100, got {vol}");
    }

    /// Setting the volume to a different value must be reflected when read back.
    ///
    /// The original volume is restored so that other tests are not affected.
    /// Run with `--test-threads=1` to avoid races.
    #[cfg(target_os = "linux")]
    #[test]
    fn set_vol_changes_volume() {
        let device = AudioDevice::default().expect("default()");
        let original = device.get_vol().expect("get_vol()");
        let target: u8 = if original >= 50 { 30 } else { 70 };
        device.set_vol(target).expect("set_vol()");
        let after = device.get_vol().expect("get_vol() after set");
        // Allow ±1 rounding error due to f32 ↔ u8 conversion.
        assert!(
            after.abs_diff(target) <= 1,
            "expected volume near {target}, got {after}"
        );
        device.set_vol(original).expect("restore original volume");
    }

    /// Toggling the mute state must be reflected when read back.
    ///
    /// The original mute state is restored so that other tests are not affected.
    /// Run with `--test-threads=1` to avoid races.
    #[cfg(target_os = "linux")]
    #[test]
    fn set_mute_changes_mute_state() {
        let device = AudioDevice::default().expect("default()");
        let original = device.is_mute().expect("is_mute()");
        let target = !original;
        device.set_mute(target).expect("set_mute()");
        let after = device.is_mute().expect("is_mute() after set");
        assert_eq!(after, target, "mute state should be {target}, got {after}");
        device
            .set_mute(original)
            .expect("restore original mute state");
    }

    // ── macOS integration tests ──────────────────────────────────────────────
    //
    // These exercise the CoreAudio stack on macOS hardware runners.

    /// The default device must be resolvable on macOS.
    #[cfg(target_os = "macos")]
    #[test]
    fn default_returns_ok() {
        let result = AudioDevice::default();
        assert!(result.is_ok(), "expected Ok, got {result:?}");
    }

    /// `list()` must return at least one device with non-empty id and name.
    #[cfg(target_os = "macos")]
    #[test]
    fn list_returns_nonempty() {
        let devices = AudioDevice::list().expect("list()");
        assert!(
            !devices.is_empty(),
            "expected at least one audio device from list()"
        );
        for (id, name) in &devices {
            assert!(!id.is_empty(), "device id must not be empty");
            assert!(!name.is_empty(), "device name must not be empty");
        }
    }

    /// Looking up a device by id obtained from `list()` must succeed.
    #[cfg(target_os = "macos")]
    #[test]
    fn from_id_valid_id_returns_ok() {
        let devices = AudioDevice::list().expect("list()");
        let (id, _name) = devices.first().expect("at least one device in list");
        let found = AudioDevice::from_id(id);
        assert!(
            found.is_ok(),
            "from_id with a valid id should succeed, got {found:?}"
        );
    }

    /// A non-numeric id must return `DeviceNotFound`.
    #[cfg(target_os = "macos")]
    #[test]
    fn from_id_nonexistent_returns_not_found() {
        match AudioDevice::from_id("not-a-number") {
            Err(AudioError::DeviceNotFound) => {}
            other => panic!("expected DeviceNotFound, got {other:?}"),
        }
    }

    /// A partial description substring of a listed device must match.
    #[cfg(target_os = "macos")]
    #[test]
    fn from_name_partial_match_returns_ok() {
        let devices = AudioDevice::list().expect("list()");
        let (_id, name) = devices.first().expect("at least one device in list");
        let partial: String = name.chars().take(3).collect();
        let found = AudioDevice::from_name(&partial);
        assert!(
            found.is_ok(),
            "from_name with partial match '{partial}' should succeed"
        );
    }

    /// A description that matches no device must return `DeviceNotFound`.
    #[cfg(target_os = "macos")]
    #[test]
    fn from_name_no_match_returns_not_found() {
        match AudioDevice::from_name("\x00\x01\x02") {
            Err(AudioError::DeviceNotFound) => {}
            other => panic!("expected DeviceNotFound, got {other:?}"),
        }
    }

    /// The reported volume must always be within the valid `0..=100` range.
    #[cfg(target_os = "macos")]
    #[test]
    fn get_vol_returns_valid_range() {
        let device = AudioDevice::default().expect("default()");
        let vol = device.get_vol().expect("get_vol()");
        assert!(vol <= 100, "volume must be in 0..=100, got {vol}");
    }

    /// Setting the volume to the same value must not change it (±1 rounding).
    #[cfg(target_os = "macos")]
    #[test]
    fn set_vol_roundtrip() {
        let device = AudioDevice::default().expect("default()");
        let original = device.get_vol().expect("get_vol()");
        device.set_vol(original).expect("set_vol()");
        let after = device.get_vol().expect("get_vol() after set");
        assert!(
            original.abs_diff(after) <= 1,
            "volume changed unexpectedly: {original} -> {after}"
        );
    }

    /// Setting the mute state to its current value must not change it.
    #[cfg(target_os = "macos")]
    #[test]
    fn set_mute_roundtrip() {
        let device = AudioDevice::default().expect("default()");
        let original = device.is_mute().expect("is_mute()");
        device.set_mute(original).expect("set_mute()");
        let after = device.is_mute().expect("is_mute() after set");
        assert_eq!(original, after, "mute state changed unexpectedly");
    }

    // ── Windows integration tests ────────────────────────────────────────────
    //
    // These exercise WASAPI on Windows CI runners that have a virtual audio
    // device installed.  Run with `--test-threads=1` to avoid races.

    /// A device ID that is guaranteed not to match any real WASAPI endpoint.
    #[cfg(target_os = "windows")]
    const BOGUS_ID: &str = "volumecontrol-test-nonexistent-{00000000-0000-0000-0000-000000000000}";

    /// A device name that is guaranteed not to match any real audio device.
    #[cfg(target_os = "windows")]
    const BOGUS_NAME: &str = "zzz-volumecontrol-test-nonexistent-device-name";

    /// The default device must be resolvable when a virtual audio device is
    /// present.
    #[cfg(target_os = "windows")]
    #[test]
    fn default_returns_ok() {
        let result = AudioDevice::default();
        assert!(result.is_ok(), "expected Ok, got {result:?}");
    }

    /// `list()` must return at least one device with non-empty id and name.
    #[cfg(target_os = "windows")]
    #[test]
    fn list_returns_nonempty() {
        let devices = AudioDevice::list().expect("list()");
        assert!(
            !devices.is_empty(),
            "expected at least one audio device from list()"
        );
        for (id, name) in &devices {
            assert!(!id.is_empty(), "device id must not be empty");
            assert!(!name.is_empty(), "device name must not be empty");
        }
    }

    /// Looking up a device by id obtained from `list()` must succeed.
    #[cfg(target_os = "windows")]
    #[test]
    fn from_id_valid_id_returns_ok() {
        let devices = AudioDevice::list().expect("list()");
        let (id, _name) = devices.first().expect("at least one device in list");
        let found = AudioDevice::from_id(id);
        assert!(
            found.is_ok(),
            "from_id with a valid id should succeed, got {found:?}"
        );
    }

    /// A bogus device id must return `DeviceNotFound` or `InitializationFailed`.
    #[cfg(target_os = "windows")]
    #[test]
    fn from_id_bogus_returns_not_found() {
        let result = AudioDevice::from_id(BOGUS_ID);
        assert!(
            matches!(
                result,
                Err(AudioError::DeviceNotFound | AudioError::InitializationFailed(_))
            ),
            "expected DeviceNotFound or InitializationFailed, got {result:?}"
        );
    }

    /// A partial description substring of a listed device must match.
    #[cfg(target_os = "windows")]
    #[test]
    fn from_name_partial_match_returns_ok() {
        let devices = AudioDevice::list().expect("list()");
        let (_id, name) = devices.first().expect("at least one device in list");
        let partial: String = name.chars().take(3).collect();
        let found = AudioDevice::from_name(&partial);
        assert!(
            found.is_ok(),
            "from_name with partial match '{partial}' should succeed"
        );
    }

    /// A bogus device name must return `DeviceNotFound` or `InitializationFailed`.
    #[cfg(target_os = "windows")]
    #[test]
    fn from_name_bogus_returns_not_found() {
        let result = AudioDevice::from_name(BOGUS_NAME);
        assert!(
            matches!(
                result,
                Err(AudioError::DeviceNotFound | AudioError::InitializationFailed(_))
            ),
            "expected DeviceNotFound or InitializationFailed, got {result:?}"
        );
    }

    /// The reported volume must always be within the valid `0..=100` range.
    #[cfg(target_os = "windows")]
    #[test]
    fn get_vol_returns_valid_range() {
        let device = AudioDevice::default().expect("default()");
        let vol = device.get_vol().expect("get_vol()");
        assert!(vol <= 100, "volume must be in 0..=100, got {vol}");
    }

    /// Setting the volume to a different value must be reflected when read back.
    ///
    /// The original volume is restored so that other tests are not affected.
    /// Run with `--test-threads=1` to avoid races.
    #[cfg(target_os = "windows")]
    #[test]
    fn set_vol_changes_volume() {
        let device = AudioDevice::default().expect("default()");
        let original = device.get_vol().expect("get_vol()");
        let target: u8 = if original >= 50 { 30 } else { 70 };
        device.set_vol(target).expect("set_vol()");
        let after = device.get_vol().expect("get_vol() after set");
        assert!(
            after.abs_diff(target) <= 1,
            "expected volume near {target}, got {after}"
        );
        device.set_vol(original).expect("restore original volume");
    }

    /// Toggling the mute state must be reflected when read back.
    ///
    /// The original mute state is restored so that other tests are not affected.
    /// Run with `--test-threads=1` to avoid races.
    #[cfg(target_os = "windows")]
    #[test]
    fn set_mute_changes_mute_state() {
        let device = AudioDevice::default().expect("default()");
        let original = device.is_mute().expect("is_mute()");
        let target = !original;
        device.set_mute(target).expect("set_mute()");
        let after = device.is_mute().expect("is_mute() after set");
        assert_eq!(after, target, "mute state should be {target}, got {after}");
        device
            .set_mute(original)
            .expect("restore original mute state");
    }
}
