fn print_usage() {
    println!("usage: osucollectrs [id]")
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let collector_id = match args.len() {
        1 => {
            print_usage();
            std::process::exit(1)
        }
        2 => &args[1],
        _ => {
            println!("Too many arguments!");
            print_usage();
            std::process::exit(1)
        }
    };

    if let Err(_) = &collector_id.parse::<usize>() {
        println!("Invalid osu!collector id! has to be a number");
        print_usage();
        std::process::exit(1)
    }

    println!("Collector id: {}", collector_id);
}
