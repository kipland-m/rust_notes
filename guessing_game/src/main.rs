use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    // from rand we used Rng at the top of our file.
    // but in the generation of the number there is no Rng anywhere.
    // this is because Rng is a trait that when in scope gives access
    // to different RNG methods
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop{
    println!("Please input your guess.");

    // this creates a new mutable string
    let mut guess = String::new();

    // input from std
    io::stdin()
        // read_line is taking input and passing input to guess
        // the & is making this reference to guess immutable

        // ALSO- read_line returns a Result. This can be either Ok or Error.
        .read_line(&mut guess)

        // This expect line handles if read_line result is Error

        //    $ cargo build
        //    Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
        //    warning: unused `Result` that must be used
        //   --> src/main.rs:10:5
        //    |
        // 10 |     io::stdin().read_line(&mut guess);
        //    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        //    |
        //    = note: this `Result` may be an `Err` variant, which should be handled
        //    = note: `#[warn(unused_must_use)]` on by default

        // This- coming from python- is crazy. The code would error out if there was 
        // no expect. So to fix your error- would be to write error-handling code.
        .expect("Failed to read line");

    // this allows rust to reuse the 'guess' variable name
    // and convert the string to an integer by using .trim() to rid of whitespace
    // and .parse() to grab the data
    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };


    // Using our little crab pincers {} to place our variable in the println
    println!("You guessed: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => {
            println!("You win! Winner Winner");
            break;
        } 

    }
}
}