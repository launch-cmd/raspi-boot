use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
pub struct Opt {
    #[structopt(short = "v", long = "verbose")]
    pub verbose: bool,

    #[structopt(short = "w", long = "wifi-region")]
    pub wifi_region: String,

    #[structopt(short = "s", long = "ssid", default_value = "RASPI_AP")]
    pub ssid: String,

    #[structopt(short = "p", long = "password", default_value = "raspberry")]
    pub password: String,

    #[structopt(long = "ap-ip", default_value = "10.0.2.1")]
    pub ap_ip: String,

    #[structopt(long = "dhcp-start-ip", default_value = "10.0.2.2")]
    pub dhcp_start_ip: String,

    #[structopt(long = "dhcp-end-ip", default_value = "10.0.2.254")]
    pub dhcp_end_ip: String,

    #[structopt(long = "dhcp-subnet-mask", default_value = "255.255.255.0")]
    pub dhcp_subnet_mask: String,
}
