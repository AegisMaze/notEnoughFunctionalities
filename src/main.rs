use std::env;
use std::io;
use std::io::{BufRead, BufReader};
use std::string::String;
use std::fs::File;
use rand::thread_rng;
use rand::seq::SliceRandom;
use rand::seq::IteratorRandom;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args[1].as_str() {
        "calculator" => calculator(),
        "eightball" => eightball(),
        "hangman" => hangman(),
        _ => println!("Unknown command!"),
    }
}

fn calculator() {
    println!("Enter 'n' to set current result to 'n', where 'n' is a number");
    println!("Enter 'n op n' to change the result to that of the calculation, where 'op' is an operator");
    println!("Enter 'op n' to implicitly use the current result as first operand");
    println!("Allowed operators are: '+', '-', '*', '/'");
    println!("Enter 'exit' to exit the program");

    let mut res : f32 = 0.0;

    loop {

        println!("Current result: {}", res);
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Input could not be read");
        let mut words = input.split_whitespace();

        let mut arg1;
        
        match words.next() {
            None => {println!("Please enter some arguments..."); continue;},
            Some("exit") => break,
            Some(i) => arg1 = i,
        }
        
        let arg2 = words.next();
        
        if arg2.is_none() {
            match arg1.parse::<f32>() {
                Err(_) => println!("Number could not be parsed"),
                Ok(i) => res = i,
            }
            continue;
        }

        let mut arg2 = arg2.unwrap();
        let arg3 = words.next();
        let mut temp1 = res;

        if arg3.is_some() {
            match arg1.parse::<f32>() {
                Err(_) => {println!("Number could not be parsed"); continue;},
                Ok(i) => temp1 = i,
            }
            arg1 = arg2;
            match arg3 {
                None => {println!("Second number not found"); continue},
                Some(i) => arg2 = i,
            }
        }

        let temp2;
        match arg2.parse::<f32>() {
            Err(_) => {println!("Number could not be parsed"); continue;},
            Ok(i) => temp2 = i,
        }

        match arg1 {
            "+" => res = temp1 + temp2,
            "-" => res = temp1 - temp2,
            "*" => res = temp1 * temp2,
            "/" => res = temp1 / temp2,
            _ => {println!("Unknown operand"); continue;},
        }
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

fn hangman() {
    println!("Welcome to hangman! Enter a lowercase letter to make a guess, seven wrong guesses and you lose");
    println!("Enter exit to leave the game");

    let filename = "src/hangman.txt";
    let f = File::open(filename)
        .unwrap_or_else(|_| panic!("File not found"));
    let f = BufReader::new(f);

    let lines = f.lines().map(|l| l.expect("Couldn't read line"));

    let word = lines
        .choose(&mut rand::thread_rng())
        .expect("File had no lines");

    let mut wrong_guesses = 0;
    
    let mut gallows = String::from(" _____\n |    \n |     \n |     \n |     \n |");
    let mut corrects = String::from(" _".repeat(word.len()));
    let mut guessed = String::from(" Wrong guesses:");
    
    let mut count = 0;

    loop {
        println!("{}\n\n{}\n\n{}", gallows, corrects, guessed);

        if wrong_guesses == 7 {
            println!("You lose! The word was: {}", word);
            return;
        }

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Input could not be read");

        if input == "exit\n" {
            break;
        }

        if input.len() != 2 {
            println!("Please enter a single, lowercase letter as input");
            continue;
        }

        let letter = input.chars().next().unwrap();

        if !letter.is_lowercase() || input != format!("{}\n", letter) {
            println!("Please enter a single, lowercase letter as input");
            continue;
        }

        let mut wrong = true;

        let occurs : Vec<_> = word.match_indices(letter).map(|(i, _)|i).collect();
        for i in occurs {
            wrong = false;
            count += 1;
            corrects.replace_range((2 * i + 1)..(2 * i + 2), letter.to_string().as_str());
        }

        if count == word.len() {
            println!("You Win!");
            return;
        }

        if wrong {
            guessed.push(' ');
            guessed.push(letter);
            match wrong_guesses {
                0 => gallows.replace_range(12..13, "|"),
                1 => gallows.replace_range(19..20, "O"),
                2 => gallows.replace_range(27..28, "|"),
                3 => gallows.replace_range(26..27, "/"),
                4 => gallows.replace_range(28..29, "\\"),
                5 => gallows.replace_range(34..35, "/"),
                6 => gallows.replace_range(36..37, "\\"),
                _ => println!("This shouldn't happen..."),
            }
            wrong_guesses += 1
        }

    }
}