use uwu_ifier::uwu_ify_sse;

use clap::{App, Arg};

use parking_lot::Mutex;

use std::io::prelude::*;
use std::io;
use std::fs::File;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;
use std::collections::HashMap;
use std::time::Instant;

// should be small enough so stuff fits in L1/L2 cache
// but big enough so each thread has enough work to do
const LEN: usize = 1 << 16;

fn main() {
    let matches = App::new("uwu")
        .about("fastest text uwu-ifier in the west")
        .arg(Arg::with_name("INPUT")
             .help("input text file")
             .default_value("-")
             .index(1))
        .arg(Arg::with_name("OUTPUT")
             .help("output text file")
             .default_value("-")
             .index(2))
        .arg(Arg::with_name("threads")
             .help("number of threads")
             .short("t")
             .long("threads")
             .value_name("THREADS")
             .takes_value(true)
             .default_value("1"))
        .get_matches();

    let in_path = matches.value_of("INPUT").unwrap();
    let out_path = matches.value_of("OUTPUT").unwrap();
    let thread_count = matches.value_of("threads").unwrap().parse::<usize>().unwrap();

    let reader: Box<dyn Read + Send> = if in_path == "-" {
        Box::new(io::stdin())
    } else {
        Box::new(File::open(in_path).unwrap())
    };

    let writer: Box<dyn Write + Send> = if out_path == "-" {
        Box::new(io::stdout())
    } else {
        Box::new(File::create(out_path).unwrap())
    };

    let start_time = Instant::now();
    parallel_uwu(reader, writer, thread_count);
    let duration = start_time.elapsed();
    eprintln!("time taken: {} ms", duration.as_millis());
}

fn parallel_uwu(reader: Box<dyn Read + Send>, writer: Box<dyn Write + Send>, thread_count: usize) {
    let reader_idx = Arc::new(Mutex::new((reader, 0usize)));
    let writer = Arc::new(Mutex::new(writer));
    let write_idx = Arc::new(AtomicUsize::new(0));
    let idx_thread = Arc::new(Mutex::new(HashMap::with_capacity(thread_count)));

    let mut threads = Vec::with_capacity(thread_count);

    for _i in 0..thread_count {
        let reader_idx = Arc::clone(&reader_idx);
        let writer = Arc::clone(&writer);
        let write_idx = Arc::clone(&write_idx);
        let idx_thread = Arc::clone(&idx_thread);

        threads.push(thread::spawn(move || {
            let mut bytes = [0u8; LEN];
            let mut temp_bytes1 = [0u8; LEN * 8];
            let mut temp_bytes2 = [0u8; LEN * 8];

            loop {
                let (len, read_idx) = {
                    let mut curr_reader_idx = reader_idx.lock();
                    curr_reader_idx.1 += 1;
                    let len = read_as_much_as_possible(&mut curr_reader_idx.0, &mut bytes);
                    (len, curr_reader_idx.1 - 1)
                };

                let res = uwu_ify_sse(&bytes, len, &mut temp_bytes1, &mut temp_bytes2);

                idx_thread.lock().insert(read_idx, thread::current());
                while write_idx.load(Ordering::Acquire) != read_idx {
                    // literally won't have anything to do until another thread updates write_idx
                    thread::park();
                }
                // at this point, only one thread is using writer at one time
                writer.lock().write_all(res).unwrap();
                write_idx.fetch_add(1, Ordering::Release);
                {
                    let mut map = idx_thread.lock();
                    map.remove(&read_idx);
                    if let Some(next_thread) = map.get(&(read_idx + 1)) {
                        next_thread.unpark();
                    }
                }

                if len < LEN {
                    break;
                }
            }
        }));
    }

    for thread in threads.into_iter() {
        thread.join().unwrap();
    }
}

fn read_as_much_as_possible(reader: &mut Box<dyn Read + Send>, mut bytes: &mut [u8]) -> usize {
    let mut res = 0;
    while bytes.len() > 0 {
        match reader.read(&mut bytes) {
            Ok(len) if len == 0 => break,
            Ok(len) => {
                bytes = &mut bytes[len..];
                res += len;
            },
            Err(e) if e.kind() == io::ErrorKind::Interrupted => continue,
            Err(e) => panic!("{}", e)
        }
    }
    res
}
