use anyhow::Result;
use clap::Parser;

#[allow(unused_imports)]
use raalog::{debug, error, info, trace, warn};

//  //  //  //  //  //  //  //
fn main() -> Result<()> {
    let args = CliArgs::parse();

    let log_file = interpret_log_file_name(args.log);
    log_init(&log_file);
    trace!("pwd: {:?}", std::env::current_dir()?);

    trace!("############\n<-----\n.\n ");
    Ok(())
}

//  //  //  //  //  //  //  //
fn log_init(log_file: &std::path::Path) {
    raalog::init()
        .expect("unable init log system")
        .set_file_mode(&log_file)
        .expect("unable to set file mode of logger")
        .set_level(raalog::LevelFilter::Trace);

    trace!("\n.\n----->\n############");
    set_panic_hook();
}

fn set_panic_hook() {
    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        error!("############\nFATAL!\n{}\n<-----\n.\n ", info);
        hook(info);
    }));
}

//  //  //  //  //  //  //  //
fn interpret_log_file_name(arg: Option<String>) -> std::path::PathBuf {
    match arg {
        None => {
            let mut log = std::env::current_dir().unwrap();
            log.push("debug");
            log.set_extension("log");
            return log;
        },
        Some(s) => {
            if s == "EXE" {
                let mut log = std::env::current_exe().unwrap();
                    log.pop();
                    log.push("debug");
                    log.set_extension("log");
                return log;
            }
            return s.into();
        },
    }
}


//  //  //  //  //  //  //  //
#[derive(Parser, Debug)]
#[command(about)]
struct CliArgs {
    /*
    #[arg(short, long, default_value_t = false)]
    uflA: bool,
    #[arg(short, long, default_value_t = false)]
    wflB: bool,
    #[arg(short, long)]
    commandName: String,
    #[arg(short, long, default_value_t = 0)]
    value: u8,
    */
    #[arg(short, long)]
    log: Option<String>,
}
