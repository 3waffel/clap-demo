use clap::{App, Arg};

pub fn build_cli() -> App<'static> {
    App::new("Cook CLI")
        .version(env!("CARGO_PKG_VERSION"))
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
            Arg::with_name("count")
                .short('c')
                .long("count")
                .takes_value(true)
                .help("Count the number"),
        )
        .arg(
            Arg::with_name("req")
                .short('r')
                .long("request")
                .takes_value(true)
                .help("Extract all links from a webpage"),
        )
        .arg(
            Arg::with_name("tree")
                .short('t')
                .long("tree")
                .takes_value(true)
                .help("Build a tree"),
        )
}
