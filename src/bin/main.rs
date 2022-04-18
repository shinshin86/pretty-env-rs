use std::env;

fn display_usage() {
    println!(
        "Usage: pretty-env [raw | csv]

Pretty display
$ pretty-env

Raw display
$ pretty-env raw

CSV display
$ pretty-env csv
"
    )
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let mut option = "";

    if args.len() == 2 {
        option = &args[1];
    } else if args.len() > 2 {
        display_usage();
        std::process::exit(0);
    }

    let valid_options = ["raw", "csv"];
    if option == "help" || !valid_options.contains(&option) {
        display_usage();
        std::process::exit(0);
    }

    pretty_env::pretty_env(option);
}
