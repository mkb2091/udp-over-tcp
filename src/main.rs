#[macro_use]
extern crate clap;

fn main() {
    let matches = clap::App::new("udp-over-tcp")
        .arg(
            clap::Arg::with_name("UDP-DEST-PORT")
                .help("Local port to send UDP packets")
                .required(true),
        )
        .arg(
            clap::Arg::with_name("UDP-BIND-PORT")
                .help("Local port to bind to and recieve UDP packets")
                .required(true),
        )
        .arg(
            clap::Arg::with_name("TCP-DEST-PORT")
                .help("Local TCP port to send TCP packets to")
                .required(true),
        )
        .arg(
            clap::Arg::with_name("TCP-BIND-PORT")
                .help("Local TCP port to bind to")
                .required(true),
        )
        .get_matches();
    let udp_dest_port: u16 = value_t!(matches, "UDP-DEST-PORT", u16).unwrap_or(0);
    let udp_bind_port: u16 = value_t!(matches, "UDP-BIND-PORT", u16).unwrap_or(0);
    let tcp_dest_port: u16 = value_t!(matches, "TCP-DEST-PORT", u16).unwrap_or(0);
    let tcp_bind_port: u16 = value_t!(matches, "TCP-BIND-PORT", u16).unwrap_or(0);
    let mut valid_ports = true;
    if udp_dest_port == 0 {
        println!(
            "Invalid port: {:}",
            matches.value_of("UDP-DEST-PORT").unwrap()
        );
        valid_ports = false;
    }
    if udp_bind_port == 0 {
        println!(
            "Invalid port: {:}",
            matches.value_of("UDP-BIND-PORT").unwrap()
        );
        valid_ports = false;
    }
    if tcp_dest_port == 0 {
        println!(
            "Invalid port: {:}",
            matches.value_of("TCP-DEST-PORT").unwrap()
        );
        valid_ports = false;
    }
    if tcp_bind_port == 0 {
        println!(
            "Invalid port: {:}",
            matches.value_of("TCP-BIND-PORT").unwrap()
        );
        valid_ports = false;
    }
    if valid_ports {
        println!("Program starting");
    }
}
