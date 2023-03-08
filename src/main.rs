use rand::Rng;
use clipboard::ClipboardProvider;
use clipboard::ClipboardContext;

fn main() {
    let mut password_length = String::new();
    let mut use_uppercase = String::new();
    let mut use_lowercase = String::new();
    let mut use_numbers = String::new();
    let mut use_symbols = String::new();

    println!("Enter password length:");
    std::io::stdin().read_line(&mut password_length).unwrap();
    let password_length: usize = password_length.trim().parse().unwrap();

    println!("Include uppercase letters? (y/n)");
    std::io::stdin().read_line(&mut use_uppercase).unwrap();
    let use_uppercase = use_uppercase.trim().to_lowercase() == "y";

    println!("Include lowercase letters? (y/n)");
    std::io::stdin().read_line(&mut use_lowercase).unwrap();
    let use_lowercase = use_lowercase.trim().to_lowercase() == "y";

    println!("Include numbers? (y/n)");
    std::io::stdin().read_line(&mut use_numbers).unwrap();
    let use_numbers = use_numbers.trim().to_lowercase() == "y";

    println!("Include symbols? (y/n)");
    std::io::stdin().read_line(&mut use_symbols).unwrap();
    let use_symbols = use_symbols.trim().to_lowercase() == "y";

    let characters: Vec<char> = [
        if use_uppercase { (65..=90) } else { (0..0) },
        if use_lowercase { (97..=122) } else { (0..0) },
        if use_numbers { (48..=57) } else { (0..0) },
        if use_symbols { (33..=47).chain(58..=64).chain(91..=96).chain(123..=126) } else { (0..0) },
    ]
    .iter()
    .flatten()
    .map(|&i| i as u8 as char)
    .collect();

    let password: String = (0..password_length)
        .map(|_| characters[rand::thread_rng().gen_range(0..characters.len())])
        .collect();

    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.set_contents(password).unwrap();

    println!("Password generated: {}", password);
    println!("Password copied to clipboard");
}
