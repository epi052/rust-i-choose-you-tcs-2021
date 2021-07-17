use std::net::TcpStream;

use clap::{App, Arg}; // command line parsing
use rand::Rng; // random number generation

// a prelude is a rust convention that groups the most commonly used parts of a library into one
// convenient location. The syntax below is a glob import of the entire prelude.
use pyo3::prelude::*; // foreign function interface for python
use rayon::prelude::*; // parallel execution // rust/python

use client_server::protocol::{Command, Message}; // our internal protocol

/// parse command line agrument `-n` and return its value as `usize`
fn get_number_of_connections() -> usize {
    // define a new application that accepts a single argument using the clap crate
    let app = App::new("client").arg(
        Arg::with_name("num_connections")
            .short("n")
            .help("Number of connections to spawn (default: 30)")
            .takes_value(true)
            .default_value("30"),
    );

    // perform the actual parsing
    let matches = app.get_matches();

    // we provide a default to Arg; this will always have a value/can't fail
    let conns = matches.value_of("num_connections").unwrap();

    // try to parse &str as the variable's specified type (usize)
    let conns_as_usize: usize = conns.parse().expect("Couldn't cast -n value to usize");

    // if the casting operation was successful, return the parsed/casted value
    conns_as_usize
}

/// given a unique id, create a new connection to the companion server and send a randomly selected
/// Command
fn spawn_connection(id: usize) {
    // establish connection to the server
    let mut client = TcpStream::connect("127.0.0.1:4444").expect("Couldn't connect to server");

    // create thread-local random number generator, seeded by the system
    let mut rng = rand::thread_rng();

    // generate a random value in the given range, this value is only used when the randomized
    // action is increment or decrement
    let val = rng.gen_range(0..1000);

    // rhs values can be full blown expressions with blocks. Below is a match expression that
    // 'returns' a `Command` based on a random value supplied by `rng.gen_range`. The result of
    // the match expression is stored in `action`
    let action = match rng.gen_range(0..=3) {
        0 => Command::Ping,
        1 => Command::Increment(val),
        2 => Command::Decrement(val),
        3 => Command::Fetch,
        // the fact that gen_range() is constrained to produce values 0-3 inclusive is not known to
        // the compiler (more likely, it's not willing to admit it knows it); as far as it's
        // willing to prove, gen_range can have any value from 0 to 2^32-1. The `_` match arm
        // seen below covers all of those other possibilities, as match statements must be
        // exhaustive. Since WE know this can't happen, we'll raise an exception, halting execution
        // if it does
        _ => panic!("gen_range returned an unexpected value"),
        // side-note: there are more elegant ways of handling randomly selecting an enum variant,
        // but are harder to digest from a 'learning rust' perspective. In the event you want to
        // explore implementing a more robust solution, you could implement the Distribution trait
        // from the rand crate on the Command enum
        //
        // https://docs.rs/rand/0.8.4/rand/distributions/trait.Distribution.html
    };

    // use the random action to create a Message
    let msg = Message {
        cmd: Some(action),
        body: None,
    };

    // send the message over the established connection
    msg.to_stream(&mut client);

    // and then read the reply
    let response = Message::from_stream(&client);

    println!("[{:7}] sent {}; received {}", id, msg, response);
}

fn main() {
    // parse -n from the command line and return the number of connections
    let num_conns = get_number_of_connections();

    // acquire CPython's infamous Global Interpreter Lock, which prevents several threads
    // from executing Python bytecode in parallel
    let gil = Python::acquire_gil();
    let python = gil.python();

    // import the `rich` python library; rich needs to be installed in our current python env
    let rich = python.import("rich").expect("Couldn't import rich");

    // get a reference to rich.print(), so we can use rich's markup language and format output
    let rich_print = rich.getattr("print").expect("Couldn't get print function");

    // call rich.print
    rich_print
        .call(("[green][+][/green] beginning parallel execution",), None)
        .expect("Couldn't print with rich");

    // temporarily release the GIL
    python.allow_threads(move || {
        // use the rayon library for incredibly simple parallel execution with a high-level iterator
        // style interface. `i` in the expression below is simply the values from 0 to `num_conns`
        // being passed to the `for_each` block
        (0..num_conns).into_par_iter().for_each(|i| {
            spawn_connection(i);
        });
    });
    // GIL reacquired at this point

    rich_print
        .call(("[green][+][/green] parallel execution complete",), None)
        .expect("Couldn't print with rich");
}
