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

const VERSION: &str = "0.0.4";

fn main() {
    pretty_env_logger::init();
    let matches = App::new("wg-conf")
        .version(VERSION)
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
                )
                .arg(
                    Arg::with_name("config-file")
                        .long("config-file")
                        .default_value("test.ini")
                        .help("config file for wg-quick")
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
                    Arg::with_name("wg-port")
                        .short("w")
                        .default_value("51820")
                        .help("port wireguard to listens on")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("config-file")
                        .long("config-file")
                        .help("config file for wg-quick")
                        .takes_value(true),
                ),
        )
        .version(VERSION)
        .get_matches();

    match matches.subcommand() {
        ("client", Some(matches)) => {
            let endpoint = matches.value_of("endpoint").unwrap();
            let netmask = matches.value_of("netmask").unwrap();
            let config_file = matches.value_of("config-file");
            match start_client(endpoint, netmask, config_file) {
                Err(err) => error!("{}", err),
                _ => (),
            }
        }
        ("server", Some(matches)) => {
            let port = matches.value_of("port").unwrap_or("50051");
            let wg_port = matches.value_of("wg-port").unwrap_or("51820");
            let config_file = matches.value_of("config-file").unwrap_or("test.ini");
            match start_server("0.0.0.0", port, wg_port, config_file) {
                Err(err) => error!("{}", err),
                _ => (),
            }
        }
        _ => (),
    }
}
