use std::io;
use std::io::Write; // bring flush() into scope

fn main() {
    loop {
        println!("\nHello User, what would you like to do?");
        println!(" 1. Convert Temperature: Celsius <-> Fahrenheit");
        println!(" 2. Generate the nth Fibonacci number");
        println!(" 3. Print the Lyrics to \"The Twelve Days of Christmas\"");
        println!(" 4. Quit");
        print!("Your Choice: ");
        io::stdout().flush().unwrap();
    
        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");
        let choice: u32 = choice.trim().parse().expect("Invalid input");

        match choice {
            1 => convert_temp(),
            2 => fibonacci_binets_formula(),
            3 => christmas_carol(),
            0_u32 | 4_u32..=u32::MAX => break
        }
    }
}

fn convert_temp() {
    println!("*Convert Temperature: Celsius <-> Fahrenheit*");
    println!(" 1. Convert Celsius to Fahrenheit");
    println!(" 2. Convert Fahrenheit to Celsius");
    print!("Your Choice: ");
    io::stdout().flush().unwrap();

    let mut choice: String = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read line");
    let choice: u8 = choice.trim().parse().expect("Invalid input");

    if choice == 1 {
        println!("*Celsius to Fahrenheit Converter*");
        print!(" Enter the temperature in Celsius: ");
        io::stdout().flush().unwrap();

        let mut temp_c: String = String::new();
        io::stdin().read_line(&mut temp_c).expect("Failed to read line");
        let temp_c: f32 = temp_c.trim().parse().expect("Invalid input");
        
        println!(" {temp_c} °C = {} °F", (temp_c * 9.0/5.0) + 32.0);
    } else if choice == 2 {
        println!("*Fahrenheit to Celsius Converter*");
        print!(" Enter the temperature in Fahrenheit: ");
        io::stdout().flush().unwrap();

        let mut temp_f: String = String::new();
        io::stdin().read_line(&mut temp_f).expect("Failed to read line");
        let temp_f: f32 = temp_f.trim().parse().expect("Invalid input");
        
        println!(" {temp_f} °F = {} °C", (temp_f - 32.0) * 5.0/9.0);
    }
}

fn fibonacci_binets_formula() {
    println!("*Generate the nth Fibonacci number*");
    print!(" Enter n: ");
    io::stdout().flush().unwrap();

    let mut n: String = String::new();
    io::stdin().read_line(&mut n).expect("Failed to read line");
    let n: f64 = n.trim().parse().expect("Invaid input");

    if n >= 1.0 && n <= 71.0 {
        // Binet's Formula (ref: https://r-knott.surrey.ac.uk/Fibonacci/fibFormula.html)
        // Accurate for 1 <= n <= 71
        println!(" Fib({n}) = {}", ((1.61803398874989484820_f64.powf(n) - (-0.61803399984989484820_f64.powf(n))) / 5_f64.powf(0.5)).floor());
    } else {
        // Takes incredibly long time: for n = 72, I guess it would take >300 hours on a 4GHz core. [Time complexity: T(n) = O(1.6180)^n]
        // Ref: https://math.stackexchange.com/questions/4619842/time-complexity-of-computing-fibonacci-numbers-using-naive-recursion
        println!(" Fib({n}) = {}", fibonacci_naive_recursion(n as u128));
    }
}

fn fibonacci_naive_recursion(n: u128) -> u128 {
    // println!(" n: {n}");
    if n <= 1 {
        return n;
    } else {
        return fibonacci_naive_recursion(n - 1) + fibonacci_naive_recursion(n - 2);
    }
}

fn christmas_carol() {
    println!("*Lyrics to the \"The Twelve Days of Christmas\"*");
    let lyrics = [
        "Twelve drummers drumming",
        "Eleven pipers piping",
        "Ten lords a-leaping",
        "Nine ladies dancing",
        "Eight maids a-milking",
        "Seven swans a-swimming",
        "Six geese a-laying",
        "Five golden rings",
        "Four calling birds",
        "Three french hens",
        "Two turtle doves, and",
        "A partridge in a pear tree",
    ];
    for verse in 1..13 {
        println!("\n[Verse {verse}]");
        if verse == 1 {
            println!("On the {verse}st day of Christmas, my true love sent to me");
        } else if verse == 2 {
            println!("On the {verse}nd day of Christmas, my true love sent to me");
        } else if verse == 3 {
            println!("On the {verse}rd day of Christmas, my true love sent to me");
        } else {
            println!("On the {verse}th day of Christmas, my true love sent to me");
        }
        // Why does this not need mut?
        // is it because the compiler swaps the expression inplace of lyrics_start_num in the for loop?
        // like, for n in lyrics.len() - verse..lyrics.len() {... ?
        // 29-05-2023. You blathering baboon! The scope must ends after each iteration so it is redefined every iteration, I..I guess...
        let lyrics_start_num = lyrics.len() - verse;
        for n in lyrics_start_num..lyrics.len() {
            println!("{}", lyrics[n]);
        }
    }
}