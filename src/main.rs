/* hilite - runs a command, highlighting everything it sends to stderr
 * based on hilite by Mike Schiraldi <mgs21@columbia.edu>
 *
 */

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
    if args.len() <  2 {
        print_stderr!("{}: specify a command to execute\n",
                      args.get(0).unwrap());
        return;
    }

    let program_name = args.get(1).unwrap();
    let (_, program_args) = args.split_at(2);

    let running_program = process::Command::new(program_name)
                              .args(& program_args)
                              .stderr(process::Stdio::piped())
                              .spawn()
                              .unwrap_or_else({|_| panic!("Failed to spawn program") });

    let mut running_program_stderr = running_program.stderr.unwrap();

    let style = HighlightStyles::RedUnderline;
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
