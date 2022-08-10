use ansi_term::{Colour, Style};
use clap::{App, Arg, ArgMatches};
use select::document::Document;
use select::predicate::Name;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[tokio::main]
async fn main() {
    let matches = App::new("Cook CLI")
        .version("0.1.0")
        .author("Wafu")
        .about("A CLI for learning argument parsing")
        .arg(
            Arg::with_name("file")
                .short('f')
                .long("file")
                .takes_value(true)
                .help("Specify a file to input"),
        )
        .arg(
            Arg::with_name("num")
                .short('n')
                .long("number")
                .takes_value(true)
                .help("Five less than your fav number"),
        )
        .arg(
            Arg::with_name("req")
                .short('r')
                .long("request")
                .takes_value(true)
                .help("Extract all links from a webpage"),
        )
        .get_matches();

    file(&matches);
    num(&matches);
    req(&matches).await.unwrap();
}

fn file(matches: &ArgMatches) {
    let path = matches.value_of("file");
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

fn num(matches: &ArgMatches) {
    let num_str = matches.value_of("num");
    match num_str {
        None => {}
        Some(s) => match s.parse::<i32>() {
            Ok(n) => println!(
                "Your fav number must be: {}",
                Colour::Blue.paint((n + 5).to_string())
            ),
            Err(_) => println!("That's not a number!: {}", s),
        },
    }
}

async fn req(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let url = matches.value_of("req");
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
