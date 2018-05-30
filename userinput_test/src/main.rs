
use std::io::stdin;

fn main() {

    'outer: loop { //Naming loop
        let number: i32 = 10;
        println!("Pick a number");

        loop {
            let mut line = String::new();

            let input = stdin().read_line(&mut line); //Reference to the input

            let guess: Option<i32> = input.ok().map_or(None, |_| line.trim().parse().ok()); //.ok means the end of the line of input

            match guess {
                None => println!("Enter a number"),
                Some(n) if n == number => {
                    println!("You guessed it!");
                    break 'outer; //Breaks out of loop completely
                }
                Some(n) if n < number  => println!("Number is too low!"),
                Some(n) if n > number => println!("Number is too high!"),
                Some(_) => println!("An error occured"),
            }
        }
    } 
}
