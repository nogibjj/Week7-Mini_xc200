use std::io;

fn main() {
    println!("Welcome to the Dice Roller!");
    let mut total_rolls = 0;
    
    loop {
        println!("Enter the number of sides on the dice (0 to exit):");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        
        let num_sides: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input, please enter a number.");
                continue;
            }
        };
        
        if num_sides == 0 {
            println!("Exiting the program...");
            break;
        }
        
        let roll_result = rand::thread_rng().gen_range(1, num_sides + 1);
        println!("You rolled a {}!", roll_result);
        total_rolls += 1;
        println!("Total rolls: {}\n", total_rolls);
    }
}
