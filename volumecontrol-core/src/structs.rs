use std::fmt;

/// Metadata for an available audio device returned by [`crate::AudioDevice::list`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeviceInfo {
    /// Unique platform-specific device identifier.
    ///
    /// This is the same opaque string that [`crate::AudioDevice::from_id`]
    /// accepts as its argument.  It is guaranteed to be non-empty.
    pub id: String,
    /// Human-readable device name.
    ///
    /// This is the same string that [`crate::AudioDevice::from_name`] uses for
    /// substring matching.  It is guaranteed to be non-empty.
    pub name: String,
}

impl fmt::Display for DeviceInfo {
    /// Formats the device as `"name (id)"`.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.name, self.id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_format_is_name_paren_id() {
        let info = DeviceInfo {
            id: "alsa_output.pci-0000_00_1b.0.analog-stereo".to_string(),
            name: "Built-in Audio Analog Stereo".to_string(),
        };
        assert_eq!(
            info.to_string(),
            "Built-in Audio Analog Stereo (alsa_output.pci-0000_00_1b.0.analog-stereo)"
        );
    }
}
