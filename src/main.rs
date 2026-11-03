use anyhow::Result;
use clap::Parser;
use lighttpd::lighttpd;

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    production: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let config = lighttpd::Config { 
        production_mode: cli.production, 
        ..Default::default() 
    };
    let mut component = lighttpd::new(config);
    
    component.initialize()?;
    component.start()?;
    
    println!("SigmaOS-lighttpd running");
    Ok(())
}
