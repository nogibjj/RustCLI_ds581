use clap::Parser;

# [derive(Parser)]

struct Cli {
    pattern: String,
}

fn main() { 

    let _args =  Cli::parse();

    println!("Hello, {}",(&_args.pattern))

}
