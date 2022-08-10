use clap::{App, Arg};

fn main() {
    let matches = App::new("Cook CLI")
        .version("0.1.0")
        .author("Wafu")
        .about("A CLI for learning argument parsing")
        .arg(
            Arg::with_name("file")
                .short('f')
                .long("file")
                .takes_value(true)
                .help("A cool file"),
        )
        .arg(
            Arg::with_name("num")
                .short('n')
                .long("number")
                .takes_value(true)
                .help("Five less than your fav number"),
        )
        .get_matches();
    let myfile = matches.value_of("file").unwrap_or("input.txt");
    println!("The file passed in is: {}", myfile);

    let num_str = matches.value_of("num");
    match num_str {
        None => println!("No idea what your fav number is."),
        Some(s) => {
            match s.parse::<i32>() {
                Ok(n) => println!("Your fav number must be: {}", n + 5),
                Err(_) => println!("That's not a number!: {}", s),
            }
        } 
    }
}
