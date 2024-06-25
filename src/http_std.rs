// Steps:
// run Http::new(), will return fut
// then, keep poll()-ing the fut until the state is Poll::Ready(data)
// first poll() sends the TCP request
// later poll()s waits for the response from Server
use std::{
    io::{ErrorKind, Read, Write},
    net::TcpStream,
};
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

fn get_req(path: &str) -> String {
    format!(
        "GET {path} HTTP/1.1\r\n\
             Host: localhost\r\n\
             Connection: close\r\n\
             \r\n"
    )
}

pub struct Http;

// Returns a type that implements a Future
impl Http {
    pub fn new(path: &str) -> impl Future<Output = String> {
        HttpFuture::new(path)
    }
}

// HttpFuture is a Leaf Future
// implements a Future, so it is a Future type
// and to get the data from the Future, we do poll()
struct HttpFuture {
    stream: Option<mio::net::TcpStream>,
    buffer: Vec<u8>,
    path: String,
}

impl HttpFuture {
    fn new(path: &str) -> Self {
        Self {
            stream: None,
            buffer: vec![],
            path: String::from(path),
        }
    }
    fn write_request(&mut self) {
        let stream = TcpStream::connect("localhost:8080").unwrap();
        stream.set_nonblocking(true).unwrap();

        let mut stream = mio::net::TcpStream::from_std(stream);
        let buf = get_req(&self.path);
        println!("{buf}");
        stream.write_all(buf.as_bytes()).unwrap();
        self.stream = Some(stream);
    }
}


// Using Standard Library Future Trait
impl Future for HttpFuture {
    // Associated type is set to String
    type Output = String;
    fn poll(mut self: Pin<&mut HttpFuture>, _ctx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.stream.is_none() {
            self.write_request();
            return Poll::Pending;
        }
        let mut buf = [0u8; 1024];
        // EVENT Loop
        // Runtime Reactor will replace this loop. (?)
        loop {
            match self.stream.as_mut().unwrap().read(&mut buf) {
                // Ok() means DATA HAS ARRIVED

                // zero n_bytes
                Ok(0) => {
                    let string = String::from_utf8_lossy(&self.buffer);
                    break Poll::Ready(string.to_string());
                }
                // n bytes received from TCP read()
                // if response bytes are nonempty, n > 0, then store it in the buffer
                Ok(n_bytes) => {
                    self.buffer.extend(&buf[0..n_bytes]);
                    continue;
                }

                // Err() means DATA ARRIVAL IS STILL PENDING or FAILED

                // rerun the loop if there were some OS Interrupt
                Err(e) if e.kind() == ErrorKind::Interrupted => {
                    continue;
                }

                // WOULDBLOCK means Wait state
                // For "reads", EWOULDBLOCK says "there isn't any data". It's saying "if this were 'normal I/O', then I'd block".
                Err(e) if e.kind() == ErrorKind::WouldBlock => {
                    break Poll::Pending;
                }
                Err(e) => {
                    panic!("{e}")
                }
            }
        }
    }
}
