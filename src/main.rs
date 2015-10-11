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
    ($($arg:tt)*) => { write!(&mut io::stderr(), "{}", format_args!($($arg)*)) }
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

    let mut program_command: process::Command = process::Command::new(program_name);
    program_command.stderr(process::Stdio::piped());
    let mut running_program = program_command.spawn().unwrap_or_else({|_| panic!("Failed to spawn program") });
    let mut running_program_stderr = running_program.stderr.unwrap();

    let mut buf = [0; 4096];
    loop {
        let res = running_program_stderr.read(&mut buf[..]);
        match res {
            Ok(0) => break,
            Ok(bytes) => {
                let s = String::from_utf8_lossy(&mut buf);
                print_stderr!("{}", s);
            },
            Err(_) => panic!("Error reading from child process")
        }
    }
}
