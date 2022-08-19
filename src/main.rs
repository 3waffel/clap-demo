mod cli;

use ansi_term::Colour;
use clap::ArgMatches;
use cook_utils::*;
use select::document::Document;
use select::predicate::Name;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() {
    let matches = cli::build_cli().get_matches();

    read_file(&matches);
    thread_count(&matches);
    make_request(&matches).await.unwrap();
    build_tree(&matches);
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

                for _ in 0..2 {
                    let count_clone = Arc::clone(&count);
                    let handle = std::thread::spawn(move || {
                        for _ in if n > 0 { 0..n } else { n..0 } {
                            *count_clone.lock().unwrap() += n.clamp(-1, 1);
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

fn build_tree(matches: &ArgMatches) {
    let str = matches.get_one::<String>("tree");
    match str {
        None => {}
        Some(str) => {
            let mut nodes = vec![];
            let _ = str.split_ascii_whitespace().for_each(|e| {
                nodes.push(e.parse::<i32>().expect("Error input"));
            });
            println!("{:?}", &nodes);
            println!("{:#?}", tree!(1, tree!(1, tree!(1, tree!(1), None), None), None));
        }
    }
}
