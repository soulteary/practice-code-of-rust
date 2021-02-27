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
        _turn += 1;

        let mut guess_number = String::new();
        io::stdin()
            .read_line(&mut guess_number)
            .expect("Input error.");

        let guess_number: u8 = match guess_number.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!(
                    "> Should pick a number between [{}, {}]",
                    _min_number, _max_number
                );
                continue;
            }
        };
        println!("Your input: {}", guess_number);
        println!("");

        if guess_number >= MAX_LIMIT {
            println!("> Should be less than the maximum.");
            continue;
        } else if guess_number <= MIN_LIMIT {
            println!("> Should be greater than the minimum.");
            continue;
        }

        match guess_number.cmp(&_secret_number) {
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
        }
    }
}
