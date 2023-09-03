extern crate rand;

pub mod guessing_number_mod {
    use std::{
        io, cmp::Ordering
    };

    use rand::Rng;

    pub fn guessing_number() {
        // !가 붙어있으면 메소드가 아니라 Rust Macro이다
        println!("Guess The Number!");

        let secret_number: u32 = rand::thread_rng().gen_range(1, 101);

        loop {
            let mut guessed_num = String::new();

            io::stdin()
                .read_line(&mut guessed_num)
                .expect("Failed to read line");

            // Rust: variable Shadowing 
            // useCase: TypeCasting
            let guessed_num: u32 = match guessed_num
                .trim()
                .parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            println!("You Number is: {}", guessed_num);

            match guessed_num.cmp(&secret_number) {
                Ordering::Less    => println!("Too small!"),
                Ordering::Greater => println!("Too big!"),
                Ordering::Equal   => {
                    println!("You win!");
                    break;
                }
            }
        }
    }
}