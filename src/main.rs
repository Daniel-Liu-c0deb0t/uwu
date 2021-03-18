use uwu_ifier::uwu_ify_sse;

use clap::{App, Arg};

use std::io::prelude::*;
use std::io;
use std::fs::File;

const LEN: usize = 1 << 14;

fn main() {
    let matches = App::new("uwu")
        .about("fastest text uwu-ifier in the west")
        .arg(Arg::with_name("INPUT")
             .help("input text file")
             .required(false)
             .index(1))
        .arg(Arg::with_name("OUTPUT")
             .help("output text file")
             .required(false)
             .index(2))
        .get_matches();

    let mut reader: Box<dyn Read> = match matches.value_of("INPUT") {
        Some(path) if path != "-" => Box::new(File::open(path).unwrap()),
        _ => Box::new(io::stdin())
    };

    let mut writer: Box<dyn Write> = match matches.value_of("OUTPUT") {
        Some(path) if path != "-" => Box::new(File::create(path).unwrap()),
        _ => { Box::new(io::stdout()) }
    };

    let mut bytes = [0u8; LEN];
    let mut temp_bytes1 = [0u8; LEN * 4];
    let mut temp_bytes2 = [0u8; LEN * 4];

    loop {
        let len = match reader.read(&mut bytes) {
            Ok(len) if len == 0 => break,
            Ok(len) => len,
            Err(e) if e.kind() == io::ErrorKind::Interrupted => continue,
            Err(e) => panic!("{}", e)
        };

        let res = uwu_ify_sse(&bytes, len, &mut temp_bytes1, &mut temp_bytes2);
        writer.write_all(res).unwrap();
    }
}
