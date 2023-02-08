use clap::Parser;

mod converter;
mod api;


fn main() {
    let args = Cli::parse();

    let runtime = tokio::runtime::Runtime::new().unwrap();
    
    let response = runtime.block_on(converter::convert(&args.from, &args.to, &args.value));

    match response {
        Ok(value) => println!("{:.1$}", value, 2),
        Err(e) => println!("An error occurred: {}", e)
    }
}

#[derive(Parser)]
struct Cli {
    value: f64,

    #[arg(short = 'f', long)]
    from: String,

    #[arg(short = 't', long)]
    to: String,
}