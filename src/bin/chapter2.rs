use std::cmp::Ordering;
use std::io;
use rand::Rng;

pub fn run_chapter_2() {
    let random: i32 = rand::thread_rng().gen_range(1..=10);

    loop {
        println!("Guess a number");
        let mut input:String = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read output");
        let input: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };
        let res = compare_them(&random, &input);
        print!("You guessed {}, {} it was : {}\n", input, res.message, random);
        if res.result {
            break;
        }
    }
}

fn compare_them(random: &i32, input: &i32) -> ResultComp {
    return match input.cmp(random) {
        Ordering::Less => ResultComp { message: "but it was too low!".to_string(), result: false },
        Ordering::Equal => ResultComp { message: "and you were right!!!".to_string(), result: true },
        Ordering::Greater => ResultComp { message: "but it was too high!".to_string(), result: false }
    };
}

struct ResultComp {
    message: String,
    result: bool,
}

fn main() {
    run_chapter_2()
}
