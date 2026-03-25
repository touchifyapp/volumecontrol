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
    // Fields are populated by the real implementation; unused in stubs.
    #[allow(dead_code)]
    id: String,
    /// Friendly device name.
    #[allow(dead_code)]
    name: String,
}

impl AudioDeviceTrait for AudioDevice {
    fn default() -> Result<Self, AudioError> {
        #[cfg(feature = "wasapi")]
        {
            // TODO: use IMMDeviceEnumerator::GetDefaultAudioEndpoint via the
            //       `windows` crate to retrieve the default render device.
            todo!("WASAPI default device lookup not yet implemented")
        }
        #[cfg(not(feature = "wasapi"))]
        Err(AudioError::Unsupported)
    }

    fn from_id(id: &str) -> Result<Self, AudioError> {
        #[cfg(feature = "wasapi")]
        {
            let _ = id;
            // TODO: use IMMDeviceEnumerator::GetDevice
            todo!("WASAPI device lookup by id not yet implemented")
        }
        #[cfg(not(feature = "wasapi"))]
        {
            let _ = id;
            Err(AudioError::Unsupported)
        }
    }

    fn from_name(name: &str) -> Result<Self, AudioError> {
        #[cfg(feature = "wasapi")]
        {
            let _ = name;
            // TODO: enumerate endpoints and match by friendly name
            todo!("WASAPI device lookup by name not yet implemented")
        }
        #[cfg(not(feature = "wasapi"))]
        {
            let _ = name;
            Err(AudioError::Unsupported)
        }
    }

    fn list() -> Result<Vec<(String, String)>, AudioError> {
        #[cfg(feature = "wasapi")]
        {
            // TODO: use IMMDeviceEnumerator::EnumAudioEndpoints
            todo!("WASAPI device listing not yet implemented")
        }
        #[cfg(not(feature = "wasapi"))]
        Err(AudioError::Unsupported)
    }

    fn get_vol(&self) -> Result<u8, AudioError> {
        #[cfg(feature = "wasapi")]
        {
            // TODO: use IAudioEndpointVolume::GetMasterVolumeLevelScalar
            todo!("WASAPI get_vol not yet implemented")
        }
        #[cfg(not(feature = "wasapi"))]
        Err(AudioError::Unsupported)
    }

    fn set_vol(&self, vol: u8) -> Result<(), AudioError> {
        #[cfg(feature = "wasapi")]
        {
            let _ = vol;
            // TODO: use IAudioEndpointVolume::SetMasterVolumeLevelScalar
            todo!("WASAPI set_vol not yet implemented")
        }
        #[cfg(not(feature = "wasapi"))]
        {
            let _ = vol;
            Err(AudioError::Unsupported)
        }
    }

    fn is_mute(&self) -> Result<bool, AudioError> {
        #[cfg(feature = "wasapi")]
        {
            // TODO: use IAudioEndpointVolume::GetMute
            todo!("WASAPI is_mute not yet implemented")
        }
        #[cfg(not(feature = "wasapi"))]
        Err(AudioError::Unsupported)
    }

    fn set_mute(&self, muted: bool) -> Result<(), AudioError> {
        #[cfg(feature = "wasapi")]
        {
            let _ = muted;
            // TODO: use IAudioEndpointVolume::SetMute
            todo!("WASAPI set_mute not yet implemented")
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

    #[test]
    fn default_returns_unsupported_without_feature() {
        let result = AudioDevice::default();
        assert!(result.is_err());
        #[cfg(not(feature = "wasapi"))]
        assert!(matches!(result.unwrap_err(), AudioError::Unsupported));
    }

    #[test]
    fn from_id_returns_unsupported_without_feature() {
        let result = AudioDevice::from_id("test-id");
        assert!(result.is_err());
        #[cfg(not(feature = "wasapi"))]
        assert!(matches!(result.unwrap_err(), AudioError::Unsupported));
    }

    #[test]
    fn from_name_returns_unsupported_without_feature() {
        let result = AudioDevice::from_name("test-name");
        assert!(result.is_err());
        #[cfg(not(feature = "wasapi"))]
        assert!(matches!(result.unwrap_err(), AudioError::Unsupported));
    }

    #[test]
    fn list_returns_unsupported_without_feature() {
        let result = AudioDevice::list();
        assert!(result.is_err());
        #[cfg(not(feature = "wasapi"))]
        assert!(matches!(result.unwrap_err(), AudioError::Unsupported));
    }
}
