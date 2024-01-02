use std::env;
use std::io;
use rand::thread_rng;
use rand::seq::SliceRandom;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args[1].as_str() {
        "calculator" => calculator(),
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