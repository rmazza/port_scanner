use clap::Parser;
use std::net::{TcpStream, ToSocketAddrs, SocketAddr};
use std::time::Duration;
use rayon::prelude::*;
use rayon::ThreadPoolBuilder;

/// Network port scanner
#[derive(Parser)]
struct Cli {
    /// host ip to connect to
    #[arg(short = 'H', long, use_value_delimiter = true, value_delimiter = ',', required = true)]
    hosts: Vec<String>,
    /// port to check
    #[arg(short, long, default_value = "-1")]
    port: String,
    /// number of threads to use during a vanilla scan
    #[arg(short, long, default_value = "8")]
    number_of_threads: usize
}

impl Cli {
    fn get_socket_addrs(self) -> Vec<SocketAddr> {
        self.hosts
        .into_iter()
        .flat_map(|addr| { 
            let full_addr = format!("{}:{}", addr, self.port);
            full_addr.to_socket_addrs().ok()
        })
        .flatten()
        .collect::<Vec<SocketAddr>>()
    }

    fn get_socket_addrs_with_port(&self, port: i32) -> Vec<SocketAddr> {
        self.hosts
        .iter()
        .flat_map(|addr| { 
            let full_addr = format!("{}:{}", addr, port);
            full_addr.to_socket_addrs().ok()
        })
        .flatten()
        .collect::<Vec<SocketAddr>>()
    }
}

fn main() {

    let args = Cli::parse();
    let vanilla_scan = args.port == "-1";
    
    if vanilla_scan {
        let num_threads = args.number_of_threads; // Set the desired number of threads

        // Build a thread pool with a specific number of threads
        let pool = ThreadPoolBuilder::new()
                                                .num_threads(num_threads)
                                                .build()
                                                .unwrap_or_else(|e| {
                                                    eprintln!("Failed to create a thread pool: {}", e);
                                                    std::process::exit(1);
                                                });
        pool.install(|| {
            (1..=65535).into_par_iter().for_each(|i| {
                scan_socket_addresses(args.get_socket_addrs_with_port(i));
            });
        });
    } else {
        scan_socket_addresses(args.get_socket_addrs());
    }
}

fn scan_socket_addresses<I>(socket_addresses: I)
where
    I: IntoIterator<Item = SocketAddr>,
{
    let timeout = Duration::new(1, 0);

    for addr in socket_addresses {
        if let Ok(_stream) = TcpStream::connect_timeout(&addr, timeout) {
            println!("{} is open", addr);
        }
    }
}