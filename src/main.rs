use anyhow::Result;
use clap::Parser;

#[allow(unused_imports)]
use raalog::{debug, error, info, trace, warn};

//  //  //  //  //  //  //  //
mod calculations;

#[derive(Parser, Debug)]
#[command(about)]
struct CliArgs {
    #[arg(short, long)]
    log: Option<String>,
    #[arg(short, long)]
    i_max: usize,
    #[arg(short, long)]
    j_max: usize,
    #[arg(short, long)]
    k_max: usize,
    #[arg(long)]
    actnum: Option<String>,
    #[arg(long, default_value = "bw.ascii")]
    bw: String,
    #[arg(long, default_value = "result.ascii")]
    result: String,
    #[arg(long, default_value = "-999")]
    undef_value: String,
    #[arg(long, default_value_t = 1.0)]
    k_mult: f64,
}

//  //  //  //  //  //  //  //
fn main() -> Result<()> {
    let args = CliArgs::parse();

    let log_file = interpret_log_file_name(args.log);
    log_init(&log_file);
    debug!("pwd: {:?}", std::env::current_dir()?);

    let status = calculations::invoke(
        args.i_max,
        args.j_max,
        args.k_max,
        args.actnum.as_ref().map(|s| &**s),
        &args.bw,
        &args.result,
        &args.undef_value,
        args.k_mult,
    );
    match status {
        Ok(()) => {
            trace!("############\n<-----\n.\n ");
        },
        Err(ref e) => {
            error!("############\nERROR!\n{}\n<-----\n.\n ", e.to_string());
        },
    };

    status
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
        }
        Some(s) => {
            if s == "EXE" {
                let mut log = std::env::current_exe().unwrap();
                log.pop();
                log.push("debug");
                log.set_extension("log");
                return log;
            }
            return s.into();
        }
    }
}
