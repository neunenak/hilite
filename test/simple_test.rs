use std::io;
use std::io::Write;

fn main() {
    println!("This goes to stdout");
    println!("ユニコードを作りましょう");
    write!(&mut io::stderr(),
            "This goes to stderr\n").unwrap();
    write!(&mut io::stderr(),
            "ユニコードを作りましょう\n").unwrap();
}

