/* hilite - runs a command, highlighting everything it sends to stderr
 * based on hilite by Mike Schiraldi <mgs21@columbia.edu>
 *
 */

use std::env;
use std::io;
use std::io::Write;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <  2 {
        write!(&mut io::stderr(),
               "{}: specify a command to execute\n",
               args.get(0).unwrap()).unwrap();
        return;
    }

    let program_name = args.get(1).unwrap();
    let (_, program_args) = args.split_at(2);

    println!("Running {} with args {:?}", program_name, program_args);

    let mut program_command: process::Command = process::Command::new(program_name);
    let output = program_command.spawn().unwrap_or_else({|_| panic!("Failed to spawn program") });
}
