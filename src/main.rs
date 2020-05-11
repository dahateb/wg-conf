pub mod client;
pub mod server;

extern crate pretty_env_logger;
extern crate ini;
extern crate ipnetwork;
#[macro_use]
extern crate log;
extern crate clap;
use clap::{App, Arg, SubCommand, AppSettings};
use client::start_client;
use server::start_server;

fn main() {
    pretty_env_logger::init();
    let matches = App::new("wg-conf")
        .version("0.0.1")
        .author("Dan H. ")
        .about("ip config tool for wireguard")        
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("client").arg(
                Arg::with_name("client_public_key")
                    .short("k")          
                    .required(true)      
                    .help("Public key of the client")
                    .takes_value(true),
            ).arg(
                Arg::with_name("endpoint")
                    .short("h")          
                    .required(true)      
                    .default_value("http://localhost:50051")
                    .help("Server endpoint to connect to")
                    .takes_value(true),
            )
        )
        .subcommand(
            SubCommand::with_name("server").arg(
                Arg::with_name("port")
                    .short("p")
                    .default_value("50051")
                    .help("port to listen on")
                    .takes_value(true),
            ),
        )
        .get_matches();

    match matches.subcommand() {
        ("client", Some(matches)) => {
            let public_key = matches.value_of("client_public_key").unwrap();
            let endpoint = matches.value_of("endpoint").unwrap();
            match start_client(endpoint, public_key) {
                Err(err) => error!("{}", err),
                _ => (),
            }
        },
        ("server", Some(matches)) => {
            let port = matches.value_of("port").unwrap_or("50051");
            match start_server("0.0.0.0", port) {
                Err(err) => error!("{}", err),
                _ => (),
            }
        }
        _ => ()
    }
}
