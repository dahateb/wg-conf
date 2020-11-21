pub mod client;
pub mod server;

extern crate ini;
extern crate ipnetwork;
extern crate pretty_env_logger;
#[macro_use]
extern crate log;
extern crate base64;
extern crate clap;
extern crate rand_core;
extern crate url;
extern crate x25519_dalek;

use clap::{App, AppSettings, Arg, SubCommand};
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
            SubCommand::with_name("client")
                .arg(
                    Arg::with_name("endpoint")
                        .short("h")
                        .required(true)
                        .default_value("http://localhost:50051")
                        .help("Server endpoint to connect to")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("netmask")
                        .short("n")
                        .required(true)
                        .default_value("16")
                        .help("Netmask of the route to the VPN")
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("server")
                .arg(
                    Arg::with_name("port")
                        .short("p")
                        .default_value("50051")
                        .help("port to listen on")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("wg_port")
                        .short("w")
                        .default_value("51820")
                        .help("port wireguard to listens on")
                        .takes_value(true),
                ),
        )
        .version("0.0.1")
        .get_matches();

    match matches.subcommand() {
        ("client", Some(matches)) => {
            let endpoint = matches.value_of("endpoint").unwrap();
            let netmask = matches.value_of("netmask").unwrap();
            match start_client(endpoint, netmask) {
                Err(err) => error!("{}", err),
                _ => (),
            }
        }
        ("server", Some(matches)) => {
            let port = matches.value_of("port").unwrap_or("50051");
            let wg_port = matches.value_of("wg_port").unwrap_or("51820");
            match start_server("0.0.0.0", port, wg_port) {
                Err(err) => error!("{}", err),
                _ => (),
            }
        }
        _ => (),
    }
}
