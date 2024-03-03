use std::env;
use std::time::Duration;
use std::thread;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 2 {
        panic!("This program requires 1 ip argument")
    }

    let addr = args[1].parse().unwrap();
    let data = [1,2,3,4];
    let timeout = Duration::from_secs(1);
    let options = ping_rs::PingOptions { ttl: 128, dont_fragment: true };
    loop {
        let result = ping_rs::send_ping(&addr, timeout, &data, Some(&options));
        match result {
            Ok(reply) => println!("Reply from {}: bytes={} time={}ms TTL={}", reply.address, data.len(), reply.rtt, options.ttl),
            Err(e) => println!("{:?}", e)
        }
        thread::sleep(timeout)
    }
    
}
