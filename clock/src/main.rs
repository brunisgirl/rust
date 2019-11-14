mod lib;
use lib::Clock;

fn main() {
    // Statements here are executed when the compiled binary is called

    // Print text to the console
    println!("{}", -10 % 20);
    println!("{}", Clock::new(1, -160).to_string());
    //assert_eq!(Clock::new(6, 15), Clock::new(6, -4305))
}
