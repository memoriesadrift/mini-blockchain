use std::io;

mod models;
mod config;
mod miner;
mod storage;

fn main() {
    println!("Hello, world, enter client name!");
    let mut node_name: String = String::new();

    io::stdin()
        .read_line(&mut node_name)
        .expect("Failed to read line");

    println!("Hello, {}", node_name)
}
