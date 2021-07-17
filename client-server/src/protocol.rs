use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::io::{Read, Write};
use std::net::TcpStream;

/// size of buffer used for tcp stream read/write operations
const BUF_SIZE: usize = 1024;

/// Possible commands the server can execute
#[derive(Serialize, Deserialize, Debug)]
pub enum Command {
    /// simple server ping, if alive, server will respond with pong
    Ping,

    /// increment the server's counter by the given amount
    Increment(i32),

    /// decrement the server's counter by the given amount
    Decrement(i32),

    /// get the current value of the server's counter
    Fetch,
}

/// Simple message protocol definition
#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    /// optional command, when present dictates server actions
    pub cmd: Option<Command>,

    /// houses any data that needs to be passed between client and server
    pub body: Option<String>,
}

impl Message {
    /// Serialize and return the current Message
    pub fn to_stream(&self, stream: &mut TcpStream) {
        let serialized = serde_json::to_string(&self).unwrap();

        stream
            .write_all(serialized.as_bytes())
            .expect("Couldn't send via socket");
    }

    /// Read up to `BUF_SIZE` bytes from `stream` and attempt to deserialize. If deserialization
    /// succeeds, the parsed `Message` is returned to the caller.
    pub fn from_stream(mut stream: &TcpStream) -> Message {
        // scratch buffer, defined on the stack
        let mut buf = [0; BUF_SIZE];

        // read up to BUF_SIZE bytes from the established connection
        let bytes_read = stream.read(&mut buf).expect("Couldn't read from socket");

        // grab a slice of bytes that should be json, discarding the remaining null bytes as they
        // jack up the deserialization call below
        let maybe_json = &buf[..bytes_read];

        // attempt to deserialize the bytes into a Message object and return it to the caller
        serde_json::from_slice(maybe_json).expect("Couldn't deserialize")
    }
}

impl Display for Message {
    /// allow for easy printing of either Message::body or Message::cmd, depending on which is
    /// set to an actual value
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // rhs values can be full blown expressions with blocks. our variable will be assigned based
        // on the result of the if/else statement (whether or not we set the `body` field of our
        // `Message`. Both arms of the match must 'return' the same type to be assigned
        let pretty = if self.body.is_some() {
            // this message's .body member is Some("..."), so we'll return the inner string to the
            // 'pretty' variable assignment
            self.body.as_ref().unwrap().to_string()
        } else {
            // this message's .cmd member is Some(Command::...), so we'll return the inner Command
            // as a string to the 'pretty' variable assignment
            format!("{:?}", self.cmd.as_ref().unwrap())
        };

        // write 'pretty' into the supplied output stream: `f`
        write!(f, "{}", pretty)
    }
}
