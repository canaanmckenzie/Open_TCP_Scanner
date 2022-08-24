use std::env;
use anyhow::Result;
use clap::{Arg,Command};

mod cli;
mod dns;
mod error;
pub use error::Error;
mod ports;
mod modules;
mod common_ports;


fn main() -> Result<()>{

    //logger
    env::set_var("RUST_LOG","info,trust_dns_proto=error");
    env_logger::init();

   let cli = Command::new(clap::crate_name!())
    .version(clap::crate_version!())
    .about(clap::crate_description!())
    .subcommand(Command::new("modules").about("List all modules"))
    .subcommand(
        Command::new("scan").about("Scan a target").arg(
            Arg::new("target")
                 .help("The domain name to scane")
                 .required(true)
                 .index(1),
        ),
    )
    .arg_required_else_help(true)
    .get_matches();

    if let Some(_) = cli.subcommand_matches("modules"){
        cli::modules();
    } else if let Some(matches) = cli.subcommand_matches("scan"){
        //safely unwrap as arg is reqd
        let target = matches.value_of("target").unwrap();
        cli::scan(target)?;
    }
    Ok(())
}
