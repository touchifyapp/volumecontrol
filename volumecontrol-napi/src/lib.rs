//! Node.js bindings for the `volumecontrol` crate via napi-rs.
//!
//! This crate exposes the cross-platform audio volume control API to Node.js
//! as a native addon (`.node` file) built with [napi-rs](https://napi.rs).
//!
//! The public types mirror the Rust API:
//!
//! - [`JsAudioDevice`] — wraps `volumecontrol::AudioDevice` and exposes all
//!   methods as `#[napi]`-annotated functions.
//! - [`JsDeviceInfo`] — a plain data object mirroring `volumecontrol_core::DeviceInfo`.

#![deny(clippy::all)]

use std::fmt;

use napi_derive::napi;

use volumecontrol::AudioDevice;
use volumecontrol_core::AudioError;

// ── Error conversion ──────────────────────────────────────────────────────────

/// Converts an [`AudioError`] into a [`napi::Error`] so that it can be thrown
/// as a JavaScript `Error`.
fn to_napi_err(err: AudioError) -> napi::Error {
    napi::Error::from_reason(format!("{err}"))
}

// ── JsDeviceInfo ─────────────────────────────────────────────────────────────

/// Plain data object representing an available audio device.
///
/// Mirrors [`volumecontrol_core::DeviceInfo`].  Exposed to JS as a plain
/// object with `id` and `name` string properties.
#[napi(object, js_name = "DeviceInfo")]
pub struct JsDeviceInfo {
    /// Platform-specific unique identifier for the device.
    ///
    /// Matches the value returned by [`JsAudioDevice::id`] and accepted by
    /// [`JsAudioDevice::from_id`].
    pub id: String,

    /// Human-readable display name for the device.
    ///
    /// Matches the value returned by [`JsAudioDevice::name`] and used for
    /// substring matching by [`JsAudioDevice::from_name`].
    pub name: String,
}

// ── JsAudioDevice ─────────────────────────────────────────────────────────────

/// Cross-platform audio output device.
///
/// Wraps [`volumecontrol::AudioDevice`] and exposes its API to Node.js.
/// All fallible methods return `napi::Result<T>`, which causes the JS side
/// to receive a thrown `Error` on failure.
#[napi(js_name = "AudioDevice")]
pub struct JsAudioDevice {
    inner: AudioDevice,
}

#[napi]
impl JsAudioDevice {
    /// Returns the system default audio output device.
    ///
    /// # Errors
    ///
    /// Throws if the default device cannot be resolved.
    #[napi(factory)]
    pub fn from_default() -> napi::Result<Self> {
        AudioDevice::from_default()
            .map(|inner| Self { inner })
            .map_err(to_napi_err)
    }

    /// Returns the audio device identified by `id`.
    ///
    /// # Errors
    ///
    /// Throws if no device with the given identifier exists, or if the lookup
    /// fails.
    #[napi(factory)]
    pub fn from_id(id: String) -> napi::Result<Self> {
        AudioDevice::from_id(&id)
            .map(|inner| Self { inner })
            .map_err(to_napi_err)
    }

    /// Returns the first audio device whose name contains `name`.
    ///
    /// The match is a case-insensitive substring search on most platforms.
    ///
    /// # Errors
    ///
    /// Throws if no matching device is found, or if the lookup fails.
    #[napi(factory)]
    pub fn from_name(name: String) -> napi::Result<Self> {
        AudioDevice::from_name(&name)
            .map(|inner| Self { inner })
            .map_err(to_napi_err)
    }

    /// Lists all available audio devices.
    ///
    /// # Errors
    ///
    /// Throws if the device list cannot be retrieved.
    #[napi]
    pub fn list() -> napi::Result<Vec<JsDeviceInfo>> {
        AudioDevice::list()
            .map(|devices| {
                devices
                    .into_iter()
                    .map(|d| JsDeviceInfo {
                        id: d.id,
                        name: d.name,
                    })
                    .collect()
            })
            .map_err(to_napi_err)
    }

    /// Returns the unique identifier for this device.
    ///
    /// The value is the same opaque string that [`list`](Self::list) yields as
    /// `DeviceInfo.id` and that [`from_id`](Self::from_id) accepts as its
    /// argument.  It is guaranteed to be non-empty.
    #[napi(getter)]
    pub fn id(&self) -> String {
        self.inner.id().to_owned()
    }

    /// Returns the human-readable display name of this device.
    ///
    /// The value is the same string that [`list`](Self::list) yields as
    /// `DeviceInfo.name` and that [`from_name`](Self::from_name) uses for
    /// substring matching.  It is guaranteed to be non-empty.
    #[napi(getter)]
    pub fn name(&self) -> String {
        self.inner.name().to_owned()
    }

    /// Returns the current volume level in the range `0..=100`.
    ///
    /// The value is returned as `u32` for JavaScript compatibility (napi-rs
    /// does not support `u8` in `#[napi]` signatures).
    ///
    /// # Errors
    ///
    /// Throws if the volume cannot be read.
    #[napi]
    pub fn get_vol(&self) -> napi::Result<u32> {
        self.inner.get_vol().map(u32::from).map_err(to_napi_err)
    }

    /// Sets the volume level.
    ///
    /// `vol` is clamped to `0..=100` before being applied.  The parameter is
    /// `u32` for JavaScript compatibility; values above `100` are clamped to
    /// `100`.
    ///
    /// # Errors
    ///
    /// Throws if the volume cannot be set.
    #[napi]
    pub fn set_vol(&self, vol: u32) -> napi::Result<()> {
        let clamped = vol.min(100) as u8;
        self.inner.set_vol(clamped).map_err(to_napi_err)
    }

    /// Returns `true` if the device is currently muted.
    ///
    /// # Errors
    ///
    /// Throws if the mute state cannot be read.
    #[napi]
    pub fn is_mute(&self) -> napi::Result<bool> {
        self.inner.is_mute().map_err(to_napi_err)
    }

    /// Mutes or unmutes the device.
    ///
    /// # Errors
    ///
    /// Throws if the mute state cannot be changed.
    #[napi]
    pub fn set_mute(&self, muted: bool) -> napi::Result<()> {
        self.inner.set_mute(muted).map_err(to_napi_err)
    }

    /// Returns the device formatted as `"name (id)"`.
    ///
    /// Delegates to the [`fmt::Display`](std::fmt::Display) implementation of
    /// the inner [`AudioDevice`].
    #[napi(js_name = "toString")]
    pub fn js_to_string(&self) -> String {
        self.inner.to_string()
    }
}

impl fmt::Display for JsAudioDevice {
    /// Formats the device as `"name (id)"`, e.g. `"Speakers ({0.0.0.…}.{…})"`.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner.fmt(f)
    }
}
