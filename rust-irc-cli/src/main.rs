use std::{
    io::{
        prelude::*,
        BufReader,
    },
    net::{
        TcpListener, 
        TcpStream,
    },
};
use threadpool::ThreadPool;
use clap::Parser;

#[derive(Parser)]
#[clap(name = "rust-irc-cli", version = "0.0.1", author = "Gr-an-t")]
struct Cli {
    #[arg(short, long, default_value = "127.0.0.1")]
    a: String,

    #[arg(short, long, default_value = "7878")]
    p: u16,

    #[arg(short, long, default_value = "4")]
    t: usize,
}

fn main() {
    let cli = Cli::parse();
    let address = format!("{}:{}", cli.a, cli.p);
    let listener = TcpListener::bind(&address).unwrap();
    let pool = ThreadPool::new(cli.t);
    println!("Starting server at {}", &address);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        
        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines().map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let response = "HTTP/1.1 200 OK\r\n\r\n";

    stream.write(response.as_bytes()).unwrap();

    println!("Request: {:?}", http_request);
}