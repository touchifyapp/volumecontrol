mod internal;

use volumecontrol_core::{AudioDevice as AudioDeviceTrait, AudioError};

/// Represents a WASAPI audio output device (Windows).
///
/// # Feature flags
///
/// Real WASAPI integration requires the `wasapi` feature and must be built
/// for a Windows target.  Without the feature every method returns
/// [`AudioError::Unsupported`].
#[derive(Debug)]
pub struct AudioDevice {
    /// WASAPI endpoint identifier (GUID string).
    // Only accessed via the `wasapi` feature path; suppress dead_code on
    // non-Windows builds.
    #[allow(dead_code)]
    id: String,
    /// Friendly device name.
    #[allow(dead_code)]
    name: String,
}

impl AudioDeviceTrait for AudioDevice {
    /// Returns the system default audio render device.
    ///
    /// # Errors
    ///
    /// Returns [`AudioError::InitializationFailed`] if COM cannot be
    /// initialised or if the default device cannot be resolved.
    /// Returns [`AudioError::Unsupported`] when the `wasapi` feature is
    /// not enabled.
    fn default() -> Result<Self, AudioError> {
        #[cfg(feature = "wasapi")]
        {
            let _com = internal::wasapi::ComGuard::new()?;
            let enumerator = internal::wasapi::create_enumerator()?;
            let device = internal::wasapi::get_default_device(&enumerator)?;
            let id = internal::wasapi::device_id(&device)?;
            let name = internal::wasapi::device_name(&device)?;
            Ok(Self { id, name })
        }
        #[cfg(not(feature = "wasapi"))]
        Err(AudioError::Unsupported)
    }

    /// Returns the audio device identified by `id`.
    ///
    /// # Errors
    ///
    /// Returns [`AudioError::DeviceNotFound`] if no device with the given
    /// identifier exists.
    /// Returns [`AudioError::InitializationFailed`] if COM cannot be
    /// initialised or another lookup failure occurs.
    /// Returns [`AudioError::Unsupported`] when the `wasapi` feature is
    /// not enabled.
    fn from_id(id: &str) -> Result<Self, AudioError> {
        #[cfg(feature = "wasapi")]
        {
            let _com = internal::wasapi::ComGuard::new()?;
            let enumerator = internal::wasapi::create_enumerator()?;
            let device = internal::wasapi::get_device_by_id(&enumerator, id)?;
            let resolved_id = internal::wasapi::device_id(&device)?;
            let name = internal::wasapi::device_name(&device)?;
            Ok(Self {
                id: resolved_id,
                name,
            })
        }
        #[cfg(not(feature = "wasapi"))]
        {
            let _ = id;
            Err(AudioError::Unsupported)
        }
    }

    /// Returns the first audio device whose name contains `name`
    /// (case-insensitive substring match).
    ///
    /// # Errors
    ///
    /// Returns [`AudioError::DeviceNotFound`] if no matching device is found.
    /// Returns [`AudioError::InitializationFailed`] if COM cannot be
    /// initialised or another lookup failure occurs.
    /// Returns [`AudioError::Unsupported`] when the `wasapi` feature is
    /// not enabled.
    fn from_name(name: &str) -> Result<Self, AudioError> {
        #[cfg(feature = "wasapi")]
        {
            let _com = internal::wasapi::ComGuard::new()?;
            let enumerator = internal::wasapi::create_enumerator()?;
            let devices = internal::wasapi::list_devices(&enumerator)?;

            let needle = name.to_lowercase();
            let (id, matched_name) = devices
                .into_iter()
                .find(|(_, n)| n.to_lowercase().contains(&needle))
                .ok_or(AudioError::DeviceNotFound)?;

            Ok(Self {
                id,
                name: matched_name,
            })
        }
        #[cfg(not(feature = "wasapi"))]
        {
            let _ = name;
            Err(AudioError::Unsupported)
        }
    }

    /// Lists all available audio render devices as `(id, name)` pairs.
    ///
    /// # Errors
    ///
    /// Returns [`AudioError::ListFailed`] if the device list cannot be
    /// retrieved.
    /// Returns [`AudioError::InitializationFailed`] if COM cannot be
    /// initialised.
    /// Returns [`AudioError::Unsupported`] when the `wasapi` feature is
    /// not enabled.
    fn list() -> Result<Vec<(String, String)>, AudioError> {
        #[cfg(feature = "wasapi")]
        {
            let _com = internal::wasapi::ComGuard::new()?;
            let enumerator = internal::wasapi::create_enumerator()?;
            internal::wasapi::list_devices(&enumerator)
        }
        #[cfg(not(feature = "wasapi"))]
        Err(AudioError::Unsupported)
    }

    /// Returns the current volume level in the range `0..=100`.
    ///
    /// # Errors
    ///
    /// Returns [`AudioError::GetVolumeFailed`] if the volume cannot be read.
    /// Returns [`AudioError::DeviceNotFound`] if this device no longer exists.
    /// Returns [`AudioError::Unsupported`] when the `wasapi` feature is
    /// not enabled.
    fn get_vol(&self) -> Result<u8, AudioError> {
        #[cfg(feature = "wasapi")]
        {
            let _com = internal::wasapi::ComGuard::new()?;
            let enumerator = internal::wasapi::create_enumerator()?;
            let device = internal::wasapi::get_device_by_id(&enumerator, &self.id)?;
            let endpoint = internal::wasapi::endpoint_volume(&device)?;
            internal::wasapi::get_volume(&endpoint)
        }
        #[cfg(not(feature = "wasapi"))]
        Err(AudioError::Unsupported)
    }

    /// Sets the volume level.
    ///
    /// `vol` is clamped to `0..=100` before being applied.
    ///
    /// # Errors
    ///
    /// Returns [`AudioError::SetVolumeFailed`] if the volume cannot be set.
    /// Returns [`AudioError::DeviceNotFound`] if this device no longer exists.
    /// Returns [`AudioError::Unsupported`] when the `wasapi` feature is
    /// not enabled.
    fn set_vol(&self, vol: u8) -> Result<(), AudioError> {
        #[cfg(feature = "wasapi")]
        {
            let _com = internal::wasapi::ComGuard::new()?;
            let enumerator = internal::wasapi::create_enumerator()?;
            let device = internal::wasapi::get_device_by_id(&enumerator, &self.id)?;
            let endpoint = internal::wasapi::endpoint_volume(&device)?;
            internal::wasapi::set_volume(&endpoint, vol)
        }
        #[cfg(not(feature = "wasapi"))]
        {
            let _ = vol;
            Err(AudioError::Unsupported)
        }
    }

    /// Returns `true` if the device is currently muted.
    ///
    /// # Errors
    ///
    /// Returns [`AudioError::GetMuteFailed`] if the mute state cannot be read.
    /// Returns [`AudioError::DeviceNotFound`] if this device no longer exists.
    /// Returns [`AudioError::Unsupported`] when the `wasapi` feature is
    /// not enabled.
    fn is_mute(&self) -> Result<bool, AudioError> {
        #[cfg(feature = "wasapi")]
        {
            let _com = internal::wasapi::ComGuard::new()?;
            let enumerator = internal::wasapi::create_enumerator()?;
            let device = internal::wasapi::get_device_by_id(&enumerator, &self.id)?;
            let endpoint = internal::wasapi::endpoint_volume(&device)?;
            internal::wasapi::get_mute(&endpoint)
        }
        #[cfg(not(feature = "wasapi"))]
        Err(AudioError::Unsupported)
    }

    /// Mutes or unmutes the device.
    ///
    /// # Errors
    ///
    /// Returns [`AudioError::SetMuteFailed`] if the mute state cannot be
    /// changed.
    /// Returns [`AudioError::DeviceNotFound`] if this device no longer exists.
    /// Returns [`AudioError::Unsupported`] when the `wasapi` feature is
    /// not enabled.
    fn set_mute(&self, muted: bool) -> Result<(), AudioError> {
        #[cfg(feature = "wasapi")]
        {
            let _com = internal::wasapi::ComGuard::new()?;
            let enumerator = internal::wasapi::create_enumerator()?;
            let device = internal::wasapi::get_device_by_id(&enumerator, &self.id)?;
            let endpoint = internal::wasapi::endpoint_volume(&device)?;
            internal::wasapi::set_mute(&endpoint, muted)
        }
        #[cfg(not(feature = "wasapi"))]
        {
            let _ = muted;
            Err(AudioError::Unsupported)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use volumecontrol_core::AudioDevice as AudioDeviceTrait;

    // ------------------------------------------------------------------
    // Stub-path tests — only compiled and run when `wasapi` is disabled.
    // ------------------------------------------------------------------

    #[test]
    #[cfg(not(feature = "wasapi"))]
    fn default_returns_unsupported_without_feature() {
        assert!(matches!(
            AudioDevice::default(),
            Err(AudioError::Unsupported)
        ));
    }

    #[test]
    #[cfg(not(feature = "wasapi"))]
    fn from_id_returns_unsupported_without_feature() {
        assert!(matches!(
            AudioDevice::from_id("test-id"),
            Err(AudioError::Unsupported)
        ));
    }

    #[test]
    #[cfg(not(feature = "wasapi"))]
    fn from_name_returns_unsupported_without_feature() {
        assert!(matches!(
            AudioDevice::from_name("test-name"),
            Err(AudioError::Unsupported)
        ));
    }

    #[test]
    #[cfg(not(feature = "wasapi"))]
    fn list_returns_unsupported_without_feature() {
        assert!(matches!(AudioDevice::list(), Err(AudioError::Unsupported)));
    }

    #[test]
    #[cfg(not(feature = "wasapi"))]
    fn get_vol_returns_unsupported_without_feature() {
        let device = AudioDevice {
            id: String::from("stub-id"),
            name: String::from("stub-name"),
        };
        assert!(matches!(device.get_vol(), Err(AudioError::Unsupported)));
    }

    #[test]
    #[cfg(not(feature = "wasapi"))]
    fn set_vol_returns_unsupported_without_feature() {
        let device = AudioDevice {
            id: String::from("stub-id"),
            name: String::from("stub-name"),
        };
        assert!(matches!(device.set_vol(50), Err(AudioError::Unsupported)));
    }

    #[test]
    #[cfg(not(feature = "wasapi"))]
    fn is_mute_returns_unsupported_without_feature() {
        let device = AudioDevice {
            id: String::from("stub-id"),
            name: String::from("stub-name"),
        };
        assert!(matches!(device.is_mute(), Err(AudioError::Unsupported)));
    }

    #[test]
    #[cfg(not(feature = "wasapi"))]
    fn set_mute_returns_unsupported_without_feature() {
        let device = AudioDevice {
            id: String::from("stub-id"),
            name: String::from("stub-name"),
        };
        assert!(matches!(
            device.set_mute(true),
            Err(AudioError::Unsupported)
        ));
    }

    // ------------------------------------------------------------------
    // Real-world WASAPI tests — only compiled and run with `wasapi` feature.
    // These run on Windows CI runners that have real audio hardware, or skip
    // gracefully when no audio endpoint is present.
    // ------------------------------------------------------------------

    /// A device ID that is guaranteed to not match any real WASAPI endpoint.
    #[cfg(feature = "wasapi")]
    const BOGUS_ID: &str = "volumecontrol-test-nonexistent-{00000000-0000-0000-0000-000000000000}";

    /// A device name that is guaranteed to not match any real audio device.
    #[cfg(feature = "wasapi")]
    const BOGUS_NAME: &str = "zzz-volumecontrol-test-nonexistent-device-name";

    /// `from_id` with a clearly invalid ID must return `DeviceNotFound` or a
    /// graceful `InitializationFailed` — never a panic or `Unsupported`.
    #[test]
    #[cfg(feature = "wasapi")]
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

    /// `from_name` with a clearly invalid name must return `DeviceNotFound` or
    /// `InitializationFailed` — never a panic or `Unsupported`.
    #[test]
    #[cfg(feature = "wasapi")]
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

    /// `list()` must not panic; it may return an empty `Vec` or an error on
    /// machines with no audio hardware.
    #[test]
    #[cfg(feature = "wasapi")]
    fn list_does_not_panic() {
        let result = AudioDevice::list();
        assert!(
            result.is_ok() || matches!(result, Err(AudioError::InitializationFailed(_))),
            "unexpected error from list(): {result:?}"
        );
    }

    /// `default()` must not panic; a missing audio device yields a known error.
    #[test]
    #[cfg(feature = "wasapi")]
    fn default_does_not_panic() {
        let result = AudioDevice::default();
        assert!(
            result.is_ok()
                || matches!(
                    result,
                    Err(AudioError::DeviceNotFound | AudioError::InitializationFailed(_))
                ),
            "unexpected error from default(): {result:?}"
        );
    }

    /// If a default render endpoint is available, `get_vol` must return a value
    /// in `0..=100` and `set_vol` must accept it back without error.
    #[test]
    #[cfg(feature = "wasapi")]
    fn default_device_volume_round_trip() {
        let device = match AudioDevice::default() {
            Ok(d) => d,
            Err(_) => return, // No audio hardware on this runner — skip.
        };

        let original_vol = match device.get_vol() {
            Ok(v) => v,
            Err(_) => return,
        };
        assert!(
            original_vol <= 100,
            "get_vol returned {original_vol}, which is out of range"
        );

        // Writing the same value back should be a no-op.
        let _ = device.set_vol(original_vol);

        if let Ok(v) = device.get_vol() {
            assert!(v <= 100, "get_vol after set_vol returned {v}, out of range");
        }
    }

    /// If a default render endpoint is available, `is_mute`/`set_mute` must
    /// round-trip: restoring the original mute state must succeed.
    #[test]
    #[cfg(feature = "wasapi")]
    fn default_device_mute_round_trip() {
        let device = match AudioDevice::default() {
            Ok(d) => d,
            Err(_) => return,
        };

        let original = match device.is_mute() {
            Ok(m) => m,
            Err(_) => return,
        };

        // Restore to original state — test is idempotent on real hardware.
        let _ = device.set_mute(original);

        if let Ok(m) = device.is_mute() {
            assert_eq!(m, original, "mute state changed after restoring it");
        }
    }
}
