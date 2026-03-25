use volumecontrol_core::{AudioDevice as AudioDeviceTrait, AudioError};

/// Represents a PulseAudio audio output device.
///
/// # Feature flags
///
/// Real PulseAudio integration requires the `pulseaudio` feature and the
/// `libpulse-dev` system package.  Without the feature every method returns
/// [`AudioError::Unsupported`].
#[derive(Debug)]
pub struct AudioDevice {
    /// Unique PulseAudio sink identifier.
    // Fields are populated by the real implementation; unused in stubs.
    #[allow(dead_code)]
    id: String,
    /// Human-readable sink name.
    #[allow(dead_code)]
    name: String,
}

impl AudioDeviceTrait for AudioDevice {
    fn default() -> Result<Self, AudioError> {
        #[cfg(feature = "pulseaudio")]
        {
            // TODO: use libpulse_binding to query the default sink
            todo!("PulseAudio default device lookup not yet implemented")
        }
        #[cfg(not(feature = "pulseaudio"))]
        Err(AudioError::Unsupported)
    }

    fn from_id(id: &str) -> Result<Self, AudioError> {
        #[cfg(feature = "pulseaudio")]
        {
            let _ = id;
            // TODO: use libpulse_binding to look up a sink by its name/index
            todo!("PulseAudio device lookup by id not yet implemented")
        }
        #[cfg(not(feature = "pulseaudio"))]
        {
            let _ = id;
            Err(AudioError::Unsupported)
        }
    }

    fn from_name(name: &str) -> Result<Self, AudioError> {
        #[cfg(feature = "pulseaudio")]
        {
            let _ = name;
            // TODO: use libpulse_binding to search sinks by description
            todo!("PulseAudio device lookup by name not yet implemented")
        }
        #[cfg(not(feature = "pulseaudio"))]
        {
            let _ = name;
            Err(AudioError::Unsupported)
        }
    }

    fn list() -> Result<Vec<(String, String)>, AudioError> {
        #[cfg(feature = "pulseaudio")]
        {
            // TODO: use libpulse_binding to enumerate sinks
            todo!("PulseAudio device listing not yet implemented")
        }
        #[cfg(not(feature = "pulseaudio"))]
        Err(AudioError::Unsupported)
    }

    fn get_vol(&self) -> Result<u8, AudioError> {
        #[cfg(feature = "pulseaudio")]
        {
            // TODO: query sink volume via libpulse_binding
            todo!("PulseAudio get_vol not yet implemented")
        }
        #[cfg(not(feature = "pulseaudio"))]
        Err(AudioError::Unsupported)
    }

    fn set_vol(&self, vol: u8) -> Result<(), AudioError> {
        #[cfg(feature = "pulseaudio")]
        {
            let _ = vol;
            // TODO: set sink volume via libpulse_binding
            todo!("PulseAudio set_vol not yet implemented")
        }
        #[cfg(not(feature = "pulseaudio"))]
        {
            let _ = vol;
            Err(AudioError::Unsupported)
        }
    }

    fn is_mute(&self) -> Result<bool, AudioError> {
        #[cfg(feature = "pulseaudio")]
        {
            // TODO: query sink mute state via libpulse_binding
            todo!("PulseAudio is_mute not yet implemented")
        }
        #[cfg(not(feature = "pulseaudio"))]
        Err(AudioError::Unsupported)
    }

    fn set_mute(&self, muted: bool) -> Result<(), AudioError> {
        #[cfg(feature = "pulseaudio")]
        {
            let _ = muted;
            // TODO: set sink mute state via libpulse_binding
            todo!("PulseAudio set_mute not yet implemented")
        }
        #[cfg(not(feature = "pulseaudio"))]
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
        #[cfg(not(feature = "pulseaudio"))]
        assert!(matches!(result.unwrap_err(), AudioError::Unsupported));
    }

    #[test]
    fn from_id_returns_unsupported_without_feature() {
        let result = AudioDevice::from_id("test-id");
        assert!(result.is_err());
        #[cfg(not(feature = "pulseaudio"))]
        assert!(matches!(result.unwrap_err(), AudioError::Unsupported));
    }

    #[test]
    fn from_name_returns_unsupported_without_feature() {
        let result = AudioDevice::from_name("test-name");
        assert!(result.is_err());
        #[cfg(not(feature = "pulseaudio"))]
        assert!(matches!(result.unwrap_err(), AudioError::Unsupported));
    }

    #[test]
    fn list_returns_unsupported_without_feature() {
        let result = AudioDevice::list();
        assert!(result.is_err());
        #[cfg(not(feature = "pulseaudio"))]
        assert!(matches!(result.unwrap_err(), AudioError::Unsupported));
    }
}
