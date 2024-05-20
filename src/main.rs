pub mod utilities;

use getopts::Options;
use std::{env, panic};
use utilities::list_devices;
use windows::{
    core::PCSTR,
    Win32::Graphics::Gdi::{EnumDisplaySettingsA, DEVMODEA, ENUM_CURRENT_SETTINGS},
};

extern crate getopts;

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("l", "list", "show available resolutions");
    opts.optflag("h", "help", "show this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            panic!("{}", f.to_string())
        }
    };

    // TODO: figure out if i can match case instead of if treeing
    if matches.opt_present("h") {
        let brief = format!("Usage: {} [option]", program);
        print!("{}", opts.usage(&brief));
        return;
    }

    //if matches.opt_present("l") {
    unsafe {
        for display in list_devices() {
            println!("Identifing {}", display);
            let mut mode = DEVMODEA::default();
            if EnumDisplaySettingsA(PCSTR(display.as_ptr()), ENUM_CURRENT_SETTINGS, &mut mode)
                .as_bool()
            {
                println!(
                    "{}: {}x{} @ {}Hz",
                    display, mode.dmPelsWidth, mode.dmPelsHeight, mode.dmDisplayFrequency
                );
            }
        }
    }
    //}
}
