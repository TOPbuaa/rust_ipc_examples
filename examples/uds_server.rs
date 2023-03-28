use clap::{App, Arg};
use std::io::prelude::*;
use std::os::unix::net::UnixListener;

const TEST_NUM: u64 = 100000;

fn main() -> std::io::Result<()> {
    let matches = server_args();
    let path = matches.value_of("socket name").unwrap();
    if let Err(_) = std::fs::remove_file(path) {
        eprintln!("no file need to be clean.");
    }
    let listener = UnixListener::bind(path)?;

    let (mut stream, _) = listener.accept()?;

    for _ in 0..TEST_NUM {
        let mut request = [0; 12];
        let len = stream.read(&mut request)?;
        assert_eq!(b"hello world", &request[..len]);
        stream.write_all(b"hello")?;
    }

    for _ in 0..TEST_NUM {
        let mut buf = vec![1u8; 8192];
        stream.read_exact(&mut buf)?;
        assert_eq!(vec![0u8; 8192], buf);
        stream.write_all(&vec![1u8; 8192])?;
    }
    Ok(())
}

fn server_args() -> clap::ArgMatches {
    App::new("uds_server")
    .about("ipmpsc sender example")
    .version(env!("CARGO_PKG_VERSION"))
    .author(env!("CARGO_PKG_AUTHORS"))
    .arg(
        Arg::with_name("socket name")
            .help(
                "File to use for uds connection.  \
                 This file will be cleared if it already exists or created if it doesn't.",
            )
            .required(true),
    )
    .get_matches()
}