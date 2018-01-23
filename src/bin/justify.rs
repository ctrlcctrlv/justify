extern crate justify;
use justify::{justify, Settings, InsertAt};
use std::io::stdin;
use std::env;
use std::process::exit;

static WCWIDTH_ENABLED: bool = cfg!(feature="unicode-width");

fn get_settings_from_args() -> Settings<'static> {
    let mut ret = Settings::default();

    for arg in env::args() {
        match &*arg {
            #[cfg(feature="unicode-width")]
            "-w" => {ret.wcwidth = true;},
            "-j" => {ret.justify_last_line = true},
            "-H" => {ret.hyphenate_overflow= true},
            "-i" => {ret.ignore_spaces = true},
            "-l" => {ret.insert_at = InsertAt::Left},
            "-r" => {ret.insert_at = InsertAt::Right},
            _ => {ret.width = arg.parse().unwrap_or(80)}
        }
    }

    ret
}

fn abort_if_help() {
    if env::args().any(|x| &*x == "-h" || &*x == "--help") {
        eprintln!("justify 0.1.0\n-w: If compiled with wcwidth, take Unicode into account when justifying.\n    If compiled without wcwidth, this option is ignored.\n-j: Justify the last line of each paragraph.\n-H: Hyphenate words that are longer than the width.\n-i: Ignore spaces when justifying - should be used with -H.\n-l: Insert spaces at the left.\n-r: Insert spaces at the right.\nAny number in the arguments will be used as the width.\nNote: Argument combination such as `-Hl` is not supported. Use `-H -l`.");
        if WCWIDTH_ENABLED {
            eprintln!("Unicode functionality via `wcwidth` is available.");
        } else {
            eprintln!("Unicode functionality via `wcwidth` is not available.");
        }
        exit(255);
    }
}

fn main() {
    abort_if_help();

    let mut input = String::new();
    let settings = get_settings_from_args();
    loop {
        input.clear();
        match stdin().read_line(&mut input) {
            Ok(0) => {return}, //EOF
            Ok(_) => {println!("{}", justify(&input, &settings))},
            Err(e)=> {eprintln!("Error: {:?}", e); std::process::exit(1);}
        }
    }
}
