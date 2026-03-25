use volumecontrol_core::{AudioDevice as AudioDeviceTrait, AudioError};

/// Represents a CoreAudio audio output device (macOS).
///
/// # Feature flags
///
/// Real CoreAudio integration requires the `coreaudio` feature and must be
/// built for a macOS target.  Without the feature every method returns
/// [`AudioError::Unsupported`].
#[derive(Debug)]
pub struct AudioDevice {
    /// CoreAudio `AudioObjectID` serialized as a string.
    // Fields are populated by the real implementation; unused in stubs.
    #[allow(dead_code)]
    id: String,
    /// Human-readable device name (`kAudioObjectPropertyName`).
    #[allow(dead_code)]
    name: String,
}

impl AudioDeviceTrait for AudioDevice {
    fn default() -> Result<Self, AudioError> {
        #[cfg(feature = "coreaudio")]
        {
            // TODO: query kAudioHardwarePropertyDefaultOutputDevice via
            //       AudioObjectGetPropertyData using objc2-core-audio.
            todo!("CoreAudio default device lookup not yet implemented")
        }
        #[cfg(not(feature = "coreaudio"))]
        Err(AudioError::Unsupported)
    }

    fn from_id(id: &str) -> Result<Self, AudioError> {
        #[cfg(feature = "coreaudio")]
        {
            let _ = id;
            // TODO: parse AudioObjectID from string and verify it exists
            todo!("CoreAudio device lookup by id not yet implemented")
        }
        #[cfg(not(feature = "coreaudio"))]
        {
            let _ = id;
            Err(AudioError::Unsupported)
        }
    }

    fn from_name(name: &str) -> Result<Self, AudioError> {
        #[cfg(feature = "coreaudio")]
        {
            let _ = name;
            // TODO: enumerate devices and match by kAudioObjectPropertyName
            todo!("CoreAudio device lookup by name not yet implemented")
        }
        #[cfg(not(feature = "coreaudio"))]
        {
            let _ = name;
            Err(AudioError::Unsupported)
        }
    }

    fn list() -> Result<Vec<(String, String)>, AudioError> {
        #[cfg(feature = "coreaudio")]
        {
            // TODO: query kAudioHardwarePropertyDevices and collect IDs/names
            todo!("CoreAudio device listing not yet implemented")
        }
        #[cfg(not(feature = "coreaudio"))]
        Err(AudioError::Unsupported)
    }

    fn get_vol(&self) -> Result<u8, AudioError> {
        #[cfg(feature = "coreaudio")]
        {
            // TODO: read kAudioHardwareServiceDeviceProperty_VirtualMainVolume
            todo!("CoreAudio get_vol not yet implemented")
        }
        #[cfg(not(feature = "coreaudio"))]
        Err(AudioError::Unsupported)
    }

    fn set_vol(&self, vol: u8) -> Result<(), AudioError> {
        #[cfg(feature = "coreaudio")]
        {
            let _ = vol;
            // TODO: write kAudioHardwareServiceDeviceProperty_VirtualMainVolume
            todo!("CoreAudio set_vol not yet implemented")
        }
        #[cfg(not(feature = "coreaudio"))]
        {
            let _ = vol;
            Err(AudioError::Unsupported)
        }
    }

    fn is_mute(&self) -> Result<bool, AudioError> {
        #[cfg(feature = "coreaudio")]
        {
            // TODO: read kAudioDevicePropertyMute
            todo!("CoreAudio is_mute not yet implemented")
        }
        #[cfg(not(feature = "coreaudio"))]
        Err(AudioError::Unsupported)
    }

    fn set_mute(&self, muted: bool) -> Result<(), AudioError> {
        #[cfg(feature = "coreaudio")]
        {
            let _ = muted;
            // TODO: write kAudioDevicePropertyMute
            todo!("CoreAudio set_mute not yet implemented")
        }
        #[cfg(not(feature = "coreaudio"))]
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
        #[cfg(not(feature = "coreaudio"))]
        assert!(matches!(result.unwrap_err(), AudioError::Unsupported));
    }

    #[test]
    fn from_id_returns_unsupported_without_feature() {
        let result = AudioDevice::from_id("test-id");
        assert!(result.is_err());
        #[cfg(not(feature = "coreaudio"))]
        assert!(matches!(result.unwrap_err(), AudioError::Unsupported));
    }

    #[test]
    fn from_name_returns_unsupported_without_feature() {
        let result = AudioDevice::from_name("test-name");
        assert!(result.is_err());
        #[cfg(not(feature = "coreaudio"))]
        assert!(matches!(result.unwrap_err(), AudioError::Unsupported));
    }

    #[test]
    fn list_returns_unsupported_without_feature() {
        let result = AudioDevice::list();
        assert!(result.is_err());
        #[cfg(not(feature = "coreaudio"))]
        assert!(matches!(result.unwrap_err(), AudioError::Unsupported));
    }
}
