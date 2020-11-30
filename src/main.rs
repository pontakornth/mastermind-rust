use std::io;
mod mastermind;
fn main() {
    let stdin = io::stdin();
    let mut command = String::new();
    println!("Input your command(play, explain):");
    stdin.read_line(&mut command).unwrap();
    match command.trim() {
        "play" => mastermind::play(),
        "explain" => println!("Explanation goes brrrr."),
        _ => println!("Invalid command!"),
    }
}
