use serde::{Deserialize, Serialize};
use ipmpsc::{Receiver, Sender};
pub struct Connection {
    pub rx: Receiver,
    pub tx: Sender,
}

impl Connection {
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