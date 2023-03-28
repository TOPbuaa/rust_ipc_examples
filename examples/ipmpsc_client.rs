use clap::{App, Arg};
use ipmpsc::{Receiver, Sender, SharedRingBuffer};
use rust_ipc_examples::{IpmpscConnection, print_latency};
use std::time::Instant;

const TEST_NUM: u64 = 100000;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = client_args();
    let mut conn = create_client_conn(&matches)?;
    println!("ping pong test with hello world\n");
    pingpong(&mut conn)?;
    println!("--------------------------------------------");
    println!("ping pong test with 8192 bytes vec\n");
    pingpong_large(&mut conn)?;
    Ok(())
}

fn pingpong(conn: &mut IpmpscConnection) -> Result<(), Box<dyn std::error::Error>> {
    // ping pong test
    let begin = Instant::now();
    for _ in 0..TEST_NUM {
        conn.send(&"hello world".to_owned())?;
        let received = conn.recv::<String>()?;
        assert_eq!("hello", received);
    }
    print_latency(begin, TEST_NUM);

    // ping pong test (zero copy)
    let begin = Instant::now();
    for _ in 0..TEST_NUM {
        conn.send(&"hello world".to_owned())?;
        assert_eq!("hello", conn.rx.zero_copy_context().recv::<&str>()?); //zero copy
    }
    println!("zero copy: ");
    print_latency(begin, TEST_NUM);

    // ping pong test (busy polling)
    let begin = Instant::now();
    for _ in 0..TEST_NUM {
        conn.send(&"hello world".to_owned())?;
        let received = conn.recv_busy_poll::<String>()?;
        assert_eq!("hello", received);
    }
    println!("polling: ");
    print_latency(begin, TEST_NUM);

    // ping pong test (zero copy & busy polling)
    let begin = Instant::now();
    for _ in 0..TEST_NUM {
        conn.send(&"hello world".to_owned())?;
        loop {
            if let Some(received) = conn.rx.zero_copy_context().try_recv::<&str>()? {
                assert_eq!("hello", received);
                break;
            }
        }
    }
    println!("polling & zero copy: ");
    print_latency(begin, TEST_NUM);
    Ok(())
}

fn pingpong_large(conn: &mut IpmpscConnection) -> Result<(), Box<dyn std::error::Error>> {
    let buf = vec![0u8; 8192];
    // ping pong test
    let begin = Instant::now();
    for _ in 0..TEST_NUM {
        conn.send(&buf)?;
        let received = conn.recv::<Vec<u8>>()?;
        assert_eq!(8192, received.len());
    }
    print_latency(begin, TEST_NUM);

    // ping pong test (zero copy)
    let begin = Instant::now();
    for _ in 0..TEST_NUM {
        conn.send(&buf)?;
        assert_eq!(8192, conn.rx.zero_copy_context().recv::<Vec<u8>>()?.len()); //zero copy
    }
    println!("zero copy: ");
    print_latency(begin, TEST_NUM);

    // ping pong test (busy polling)
    let begin = Instant::now();
    for _ in 0..TEST_NUM {
        conn.send(&buf)?;
        let received = conn.recv_busy_poll::<Vec<u8>>()?;
        assert_eq!(8192, received.len());
    }
    println!("polling: ");
    print_latency(begin, TEST_NUM);

    // ping pong test (zero copy & busy polling)
    let begin = Instant::now();
    for _ in 0..TEST_NUM {
        conn.send(&buf)?;
        loop {
            if let Some(received) = conn.rx.zero_copy_context().try_recv::<Vec<u8>>()? {
                assert_eq!(8192, received.len());
                break;
            }
        }
    }
    println!("polling & zero copy: ");
    print_latency(begin, TEST_NUM);
    Ok(())
}

fn client_args() -> clap::ArgMatches {
    App::new("ipmpsc-send")
        .about("ipmpsc sender example")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .arg(
            Arg::with_name("map file")
                .help(
                    "File to use for shared memory ring buffer.  \
                     This should have already been created and initialized by the receiver.",
                )
                .required(true),
        )
        .get_matches()
}

fn create_client_conn(
    matches: &clap::ArgMatches,
) -> Result<IpmpscConnection, Box<dyn std::error::Error>> {
    let map_file = matches.value_of("map file").unwrap();

    let s2c_map_file = &format!("{}_s2c", map_file); // server to client
    let c2s_map_file = &format!("{}_c2s", map_file); // client to server

    let rx = Receiver::new(SharedRingBuffer::open(s2c_map_file)?);
    let tx = Sender::new(SharedRingBuffer::open(c2s_map_file)?);
    Ok(IpmpscConnection { rx, tx })
}
