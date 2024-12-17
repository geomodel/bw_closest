use anyhow::Result;
use clap::Parser;

fn main() -> Result<()> {
    println!("parser intro");
    let args = CliArgs::parse();
    println!("{} -> {}", args.commandName, args.value);
    println!("{} -> {}", args.uflA, args.wflB);

    Ok(())
}

//  //  //  //  //  //  //  //
#[derive(Parser, Debug)]
#[command(about)]
struct CliArgs {
    #[arg(short, long, default_value_t = false)]
    uflA: bool,
    #[arg(short, long, default_value_t = false)]
    wflB: bool,
    #[arg(short, long)]
    commandName: String,
    #[arg(short, long, default_value_t = 0)]
    value: u8,
}
