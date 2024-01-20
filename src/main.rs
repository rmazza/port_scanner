use clap::Parser;
use std::net::{TcpStream, ToSocketAddrs};
use std::time::Duration;
use rayon::prelude::*;
use rayon::ThreadPoolBuilder;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// host ip to connect to
    #[arg(short, long)]
    ip: String,
    /// port to check
    #[arg(short, long, default_value = "-1")]
    port: String,
    /// number of threads to use during a vanilla scan
    #[arg(short, long, default_value = "8")]
    number_of_threads: usize
}

impl Cli {
    fn url(&self) -> String {
        format!("{}:{}", self.ip, self.port)
    }
}


fn main() {

    let args = Cli::parse();
    let vanilla_scan = args.port == "-1";
    
    if vanilla_scan {
        let num_threads = args.number_of_threads; // Set the desired number of threads

        // Build a thread pool with a specific number of threads
        let pool = ThreadPoolBuilder::new().num_threads(num_threads).build().unwrap();
        pool.install(|| {
            (1..=65535).into_par_iter().for_each(|i| {
                check_if_open(format!("{}:{}", args.ip, i))
            });
        });
    } else {
        check_if_open(args.url());
    }
}

fn check_if_open(address: String) {
    let addrs = match address.to_socket_addrs() {
        Ok(addrs) => addrs,
        Err(e) => {
            println!("Failed to resolve address {}: {}", address, e);
            return;
        }
    };

    for addr in addrs {
        let timeout = Duration::new(0, 1000);
        if let Ok(_stream) = TcpStream::connect_timeout(&addr, timeout) {
            println!("{} is open", addr);
            return;
        }
    }
}