use clap::{App, Arg};
fn main() {
    let mut app = App::new("Terminal App")
        .version("v1.0")
        .author("thelonehegelian")
        .about("Can parse cli arguments");

    let matches = app
        .arg(
            Arg::with_name("file")
                .long("file")
                .short('f')
                .takes_value(true)
                .help("A file to parse"),
        )
        .arg(
            Arg::with_name("number")
                .long("number")
                .short('n')
                .takes_value(true)
                .help("provide a number"),
        )
        .get_matches();

    // Method 1: its possible that no value was provided
    // let num_match = matches.value_of("number").unwrap_or("");
    // let num = num_match.parse::<i32>().unwrap();
    // println!("Just added 100 to your number: {}", num + 100);

    // Method 2
    let num_match = matches.value_of("number");
    // if there is no value
    match num_match {
        None => println!("No value provided"),
        // if a value is found
        Some(num) => match num.parse::<i32>() {
            // if the value is found then add 100 to it
            Ok(num) => println!("Just added 100 to your favorite number: {}", num + 100),
            Err(e) => println!("Error: {}", e),
        },
    }

    let file_match = matches.value_of("file").unwrap_or("No file provided");

    println!("{:?}", file_match);
}
