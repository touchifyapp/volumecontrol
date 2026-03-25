//! Cross-platform crate to control system audio volume.
//!
//! This crate re-exports the platform-appropriate [`AudioDevice`] implementation
//! and the [`AudioDeviceTrait`] trait so callers only need to import from this
//! one crate.  The underlying backend is selected automatically at compile time
//! based on the target operating system:
//!
//! | Platform | Backend crate              | Feature flag to enable real impl |
//! |----------|----------------------------|----------------------------------|
//! | Linux    | `volumecontrol-linux`      | `pulseaudio`                     |
//! | Windows  | `volumecontrol-windows`    | `wasapi`                         |
//! | macOS    | `volumecontrol-macos`      | `coreaudio`                      |
//!
//! # Quick-start
//!
//! ```no_run
//! use volumecontrol::{AudioDevice, AudioDeviceTrait};
//!
//! fn main() -> Result<(), volumecontrol::AudioError> {
//!     let device = AudioDevice::default()?;
//!     let vol = device.get_vol()?;
//!     println!("Current volume: {vol}%");
//!     Ok(())
//! }
//! ```
//!
//! Alternatively, import everything through the [`prelude`] module:
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

/// The [`AudioDevice`] trait re-exported for convenience.
///
/// Importing this trait brings all device-control methods (`get_vol`,
/// `set_vol`, `is_mute`, `set_mute`, …) into scope for any value whose
/// concrete type implements it.
pub use volumecontrol_core::AudioDevice as AudioDeviceTrait;

/// The unified error type for all volumecontrol operations.
pub use volumecontrol_core::AudioError;

/// The platform-specific concrete [`AudioDevice`] type.
#[cfg(target_os = "linux")]
pub use volumecontrol_linux::AudioDevice;

/// The platform-specific concrete [`AudioDevice`] type.
#[cfg(target_os = "windows")]
pub use volumecontrol_windows::AudioDevice;

/// The platform-specific concrete [`AudioDevice`] type.
#[cfg(target_os = "macos")]
pub use volumecontrol_macos::AudioDevice;

#[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
compile_error!(
    "volumecontrol does not support the current target OS. \
     Supported targets: linux, windows, macos."
);

/// A convenience prelude that re-exports every item needed for typical usage.
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
    pub use super::AudioDeviceTrait;
    pub use super::AudioError;
}

// ── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ------------------------------------------------------------------
    // Stub-path tests (no backend feature enabled).
    // These verify that every method returns `Err(AudioError::Unsupported)`
    // when the real platform backend is not compiled in.
    // ------------------------------------------------------------------

    #[cfg(all(target_os = "linux", not(feature = "pulseaudio")))]
    #[test]
    fn default_returns_unsupported_without_feature() {
        assert!(matches!(
            AudioDevice::default(),
            Err(AudioError::Unsupported)
        ));
    }

    #[cfg(all(target_os = "linux", not(feature = "pulseaudio")))]
    #[test]
    fn from_id_returns_unsupported_without_feature() {
        assert!(matches!(
            AudioDevice::from_id("stub-id"),
            Err(AudioError::Unsupported)
        ));
    }

    #[cfg(all(target_os = "linux", not(feature = "pulseaudio")))]
    #[test]
    fn from_name_returns_unsupported_without_feature() {
        assert!(matches!(
            AudioDevice::from_name("stub-name"),
            Err(AudioError::Unsupported)
        ));
    }

    #[cfg(all(target_os = "linux", not(feature = "pulseaudio")))]
    #[test]
    fn list_returns_unsupported_without_feature() {
        assert!(matches!(AudioDevice::list(), Err(AudioError::Unsupported)));
    }

    // ------------------------------------------------------------------
    // Real-world integration tests – Linux + pulseaudio feature.
    //
    // These require a running PulseAudio server with at least one sink.
    // In CI a virtual null sink is provisioned before the test suite runs.
    // Run with `--test-threads=1` to avoid races on shared audio state.
    // ------------------------------------------------------------------

    /// The default device must be resolvable when PulseAudio is running.
    #[cfg(all(target_os = "linux", feature = "pulseaudio"))]
    #[test]
    fn default_returns_ok() {
        let result = AudioDevice::default();
        assert!(result.is_ok(), "expected Ok, got {result:?}");
    }

    /// `list()` must return at least one device with non-empty id and name.
    #[cfg(all(target_os = "linux", feature = "pulseaudio"))]
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

    /// Looking up the default device by its id from `list()` must succeed.
    #[cfg(all(target_os = "linux", feature = "pulseaudio"))]
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
    #[cfg(all(target_os = "linux", feature = "pulseaudio"))]
    #[test]
    fn from_id_nonexistent_returns_not_found() {
        match AudioDevice::from_id("__nonexistent_sink_xyz__") {
            Err(AudioError::DeviceNotFound) => {}
            other => panic!("expected DeviceNotFound, got {other:?}"),
        }
    }

    /// A partial description substring of a listed device must match via `from_name`.
    #[cfg(all(target_os = "linux", feature = "pulseaudio"))]
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
    #[cfg(all(target_os = "linux", feature = "pulseaudio"))]
    #[test]
    fn from_name_no_match_returns_not_found() {
        match AudioDevice::from_name("\x00\x01\x02") {
            Err(AudioError::DeviceNotFound) => {}
            other => panic!("expected DeviceNotFound, got {other:?}"),
        }
    }

    /// The reported volume must always be within the valid `0..=100` range.
    #[cfg(all(target_os = "linux", feature = "pulseaudio"))]
    #[test]
    fn get_vol_returns_valid_range() {
        let device = AudioDevice::default().expect("default()");
        let vol = device.get_vol().expect("get_vol()");
        assert!(vol <= 100, "volume must be in 0..=100, got {vol}");
    }

    /// Setting the volume to a different value must be reflected when read back.
    ///
    /// The original volume is restored at the end of the test so that other
    /// tests are not affected.  Run with `--test-threads=1` to avoid races.
    #[cfg(all(target_os = "linux", feature = "pulseaudio"))]
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
    /// The original mute state is restored at the end of the test so that
    /// other tests are not affected.  Run with `--test-threads=1` to avoid races.
    #[cfg(all(target_os = "linux", feature = "pulseaudio"))]
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
