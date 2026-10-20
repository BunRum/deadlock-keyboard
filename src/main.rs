#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};
use std::thread;
use std::time::Duration;

use deadlock_keyboard::{deadlock, lighting};

fn onHeroFound(hero: &str) {
    println!("Current hero: {hero}");

    let hero_color_data = deadlock::utility::get_hero_color(hero);

    println!("Hero color: {:?}", hero_color_data);

    let red = hero_color_data.0 as i32;
    let green = hero_color_data.1 as i32;
    let blue = hero_color_data.2 as i32;

    println!("Hero color i32: {:?} {:?} {:?}", red, green, blue);

    lighting::logitech::set_lighting(red, green, blue);
    lighting::logitech::set_lighting_for_keys(
        0,
        0,
        0,
        &[
            LogiLed_KeyName_W,
            LogiLed_KeyName_A,
            LogiLed_KeyName_S,
            LogiLed_KeyName_D,
        ],
    );
    println!("Set WASD lighting.")
}

fn main() {
    let log_path = deadlock::steam::find_console_log(None);
    let mut last_hero = None;

    let running = Arc::new(AtomicBool::new(true));
    let signal_flag = Arc::clone(&running);

    ctrlc::set_handler(move || {
        signal_flag.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl+C handler");

    println!("Application running. Press Ctrl+C to stop.");

    lighting::logitech::setup();

    while running.load(Ordering::SeqCst) {
        if let Some(hero) = deadlock::utility::get_current_hero(&log_path) {
            if last_hero.as_ref() != Some(&hero) {
                onHeroFound(&hero);
                last_hero = Some(hero);
            }
        }

        thread::sleep(Duration::from_secs(1));
    }

    println!("Exiting application...");

    lighting::logitech::exit();
}
