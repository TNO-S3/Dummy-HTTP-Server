use clap::Parser;
use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Port to listen on
    #[arg(short, long, default_value_t = 8080)]
    port: u16,

    /// Whether to print the full request contents
    #[arg(short, long, action)]
    verbose: bool,
}

/// Start the HTTP server. Exits on connectivity problems.
fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();

    println!(
        "Listening on port {} (specify on command line with --port <number>)",
        args.port
    );
    let listener = TcpListener::bind(("127.0.0.1", args.port))?;

    for stream in listener.incoming() {
        handle_connection(stream?, args.verbose)?;
    }
    Ok(())
}

/// Handle an incoming HTTP request, printing it to stdout and answering '200 OK'
fn handle_connection(mut stream: TcpStream, verbose: bool) -> Result<(), anyhow::Error> {
    let mut buf_reader = BufReader::new(&mut stream);
    let mut http_request = Vec::new();
    let mut content_length = 0;

    // Read the HTTP request headers into a Vec of lines. While doing so, look for a
    // Content-Length header, which would indicate that a body follows. Stop at an
    // empty line, which indicates the end of the request headers and possible start
    // of the body.
    loop {
        let mut line = String::new();
        buf_reader.read_line(&mut line)?;
        let trimmed_line = line.trim();
        if trimmed_line.is_empty() {
            break;
        }

        if trimmed_line
            .to_ascii_lowercase()
            .starts_with("content-length: ")
        {
            content_length = trimmed_line.split(':').nth(1).unwrap().trim().parse()?;
        }
        http_request.push(trimmed_line.to_owned());
    }

    if http_request.is_empty() {
        println!("<empty request>");
    } else {
        // Print the request's first line, and if '--verbose', all other lines too
        if verbose {
            println!("Request: {:#?}\n", http_request);
        } else {
            println!("Request: {}", http_request[0]);
        }

        // If body content was announced by the Content-Length header, read and print it
        if content_length > 0 {
            println!("Body:");
            let mut body = vec![0u8; content_length];
            buf_reader.read_exact(&mut body)?;
            println!("{}\n", String::from_utf8_lossy(&body));
        }
    }

    // Respond "200 OK" to all requests
    let response = "HTTP/1.1 200 OK\r\n\r\n";
    stream.write_all(response.as_bytes())?;
    Ok(())
}
