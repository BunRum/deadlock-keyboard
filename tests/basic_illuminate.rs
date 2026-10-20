use std::{thread::sleep, time::Duration};

use deadlock_keyboard::lighting::logitech::{LogiLed_KeyName, LogiLed_KeyName_A, LogiLed_KeyName_D, LogiLed_KeyName_S, LogiLed_KeyName_W, exit, set_lighting, set_lighting_for_keys, setup};

#[test]
fn illuminate_test() {
    setup();
    let red = 255;
    let green = 255;
    let blue = 255;
    let start_time = std::time::Instant::now();
    let mut elapsed_time;
    loop {
        elapsed_time = start_time.elapsed().as_secs();
        if elapsed_time == 5 {
            println!("5 seconds have passed, exiting loop.");
            break;
        }
        println!("Elapsed time: {} seconds, Red value: {}", elapsed_time, red);
        set_lighting(red, green, blue);
    }
    exit();
}

#[test]
fn keys_test() {
    setup();
    set_lighting_for_keys(255, 255, 255, &[
        LogiLed_KeyName_W,   
        LogiLed_KeyName_A,
        LogiLed_KeyName_S,
        LogiLed_KeyName_D,
    ]);
    sleep(Duration::from_secs(10));
    exit();
}