use clap::{Arg, Command};
use ff_carl::{write_entry, EntryArgs};
use std::fs;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = Command::new("ff-carl-cli")
        .version("1.0")
        .author("Dmytro Dyvulskyi <dmytro.dyvulskyi@medserv.ie>")
        .about("Manages Firefox mTLS ClientAuthRememberList.bin")
        .arg(
            Arg::new("scheme")
                .short('s')
                .long("scheme")
                .takes_value(true)
                .required(true)
                .help("The URL scheme (e.g., https)"),
        )
        .arg(
            Arg::new("host")
                .short('h')
                .long("host")
                .takes_value(true)
                .required(true)
                .help("The host (e.g., mtls.cert-demo.com)"),
        )
        .arg(
            Arg::new("port")
                .short('p')
                .long("port")
                .takes_value(true)
                .required(true)
                .help("The port (e.g., 443)"),
        )
        .arg(
            Arg::new("base_domain")
                .short('d')
                .long("domain")
                .takes_value(true)
                .required(true)
                .help("The base domain (e.g., cert-demo.com)"),
        )
        .arg(
            Arg::new("cert")
                .short('c')
                .long("cert")
                .takes_value(true)
                .required(true)
                .help("The path to the DER-encoded certificate"),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .takes_value(true)
                .required(true)
                .help("The path to the ClientAuthRememberList.bin file"),
        )
        .get_matches();

    let scheme = matches.value_of("scheme").unwrap();
    let host = matches.value_of("host").unwrap();
    let port: u32 = matches.value_of("port").unwrap().parse()?;
    let base_domain = matches.value_of("base_domain").unwrap();
    let cert_path = matches.value_of("cert").unwrap();
    let output_path = matches.value_of("output").unwrap();

    let der_cert = fs::read(cert_path)?;
    let entry_args = EntryArgs::new(
        scheme,
        host,
        port,
        base_domain,
        &der_cert,
    );

    let backing_path = PathBuf::from(output_path);

    write_entry(entry_args, backing_path)?;

    Ok(())
}
