/* hilite - runs a command, highlighting everything it sends to stderr
 * based on hilite by Mike Schiraldi <mgs21@columbia.edu>
 *
 */

use std::env;
use std::io;
use std::io::Write;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <  2 {
        write!(&mut io::stderr(),
               "{}: specify a command to execute\n",
               args.get(0).unwrap()).unwrap();
        return;
    }

    let run_command = args.get(1).unwrap();
    let (_, run_arguments) = args.split_at(2);

    println!("Running {} with args {:?}", run_command, run_arguments);

}
