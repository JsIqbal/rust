pub fn run() {
    println!("Hello World!");
    println!("Number: {}", 5);

    // Basic formatting
    println!("{} is from {}", "Iqbal", "Dhaka");

    // Postional Arguments
    println!(
        "{0} is from {1} and {0} likes to {2}",
        "Iqbal", "Dhaka", "code"
    );

    // Named Arguments
    println!(
        "{name} likes to play {game}!",
        name = "Iqbal",
        game = "Chess"
    );

    // Placeholder Traits
    println!(
        "Binary: {:b}, Hex: {:x}, Octal: {:o}", 10, 10, 10
    );

    // Placeholder for debug Traits
    println!("{:?}", (10, "hello", true));

    // Basic Math
    println!("10 + 10 = {}", 10 + 10);
}
