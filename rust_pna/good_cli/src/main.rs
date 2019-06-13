#[macro_use]
extern crate clap;



#[cfg(feature = "yaml")]
fn main() {
    use clap::App;
    
    // The YAML file is found relative to the current file, similar to how modules are found
    let yml = load_yaml!("cli.yml");
    let m = App::from_yaml(yml).get_matches();

    let config = m.value_of("config").unwrap_or("default.conf");
    println!("Value for config: {}", config);

    println!("Using input file: {}", m.value_of("INPUT").unwrap());

    match m.occurrences_of("v") {
        0 => println!("No verbose info"),
        1 => println!("Some verbose info"),
        2 => println!("Tons of verbose info"),
        3 | _ => println!("Don't be crazy"),
    }

    if let Some(m) = m.subcommand_matches("test") {
        if m.is_present("debug") {
            println!("Printing debug info...");
        } else {
            println!("Printing normally...");
        }
    }
}

#[cfg(not(feature = "yaml"))]
fn main() {
    println!("YAML feature is disabled.");
    println!("Pass --features yaml to cargo when trying this example.");
}
