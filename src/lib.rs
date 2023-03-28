use serde::{Deserialize, Serialize};
use ipmpsc::{Receiver, Sender};
use std::time::Instant;

pub struct IpmpscConnection {
    pub rx: Receiver,
    pub tx: Sender,
}

impl IpmpscConnection {
    pub fn send(&mut self, value: &impl Serialize) -> Result<(), Box<dyn std::error::Error>> {
        self.tx.send(value)?;
        Ok(())
    }

    pub fn recv<T>(&mut self) -> Result<T, Box<dyn std::error::Error>>
    where
    T: for<'de> Deserialize<'de>,
    {
        let received = self.rx.recv::<T>()?;
        Ok(received)
    }

    pub fn recv_busy_poll<T>(&mut self) -> Result<T, Box<dyn std::error::Error>>
    where
    T: for<'de> Deserialize<'de>,
    {
        loop {
            if let Some(received) = self.rx.try_recv::<T>()? {
                return Ok(received);
            }
        }
    }
}

pub fn print_latency(begin: Instant, test_num: u64) {
    let end = Instant::now();
    println!(
        "Ping-pong {} times use {} us, {} us for each time.\n",
        test_num,
        end.duration_since(begin).as_micros(),
        end.duration_since(begin).as_micros() as f64 / test_num as f64
    );
}