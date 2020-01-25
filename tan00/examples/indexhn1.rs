use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process;
use std::string::String;

use crossbeam::crossbeam_channel::{unbounded, Receiver};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Item {
    id: u64,
    title: String,
}

fn process_lines(r: Receiver<String>) {
    let item_json = r.recv().unwrap();

    let item: Item = serde_json::from_str(&item_json).unwrap();
    let id = &item.id;
    let title = &item.title;
    println!("{} {}", id, title);
}

fn read_file_to_buffer(filename: String) {
    let f = File::open(filename).unwrap();
    let file = BufReader::new(&f);

    for (_num, line) in file.lines().enumerate() {
        // Create a channel of unbounded capacity.
        let (s, r) = unbounded();

        let l = line.unwrap();
        // Send a message into the channel.
        s.send(l).unwrap();
        process_lines(r);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("You need to enter a filename");
        process::exit(1);
    }
    let filename = &args[1];
    println!("In file {}", filename);

    let _contents = read_file_to_buffer(filename.to_string());

    // println!("With text:\n{:?}", contents);
}
