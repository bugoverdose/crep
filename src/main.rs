use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(args);
}

// cargo run -- arg1 arg2
// Running `target/debug/crep arg1 arg2`
// [src/main.rs:5] args = [
//     "target/debug/crep",
//     "arg1",
//     "arg2",
// ]
