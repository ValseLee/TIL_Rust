// directory name == mod
mod guessing_number;
mod rust_programming_concepts;
mod server_clone;

use std::fs;

fn main() {
    /* Day 1: Guessing Number Practice */
    // guessing_number::guessing_number_mod::guessing_number();

    /* Day2: rust_programming_concepts */
    // rust_programming_concepts::rust_programming_concepts::variable_control_shadowing();
    // rust_programming_concepts::rust_programming_concepts::variable_control_mut();

    server_clone::rust_server::regist_listener();
    // let paths = fs::read_dir("./").unwrap();

    // for path in paths {
    //     println!("Name: {}", path.unwrap().path().display())
    // }
}

