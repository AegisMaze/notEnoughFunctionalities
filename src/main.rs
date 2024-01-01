use std::env;
use std::io;
use rand::thread_rng;
use rand::seq::SliceRandom;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args[1].as_str() {
        "eightball" => eightball(),
        _ => println!("Unknown command!"),
    }
}

fn eightball() {
    println!("Ask me a yes or no question about the future:");

    let mut question = String::new();
    io::stdin().read_line(&mut question).expect("Question could not be read");

    let responses = vec![
    "It is certain",
    "It is decidedly so",
    "Without a doubt",
    "Yes definitely",
    "You may rely on it",    
    "As I see it, yes",
    "Most likely",
    "Outlook good",
    "Yes",
    "Signs point to yes",
    "Reply hazy, try again",
    "Ask again later",
    "Better not tell you now",
    "Cannot predict now",
    "Concentrate and ask again",    
    "Don't count on it",
    "My reply is no",
    "My sources say no",
    "Outlook not so good",
    "Very doubtful" 
    ];
    println!("{}", responses.choose(&mut thread_rng()).expect("Fate itself seems to be broken..."));
}