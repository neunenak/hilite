use std::io;
use std::io::Write;

fn main() {
    println!("This goes to stdout");
    write!(&mut io::stderr(),
            "This goes to stderr\n").unwrap();
}

