use rand::Rng;
use std::cmp::Ordering;
use std::io;

struct Range {
    min: u8,
    max: u8,
}

struct Notice {}

impl Notice {
    fn show_game_title() {
        println!("[Guess Number]");
    }
    fn show_question(turn: &u8, min: &u8, max: &u8) {
        println!("[TURN #{}]Give me an answer? [{}, {}]\n", turn, min, max);
    }

    fn show_user_input(num: &u8) {
        println!("Your input: {}\n", &num);
    }

    fn show_out_of_range(min: &u8, max: &u8) {
        println!("> Should pick a number between [{}, {}]\n", min, max);
    }
    fn show_too_large() {
        println!("> Should be less than the maximum.\n");
    }
    fn show_too_small() {
        println!("> Should be greater than the minimum.\n");
    }
    fn show_less_than_answer() {
        println!("> The value is too small, readjust the range.\n");
    }
    fn show_larger_than_answer() {
        println!("> The value is too large, readjust the range.\n");
    }
}

struct Success {}

impl Success {
    fn show_win() {
        println!("> You Win!\n");
    }
}

const RANGE_LIMIT: Range = Range { min: 1, max: 100 };

fn main() {
    let mut _min_number: u8 = RANGE_LIMIT.min;
    let mut _max_number: u8 = RANGE_LIMIT.max;
    let mut _turn: u8 = 1;

    let _secret_number = rand::thread_rng().gen_range(RANGE_LIMIT.min, RANGE_LIMIT.max + 1);

    Notice::show_game_title();

    loop {
        Notice::show_question(&_turn, &_min_number, &_max_number);
        _turn += 1;

        let mut guess_number = String::new();
        io::stdin()
            .read_line(&mut guess_number)
            .expect("Input Error.");

        let guess_number: u8 = match guess_number.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                Notice::show_out_of_range(&_min_number, &_max_number);
                continue;
            }
        };

        Notice::show_user_input(&guess_number);

        if guess_number >= _max_number {
            Notice::show_too_large();
            continue;
        } else if guess_number <= _min_number {
            Notice::show_too_small();
            continue;
        }

        match guess_number.cmp(&_secret_number) {
            Ordering::Less => {
                Notice::show_less_than_answer();
                _min_number = guess_number;
            }
            Ordering::Greater => {
                Notice::show_larger_than_answer();
                _max_number = guess_number;
            }
            Ordering::Equal => {
                return Success::show_win();
            }
        }
    }
}
