#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub fn setup() {
    unsafe {
        LogiLedInit();
        // LogiLedSetTargetDevice(LOGI_DEVICETYPE_RGB as i32);
        LogiLedSaveCurrentLighting();
    }
}

pub fn exit() {
    unsafe {
        LogiLedRestoreLighting();
        LogiLedShutdown();
    }
}

fn to_percentage(channel: i32) -> i32 {
    (i32::from(channel) * 100 + 127) / 255
}

pub fn set_lighting(red: i32, green: i32, blue: i32) {
    unsafe {
        LogiLedSetLighting(
            to_percentage(red),
            to_percentage(green),
            to_percentage(blue),
        );
    }
}

pub fn set_lighting_for_keys(red: i32, green: i32, blue: i32, keys: &[i32]) {
    unsafe {
        for key in keys {
            LogiLedSetLightingForKeyWithKeyName(
                *key,
                to_percentage(red),
                to_percentage(green),
                to_percentage(blue),
            );
        }
    }
}
