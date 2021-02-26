use rand::Rng;
use std::cmp::Ordering;
use std::io;

const MIN_LIMIT: u8 = 1;
const MAX_LIMIT: u8 = 100;

fn main() {
    let mut _min_number = MIN_LIMIT;
    let mut _max_number = MAX_LIMIT;
    let mut _turn = 1;

    let _secret_number = rand::thread_rng().gen_range(MIN_LIMIT, MAX_LIMIT);

    println!("[Guess Number]");

    loop {
        println!(
            "[TURN #{}]Give me an answer? [{}, {}]",
            _turn, _min_number, _max_number
        );

        let mut guess_number = String::new();
        io::stdin()
            .read_line(&mut guess_number)
            .expect("Input error.");

        let guess_number: u8 = guess_number.trim().parse().expect("Input error.");
        println!("Your input: {}", guess_number);
        println!("");

        _turn += 1;

        match guess_number.cmp(&_min_number) {
            Ordering::Less => {
                println!("> Input error.");
                continue;
            }
            Ordering::Equal => {
                println!("> Should be greater than the minimum.");
                continue;
            }
            Ordering::Greater => match guess_number.cmp(&_max_number) {
                Ordering::Greater => {
                    println!("> Input error.");
                    continue;
                }
                Ordering::Equal => {
                    println!("> Should be less than the maximum.");
                    continue;
                }
                Ordering::Less => match guess_number.cmp(&_secret_number) {
                    Ordering::Less => {
                        println!("> The value is too small, readjust the range.\n");
                        _min_number = guess_number;
                    }
                    Ordering::Greater => {
                        println!("> The value is too large, readjust the range.\n");
                        _max_number = guess_number;
                    }
                    Ordering::Equal => {
                        println!("> You Win!\n");
                        break;
                    }
                },
            },
        }
    }
}
