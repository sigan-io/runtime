use tokio::net::UnixStream;

pub struct Connection {
    stream: UnixStream,
}

impl Connection {
    pub fn new(stream: UnixStream) -> Self {
        Self { stream }
    }

    pub fn get_stream(&self) -> &UnixStream {
        &self.stream
    }

    // pub fn send(&mut self, data: &[u8]) -> io::Result<()> {
    //     self.stream.write_all(data)?;
    //     Ok(())
    // }

    // pub fn receive(&mut self) -> io::Result<Vec<u8>> {
    //     let mut buffer = Vec::new();
    //     self.stream.read_to_end(&mut buffer)?;
    //     Ok(buffer)
    // }
}
