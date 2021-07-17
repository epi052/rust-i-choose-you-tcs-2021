use std::net::{TcpListener, TcpStream};
use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::Arc;
use std::thread;

use client_server::protocol::{Command, Message};

/// Process established connections to the server and execute tasks based on the message sent
///
/// `stream` defined as mutable for internal state tracking, even during reads
fn handle_connection(id: usize, mut stream: TcpStream, counter: Arc<AtomicI32>) {
    // pass stream as a reference to parse_message. parse_message "borrows" the stream for a bit
    // but gives ownership back to handle_connection once complete
    // let msg = parse_message(&stream);
    let msg = Message::from_stream(&stream);

    // Message read and deserialized properly, now we can build the default message, which is
    // to send back 'success', more specific messages may alter the message
    let mut response = Message {
        cmd: None,
        body: Some("success".to_string()),
    };

    // now we can switch on the given Command and act accordingly
    match msg.cmd {
        Some(Command::Ping) => {
            // simple ping/pong connectivity test
            response.body = Some("pong".to_string());
        }
        Some(Command::Increment(val)) => {
            // atomically add the given value to the counter
            counter.fetch_add(val, Ordering::SeqCst);
        }
        Some(Command::Decrement(val)) => {
            // atomically subtract the given value from the counter
            counter.fetch_sub(val, Ordering::SeqCst);
        }
        Some(Command::Fetch) => {
            // atomically retrieve the current value and return it in the response body
            response.body = Some(format!("{}", counter.load(Ordering::SeqCst)));
        }
        _ => {} // all other possibilities for the match statement; do nothing
    }

    println!("[{:7}] received {}; replying with {}", id, msg, response);

    // send serialized response back over the established connection
    response.to_stream(&mut stream);
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:4444").expect("Couldn't bind port");

    // `counter` is the server's internal counter.
    //
    // An Arc is a thread-safe reference-counting pointer.
    // 'Arc' stands for 'Atomically Reference Counted'. Arc uses atomic operations for its
    // reference counting and is thread-safe. It allows us to share immutable data across threads.
    // The reason for needing the Arc type when attempting to share data across threads is to
    // ensure that the lifetime of the type that is being shared, lives as long as the longest
    // lasting thread.
    //
    // AtomicUsize is an integer type which can be safely shared between threads
    //
    // the use of these two types together means we'll have a threaded server that manipulates
    // shared data, but is free of data races.
    let counter = Arc::new(AtomicI32::new(0));

    // loop over each incoming connection, calling handle_connection for each in turn
    for (id, stream) in listener.incoming().enumerate() {
        // in the for loop definition, stream is of the type Result<TcpStream>
        // .expect() (among other methods) consumes the Result, giving us the
        // underlying TcpStream. .expect() will panic if anything goes wrong
        let stream = stream.expect("Couldn't accept connection");

        // creating a new reference from an existing reference-counted pointer is done using
        // .clone(). An Arc is on the heap, and calling .clone() gives us another pointer to the
        // data on the heap. Calling .clone() on an Arc is a relatively cheap operation.
        let per_thread_ref = counter.clone();

        // `move` captures stream by-value, moving it into this thread. It does the same to
        // `per_thread_ref`, but since it's a clone of an Arc, we won't run into any ownership
        // problems when reusing `counter` in a loop.
        thread::spawn(move || {
            // the `move || {}` syntax seen here is an example of a closure in rust
            handle_connection(id, stream, per_thread_ref);
        });
    }
}
