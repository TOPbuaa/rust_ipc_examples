use clap::{App, Arg};
use ipmpsc::{Receiver, Sender, SharedRingBuffer};
use rust_ipc_examples::IpmpscConnection;

const TEST_NUM: u64 = 100000;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = server_args();
    let mut conn = create_server_conn(&matches)?;
    handle_pingpong(&mut conn)?;
    handle_pingpong_large(&mut conn)?;
    Ok(())
}

fn handle_pingpong(conn: &mut IpmpscConnection) -> Result<(), Box<dyn std::error::Error>> {
    for _ in 0..TEST_NUM {
        let received =  conn.recv::<String>()?;
        assert_eq!("hello world", received);
        conn.send(&"hello".to_owned())?;
    }

    for _ in 0..TEST_NUM {
        assert_eq!("hello world", conn.rx.zero_copy_context().recv::<&str>()?); //zero copy
        conn.send(&"hello".to_owned())?;
    }

    for _ in 0..TEST_NUM {
        let received =  conn.recv_busy_poll::<String>()?;
        assert_eq!("hello world", received);
        conn.send(&"hello".to_owned())?;
    }

    for _ in 0..TEST_NUM {
        loop {
            if let Some(received) =  conn.rx.zero_copy_context().try_recv::<&str>()? {
                assert_eq!("hello world", received);
                break;
            }
        }
        conn.send(&"hello".to_owned())?;
    }
    Ok(())
}

fn handle_pingpong_large(conn: &mut IpmpscConnection) -> Result<(), Box<dyn std::error::Error>> {
    let buf = vec![0u8; 8192];
    for _ in 0..TEST_NUM {
        let received =  conn.recv::<Vec<u8>>()?;
        assert_eq!(8192, received.len());
        conn.send(&buf)?;
    }

    for _ in 0..TEST_NUM {
        assert_eq!(8192, conn.rx.zero_copy_context().recv::<Vec<u8>>()?.len()); //zero copy
        conn.send(&buf)?;
    }

    for _ in 0..TEST_NUM {
        let received =  conn.recv_busy_poll::<Vec<u8>>()?;
        assert_eq!(8192, received.len());
        conn.send(&buf)?;
    }

    for _ in 0..TEST_NUM {
        loop {
            if let Some(received) =  conn.rx.zero_copy_context().try_recv::<Vec<u8>>()? {
                assert_eq!(8192, received.len());
                break;
            }
        }
        conn.send(&buf)?;
    }
    Ok(())
}

fn server_args() -> clap::ArgMatches {
    App::new("ipmpsc-send")
    .about("ipmpsc sender example")
    .version(env!("CARGO_PKG_VERSION"))
    .author(env!("CARGO_PKG_AUTHORS"))
    .arg(
        Arg::with_name("map file")
            .help(
                "File to use for shared memory ring buffer.  \
                 This file will be cleared if it already exists or created if it doesn't.",
            )
            .required(true),
    )
    .arg(
        Arg::with_name("zero copy")
            .long("zero-copy")
            .help("Use zero-copy deserialization"),
    )
    .get_matches()
}

fn create_server_conn(matches: &clap::ArgMatches) -> Result<IpmpscConnection, Box<dyn std::error::Error>> {
    let map_file = matches.value_of("map file").unwrap();
    let s2c_map_file = &format!("{}_s2c", map_file); // server to client
    let c2s_map_file = &format!("{}_c2s", map_file); // client to server
    let rx = Receiver::new(SharedRingBuffer::create(c2s_map_file, 512 * 1024)?);
    // let zero_copy = matches.is_present("zero copy");

    let _ = SharedRingBuffer::create(s2c_map_file, 512 * 1024)?;
    let tx = Sender::new(SharedRingBuffer::open(s2c_map_file)?);

    println!(
        "Ready!  Now run `cargo run --example client {}` in another terminal.",
        map_file
    );
    Ok(IpmpscConnection { rx, tx })
}