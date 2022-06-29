pub fn run() {
    // print to console
    println!("Hello from the print.rs file");

    // Basic formatting
    println!("{} is from {}", "Kip", "Kentucky");

    // Formatting using positional args
    println!("{0} is from {1} and {0} likes to {2}","Dude","USA","Code");

    // Named arguments
    println!("{name} likes to play {game}",
    name = "Kip",
    game = "minecraft 2");

}