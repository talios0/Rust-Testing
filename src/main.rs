use std::{i8, i16, i32, i64, u8, u16, u32, u64, isize, usize, f32, f64};

use std::io::stdin;

fn main() {
    println!("Hello World!");

    let _num = 10; //Makes a variable and guesses the type (constant)

    let mut _age: i32 = 16; //Makes a variable of type 32-bit float and is mutable (not constant)

    println!("Max i8 {}", i8::MAX);

    let is_is_true: bool = true;
    let let_x: char = 'x';

    println!("I am {} years old!", _age);

    let (_f_name, _l_name) = ("Charles", "R."); //Assign two variables at once!

    println!("It is {0} that {1} is {0}", is_is_true, let_x);

    println!("{:.2}", 1.234); //Only show two decimal places

    println!("B: {:b} H: {:x} O: {:o}", 10, 10, 10); //Binary Data, Hexadecimal, and Base 8

    println!("{ten:>ws$}", ten=10, ws=5); //Padding variables with whitespace
    println!("{ten:>0ws$}", ten=10, ws=5); //Padding variables with 1's

    println!("Sin 3.14 = {}", 3.14f64.sin()); //Gets the sine of 3.14 using a 64-bit float

    let age_old = 6;
    if age_old == 5 {
        println!("Go home!");
    } else if (age_old > 5) && (age_old <= 18) {
        println!("Go somewhere!");
    } else {
        println!("Go somewhere else!");
    }

    let mut x = 1;

    //LOOPING

    loop { //LOOPS
        if (x % 2) == 0 { //Even number
            println!("{}", x);
            x+=1;
            continue;
        }
        if x > 10 {
            break;
        }
        x+=1;
        continue;
    }

    let mut y = 1;

    while y <= 10 {
        println!("{}",y);
        y+=1;
    }

    for z in 1..10 { //1 to 9
        println!("FOR: {}", z);
    }
}