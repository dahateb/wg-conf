# wg-conf

build:

cargo build

run client:
cargo run client

```
wg-conf-client 

USAGE:
    wg-conf client [OPTIONS] -h <endpoint> -n <netmask>

FLAGS:
        --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --tls-ca-certificate <ca-cert>    root ca for use with tls
    -c, --config-file <config-file>       config file for wg-quick [default: examples/conf/conf.ini]
    -h <endpoint>                         Server endpoint to connect to [default: http://localhost:50051]
    -n <netmask>                          Netmask of the route to the VPN [default: 16]
```

run server:
cargo run server

```
wg-conf-server 

USAGE:
    wg-conf server [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --config-file <config-file>               config file for wg-quick
    -p <port>                                     port to listen on [default: 50051]
        --post-register-script <post-register>    shell script to run after register
        --pre-register-script <pre-register>      shell script before register
        --tls-certificate <tls-cert>              Server certificate for use with tls
        --tls-private-key <tls-key>               Server private keyfor use with tls
    -w <wg-port>                                  port wireguard to listens on [default: 51820]
```
