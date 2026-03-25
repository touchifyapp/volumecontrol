use volumecontrol_core::{AudioDevice as AudioDeviceTrait, AudioError};

#[cfg(feature = "pulseaudio")]
mod pulse;

/// Represents a PulseAudio audio output device.
///
/// # Feature flags
///
/// Real PulseAudio integration requires the `pulseaudio` feature and the
/// `libpulse-dev` system package.  Without the feature every method returns
/// [`AudioError::Unsupported`].
#[derive(Debug)]
pub struct AudioDevice {
    /// PulseAudio sink name used as the unique device identifier.
    #[cfg_attr(not(feature = "pulseaudio"), allow(dead_code))]
    id: String,
    /// Human-readable sink description (stored for introspection and future use).
    #[allow(dead_code)]
    name: String,
}

impl AudioDeviceTrait for AudioDevice {
    fn default() -> Result<Self, AudioError> {
        #[cfg(feature = "pulseaudio")]
        {
            let sink_name = pulse::default_sink_name()?;
            let snap = pulse::sink_by_name(&sink_name)?;
            Ok(AudioDevice {
                id: snap.name,
                name: snap.description,
            })
        }
        #[cfg(not(feature = "pulseaudio"))]
        Err(AudioError::Unsupported)
    }

    fn from_id(id: &str) -> Result<Self, AudioError> {
        #[cfg(feature = "pulseaudio")]
        {
            let snap = pulse::sink_by_name(id)?;
            Ok(AudioDevice {
                id: snap.name,
                name: snap.description,
            })
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
            let snap = pulse::sink_matching_description(name)?;
            Ok(AudioDevice {
                id: snap.name,
                name: snap.description,
            })
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
            pulse::list_sinks()
        }
        #[cfg(not(feature = "pulseaudio"))]
        Err(AudioError::Unsupported)
    }

    fn get_vol(&self) -> Result<u8, AudioError> {
        #[cfg(feature = "pulseaudio")]
        {
            Ok(pulse::sink_by_name(&self.id)?.volume)
        }
        #[cfg(not(feature = "pulseaudio"))]
        Err(AudioError::Unsupported)
    }

    fn set_vol(&self, vol: u8) -> Result<(), AudioError> {
        #[cfg(feature = "pulseaudio")]
        {
            pulse::set_sink_volume(&self.id, vol)
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
            Ok(pulse::sink_by_name(&self.id)?.mute)
        }
        #[cfg(not(feature = "pulseaudio"))]
        Err(AudioError::Unsupported)
    }

    fn set_mute(&self, muted: bool) -> Result<(), AudioError> {
        #[cfg(feature = "pulseaudio")]
        {
            pulse::set_sink_mute(&self.id, muted)
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

    /// When the `pulseaudio` feature is disabled, every `&self` method on an
    /// `AudioDevice` must return `Err(AudioError::Unsupported)`.
    #[cfg(not(feature = "pulseaudio"))]
    #[test]
    fn self_methods_return_unsupported_without_feature() {
        // Construct a dummy device directly; the public constructors also
        // return `Unsupported` without the feature.
        let device = AudioDevice {
            id: String::new(),
            name: String::new(),
        };
        assert!(matches!(
            device.get_vol().unwrap_err(),
            AudioError::Unsupported
        ));
        assert!(matches!(
            device.set_vol(50).unwrap_err(),
            AudioError::Unsupported
        ));
        assert!(matches!(
            device.is_mute().unwrap_err(),
            AudioError::Unsupported
        ));
        assert!(matches!(
            device.set_mute(false).unwrap_err(),
            AudioError::Unsupported
        ));
    }
}
