pub mod client;
pub mod server;

extern crate ini;
extern crate ipnetwork;
extern crate pretty_env_logger;
#[macro_use]
extern crate log;
extern crate clap;
extern crate crypto;
extern crate hooks;
extern crate url;

#[macro_use]
extern crate simple_error;
extern crate http;

use clap::{App, AppSettings, Arg, SubCommand};
use client::start_client;
use server::start_server;

const VERSION: &str = "0.0.9";

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
                        .short("c")
                        .long("config-file")
                        .default_value("examples/conf/conf.ini")
                        .help("config file for wg-quick")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("ca-cert")
                        .long("tls-ca-certificate")
                        .help("root ca for use with tls")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("auth-token")
                        .long("auth-token")
                        .help("Auth Token to send to server. If using single token auth")
                        .conflicts_with_all(&["auth-user", "auth-password"])                        
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("auth-user")
                        .long("auth-user")
                        .help("Auth user to send to server. Use together with auth-password")
                        .requires("auth-password")
                        .conflicts_with("auth-token")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("auth-password")
                        .long("auth-password")
                        .help("Auth password to send to server")
                        .requires("auth-user")
                        .conflicts_with("auth-token")                        
                        .takes_value(true),
                )
                ,
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
                        .short("c")
                        .long("config-file")
                        .help("config file for wg-quick")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("pre-register")
                        .long("pre-register-script")
                        .help("shell script before register")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("post-register")
                        .long("post-register-script")
                        .help("shell script to run after register")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("tls-cert")
                        .long("tls-certificate")
                        .help("Server certificate for use with tls")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("auth-script")
                        .long("auth-script")
                        .help("shell script to validate auth info from client. Args will be passed as $1 $2")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("tls-key")
                        .long("tls-private-key")
                        .help("Server private keyfor use with tls")
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
            let ca_cert = matches.value_of("ca-cert");
            let auth_token = matches.value_of("auth-token");
            match start_client(endpoint, netmask, config_file, ca_cert, auth_token) {
                Err(err) => error!("{}", err),
                _ => (),
            }
        }
        ("server", Some(matches)) => {
            let port = matches.value_of("port").unwrap_or("50051");
            let wg_port = matches.value_of("wg-port").unwrap_or("51820");
            let config_file = matches
                .value_of("config-file")
                .unwrap_or("examples/conf/test.ini");
            let pre_register = matches.value_of("pre-register");
            let post_register = matches.value_of("post-register");
            let auth_script = matches.value_of("auth-script");
            let server_tls_cert = matches.value_of("tls-cert");
            let server_tls_key = matches.value_of("tls-key");
            match start_server(
                "0.0.0.0",
                port,
                wg_port,
                config_file,
                pre_register,
                post_register,
                auth_script,
                server_tls_cert,
                server_tls_key,
            ) {
                Err(err) => error!("{}", err),
                _ => (),
            }
        }
        _ => (),
    }
}
