/* hilite - runs a command, highlighting everything it sends to stderr
 * based on hilite by Mike Schiraldi <mgs21@columbia.edu>
 *
 */

extern crate getopts;
use std::env;
use std::io;
use std::io::Write;
use std::io::Read;
use std::process;

macro_rules! print_stderr {
    ($($arg:tt)*) => {
         (write!(&mut io::stderr(), "{}", format_args!($($arg)*))).unwrap();
    }
}

enum HighlightStyles {
    Red,
    Cyan,
    BlackUnderline,
    WhiteUnderline,
    RedUnderline,
    CyanUnderline,
    RedBackground,
    CyanBackground
}

fn color_code(style: HighlightStyles) -> &'static str {
    match style {
        HighlightStyles::Red => "\x1b[1;31m",
        HighlightStyles::Cyan => "\x1b[1;36m",
        HighlightStyles::BlackUnderline => "\x1b[4;30m",
        HighlightStyles::WhiteUnderline => "\x1b[4;37m",
        HighlightStyles::RedUnderline => "\x1b[4;31m",
        HighlightStyles::CyanUnderline => "\x1b[4;36m",
        HighlightStyles::RedBackground => "\x1b[41m",
        HighlightStyles::CyanBackground => "\x1b[46m",
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut opts = getopts::Options::new();
    opts.optopt("s", "style", "STYLE is one of: red | cyan | underline-{red|cyan|black|white} | background-{red|cyan}", "STYLE");
    opts.optflag("h", "help", "Print help");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string())
    };

    if matches.opt_present("h") {
        let brief = format!("Usage: OPTIONS PROGRAM {}", "{-- PROGRAM_OPTIONS }");
        print!("{}", opts.usage(&brief));
        return;
    }

    if matches.free.len() <  1 {
        print_stderr!("{}: specify a command to execute\n",
                      args.get(0).unwrap());
        return;
    }

    let style = match matches.opt_str("style") {
        Some(ref s) if s == "red" => HighlightStyles::Red,
        Some(ref s) if s == "cyan" => HighlightStyles::Cyan,
        Some(ref s) if s == "underline-black" => HighlightStyles::BlackUnderline,
        Some(ref s) if s == "underline-white" => HighlightStyles::WhiteUnderline,
        Some(ref s) if s == "underline-red" => HighlightStyles::RedUnderline,
        Some(ref s) if s == "underline-cyan" => HighlightStyles::CyanUnderline,
        Some(ref s) if s == "background-red" => HighlightStyles::RedBackground,
        Some(ref s) if s == "background-cyan" => HighlightStyles::CyanBackground,
        Some(ref s) => panic!("Bad option for style: {}", s),
        None => HighlightStyles::Red
    };

    let program_name = matches.free.get(0).unwrap();
    let (_, program_args) = matches.free.split_at(1);

    let running_program = process::Command::new(program_name)
                              .args(& program_args)
                              .stderr(process::Stdio::piped())
                              .spawn()
                              .unwrap_or_else({|_| panic!("Failed to spawn program") });

    let mut running_program_stderr = running_program.stderr.unwrap();

    let color_header = color_code(style);
    let color_footer = "\x1b[0m";

    let mut buf = [0; 4096];
    loop {
        let res = running_program_stderr.read(&mut buf[..]);
        match res {
            Ok(0) => break,
            Ok(_) => {
                print_stderr!("{}{}{}", color_header,
                                        String::from_utf8_lossy(&mut buf),
                                        color_footer);
            },
            Err(_) => panic!("Error reading from child process")
        }
    }
}
