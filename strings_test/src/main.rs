fn main() {
    let rand_string = "I am a random string";

    println!("Length: {}", rand_string.len());

    let (first,second) = rand_string.split_at(6);
    println!("First {} Second: {}", first, second);

    let mut chars = rand_string.chars(); //Auto type declaration. mutable means it can be changed

    let mut indiv_char = chars.next();

    loop {
        match indiv_char { //Similar to a switch block
            Some(x) => {
                println!("{}", x);
            }
            None => break,
        }
        indiv_char = chars.next(); //Gets the next character in the array
    }

    let mut iter = rand_string.split_whitespace(); //Splits into an array based on whitespace

    let mut indiv_word = iter.next(); //Gets the next value in the array

    loop { //Prints the words in the array 1 by 1
        match indiv_word {
            Some(x) => {
                println!("{}", x)
            }
            None => break,
        }
        indiv_word = iter.next(); //Gets the next value in the array
    }

    let rand_string2 = "I am a randoms string\nThere are other string like it\nThis string is the best";

    let mut lines = rand_string2.lines(); //Seperates string into an array based off of lines (denoted by \n)
    let mut indiv_line = lines.next();

    loop {
        match indiv_line {
            Some(x) => println!("{}", x), //Simpler way of writing the same code
            None => break,
        }
        indiv_line = lines.next();
    }

    println!("Find Best: {}", rand_string2.contains("best")); //Prints true if the word 'best' is found within the string
}
