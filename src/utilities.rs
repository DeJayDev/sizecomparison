use std::mem;

use windows::Win32::Graphics::Gdi::{EnumDisplayDevicesA, DISPLAY_DEVICEA, DISPLAY_DEVICE_ACTIVE};

pub fn list_devices() -> Vec<String> {
    // https://github.com/horyu/rust-win32api-samples/blob/bf1591eac268480ff98ffaeb4c9cbddee14704c5/show-display-settings/src/main.rs#L3
    let mut res: Vec<String> = vec![];

    unsafe {
        for i in 0.. {
            let mut device: DISPLAY_DEVICEA = mem::zeroed();
            device.cb = mem::size_of::<DISPLAY_DEVICEA>() as u32;
            if !EnumDisplayDevicesA(None, i, &mut device as *mut _, 1).as_bool() {
                break;
            }

            if device.StateFlags & DISPLAY_DEVICE_ACTIVE != 0 {
                let p = device.DeviceName.as_ptr() as *const i8;
                let s = std::ffi::CStr::from_ptr(p).to_str().unwrap().to_owned();
                res.push(s);
            }
        }

        return res;
    }
}
