// use std::fs::{self, File};
// use std::io;
// use std::io::prelude::*;

// fn main() -> io::Result<()> {
//     // let mut f = File::open("foo.txt")?;
//     // let mut buffer = String::new();
//     let path = File::open("foo.txt")?;

//     // f.read_to_string(&mut buffer)?;
//     fs::read_to_string(path);
//     dbg!(path);

//     Ok(())
// }
fn main() {
    println!("{}", std::fs::read_to_string("foo.txt").unwrap());
}
