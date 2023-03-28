use clap::{App, Arg};
use std::io::prelude::*;
use std::os::unix::net::UnixStream;
use std::time::Instant;
use rust_ipc_examples::print_latency;

const TEST_NUM: u64 = 100000;

fn main() -> std::io::Result<()> {
    let matches = client_args();
    let path = matches.value_of("socket name").unwrap();
    let mut stream = UnixStream::connect(path)?;

    println!("ping pong test with hello world\n");
    let begin = Instant::now();
    for _ in 0..TEST_NUM {
        stream.write_all(b"hello world")?;
        let mut response = [0; 12];
        let len = stream.read(&mut response)?;
        assert_eq!(b"hello", &response[..len]);
    }
    print_latency(begin, TEST_NUM);

    println!("--------------------------------------------");
    println!("ping pong test with 8192 bytes vec\n");
    let begin = Instant::now();
    for _ in 0..TEST_NUM {
        stream.write_all(&vec![0u8; 8192])?;
        let mut buf = vec![0u8; 8192];
        stream.read_exact(&mut buf)?;
        assert_eq!(vec![1u8; 8192], buf);
    }
    print_latency(begin, TEST_NUM);
    Ok(())
}

fn client_args() -> clap::ArgMatches {
    App::new("uds_client")
        .about("uds client example")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .arg(
            Arg::with_name("socket name")
                .help(
                    "File to use for uds connection.  \
                     This should have already been created and initialized by the server.",
                )
                .required(true),
        )
        .get_matches()
}
