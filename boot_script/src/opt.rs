use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Opt {
    #[structopt(short = "v", long = "verbose")]
    pub verbose: bool,

    #[structopt(short = "m", long = "mac", about = "Client MAC address")]
    pub mac: String,

    #[structopt(
        short = "w",
        long = "wifi-region",
        about = "WiFi region used in wpa-supplicant.conf"
    )]
    pub wifi_region: String,

    #[structopt(short = "s", long = "ssid", about = "SSID to connect to")]
    pub ssid: String,

    #[structopt(short = "p", long = "password", about = "WiFi password")]
    pub password: String,

    #[structopt(long = "server-hostname", default_value = "server")]
    pub server_hostname: String,

    #[structopt(long = "client-hostname", default_value = "client")]
    pub client_hostname: String,

    #[structopt(long = "boot-server-ip", default_value = "10.0.1.1")]
    pub server_ip: String,

    #[structopt(long = "boot-client-ip", default_value = "10.0.1.2")]
    pub client_ip: String,
}
