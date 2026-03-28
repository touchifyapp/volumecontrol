import { describe, it, expect, beforeEach, afterEach } from 'vitest';
import { AudioDevice, DeviceInfo } from '../index.js';

// A bogus device id guaranteed not to match any real device.
const BOGUS_ID = '__nonexistent_sink_xyz__';

// A bogus device name guaranteed not to match any real audio device.
const BOGUS_NAME = 'zzz-volumecontrol-test-nonexistent-device-name';

describe('AudioDevice', () => {
  // ── Construction ────────────────────────────────────────────────────────────

  it('fromDefault() returns a device', () => {
    const device = AudioDevice.fromDefault();
    expect(device).toBeDefined();
    expect(device).toBeInstanceOf(AudioDevice);
  });

  it('fromDefault() device has non-empty id and name', () => {
    const device = AudioDevice.fromDefault();
    expect(device.id).toBeTruthy();
    expect(device.name).toBeTruthy();
  });

  it('list() returns at least one device', () => {
    const devices = AudioDevice.list();
    expect(devices.length).toBeGreaterThan(0);
  });

  it('list() returns devices with non-empty id and name', () => {
    const devices = AudioDevice.list();
    for (const info of devices) {
      expect(info.id).toBeTruthy();
      expect(info.name).toBeTruthy();
    }
  });

  it('fromId() with a valid id returns a device', () => {
    const devices = AudioDevice.list();
    const first = devices[0] as DeviceInfo;
    const device = AudioDevice.fromId(first.id);
    expect(device).toBeInstanceOf(AudioDevice);
  });

  it('fromId() with a nonexistent id throws', () => {
    expect(() => AudioDevice.fromId(BOGUS_ID)).toThrow();
  });

  it('fromName() with a partial name match returns a device', () => {
    const devices = AudioDevice.list();
    const first = devices[0] as DeviceInfo;
    const partial = first.name.slice(0, 3);
    const device = AudioDevice.fromName(partial);
    expect(device).toBeInstanceOf(AudioDevice);
  });

  it('fromName() with no match throws', () => {
    expect(() => AudioDevice.fromName(BOGUS_NAME)).toThrow();
  });

  // ── Display format ──────────────────────────────────────────────────────────

  it('toString() follows "name (id)" format', () => {
    const device = AudioDevice.fromDefault();
    const str = device.toString();
    expect(str).toContain(device.name);
    expect(str).toContain(device.id);
    expect(str).toBe(`${device.name} (${device.id})`);
  });

  // ── Volume ──────────────────────────────────────────────────────────────────

  it('getVol() returns a value in 0..=100', () => {
    const device = AudioDevice.fromDefault();
    const vol = device.getVol();
    expect(vol).toBeGreaterThanOrEqual(0);
    expect(vol).toBeLessThanOrEqual(100);
  });

  it('setVol() changes the volume', () => {
    const device = AudioDevice.fromDefault();
    const original = device.getVol();
    const target = original >= 50 ? 30 : 70;
    device.setVol(target);
    const after = device.getVol();
    // Allow ±1 rounding error due to floating-point ↔ integer conversion.
    expect(Math.abs(after - target)).toBeLessThanOrEqual(1);
    // Restore original volume so other tests are not affected.
    device.setVol(original);
  });

  it('setVol() clamps values above 100', () => {
    const device = AudioDevice.fromDefault();
    const original = device.getVol();
    device.setVol(999);
    const after = device.getVol();
    expect(after).toBeLessThanOrEqual(100);
    device.setVol(original);
  });

  // ── Mute ────────────────────────────────────────────────────────────────────

  it('setMute() toggles the mute state', () => {
    const device = AudioDevice.fromDefault();
    const original = device.isMute();
    const target = !original;
    device.setMute(target);
    const after = device.isMute();
    expect(after).toBe(target);
    // Restore original mute state so other tests are not affected.
    device.setMute(original);
  });
});
