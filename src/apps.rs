use rand::Rng;
use std::io::stdin;

pub fn number_guessing() {
    let number = rand::thread_rng().gen_range(1..101);
    loop {
        println!("Enter your guess:");
        let mut buffer = String::new();
        match stdin().read_line(&mut buffer) {
            Ok(_) => {
                let parsed = buffer.trim_end().parse::<i64>();
                match parsed {
                    Ok(guess) => {
                        if guess < 1 || guess > 100  {
                            println!("Your guess is out of range");
                        } else if guess < number {
                            println!("Your guess is low")
                        } else if guess > number {
                            println!("Your guess is high")
                        } else {
                            println!("Correct!!");
                            break;
                        }
                    },
                    Err(e) => {
                        println!("Error occured! {}, Try again!", e);
                    }
                }
            },
            Err(_) => continue
        }
    }
}