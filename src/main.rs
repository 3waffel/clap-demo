use ansi_term::Colour;
use clap::ArgMatches;
use select::document::Document;
use select::predicate::Name;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::{Arc, Mutex};

mod cli;

#[tokio::main]
async fn main() {
    let matches = cli::build_cli().get_matches();

    read_file(&matches);
    thread_count(&matches);
    make_request(&matches).await.unwrap();
    run_guess(&matches).await.unwrap();
}

fn read_file(matches: &ArgMatches) {
    let path = matches.get_one::<String>("file");
    match path {
        None => {}
        Some(path) => {
            let input = File::open(path);
            match input {
                Ok(input) => {
                    let buffered = BufReader::new(input);
                    for line in buffered.lines() {
                        println!("{}", line.unwrap());
                    }
                }
                Err(_) => println!("Invalid file: {}", path),
            }
        }
    }
}

fn thread_count(matches: &ArgMatches) {
    let num_str = matches.get_one::<String>("count");
    match num_str {
        None => {}
        Some(s) => match s.parse::<i32>() {
            Ok(n) => {
                let count = Arc::new(Mutex::new(0));
                let mut handle_vec = vec![];

                for i in 0..2 {
                    let count_clone = Arc::clone(&count);
                    let handle = std::thread::spawn(move || {
                        for _ in if n > 0 { 0..n } else { n..0 } {
                            let mut count_lock = count_clone.lock().unwrap();
                            *count_lock += n.clamp(-1, 1);
                            println!("thread {}: {}", i, count_lock);
                        }
                    });
                    handle_vec.push(handle);
                }

                handle_vec
                    .into_iter()
                    .for_each(|handle| handle.join().unwrap());
                println!(
                    "The number is: {}",
                    Colour::Blue.paint((&count.lock().unwrap()).to_string())
                )
            }

            Err(_) => println!("That's not a number!: {}", Colour::Red.paint(s)),
        },
    }
}

async fn make_request(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let url = matches.get_one::<String>("req");
    match url {
        None => {}
        Some(url) => {
            let res = reqwest::get(url).await?.text().await?;
            Document::from(res.as_str())
                .find(Name("a"))
                .filter_map(|n| n.attr("href"))
                .for_each(|x| println!("{}", x));
        }
    }
    Ok(())
}

use std::io::{stdin, stdout, Write};
async fn run_guess(matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    if matches.contains_id("guess") {
        let word: String = String::from("cheese");
        let mut answer = "_".repeat(word.len());
        let mut input = String::new();
        let mut lives: u32 = 3;

        let flush = stdout().flush();
        flush.expect("failed to flush stdout");

        loop {
            println!("{}", answer);
            stdin().read_line(&mut input).expect("invalid input");

            if input.len() == 2 {
                let c = input.chars().next().expect("no character found");

                if word.find(c) == None {
                    lives -= 1;
                }
                answer = answer
                    .chars()
                    .enumerate()
                    .map(|(i, x)| {
                        if word.chars().nth(i).unwrap() == c {
                            c
                        } else {
                            x
                        }
                    })
                    .collect::<String>();
            } else {
                println!("Please enter a single character");
            }

            if lives == 0 {
                println!("You died");
                return Ok(());
            }
            if answer == word {
                break;
            }
            input = "".to_owned();
        }
        println!("Congratulations!");
    }
    Ok(())
}
