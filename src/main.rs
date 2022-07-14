use std::net::UdpSocket;
use std::{thread};
use std::time::{SystemTime, UNIX_EPOCH, Duration};
use rand::Rng;

fn main() -> std::io::Result<()> {
    {
        let mut used_ports = Vec::new();
        let mut thread_handles = Vec::new();
        for i in 0..15 {
            let mut got_new_port = false;
            let mut port = 0;
            println!("Searching for an unused port...");
            while !got_new_port {
                port = rand::thread_rng().gen_range(34000..35000);
                if !used_ports.contains(&port) {
                    got_new_port = true;
                    used_ports.push(port);
                }
            }
            println!("Thread {} starting...", i);
            let handle = thread::spawn( move || {
                let socket = UdpSocket::bind(format!("127.0.0.1:{port}")).expect(format!("could not bind to port {}", &port).as_str());
                emit(50, &socket).expect("Could not emit from pulsar");
            });
            thread_handles.push(handle);
        }
        println!("Waiting for threads to finish");
        for h in thread_handles {
            h.join().expect("thread failed");
        }
    } // the socket is closed here
    Ok(())
}

fn emit(iters: usize, socket: &std::net::UdpSocket) -> std::io::Result<()> {
    let instance_id = rand::thread_rng().gen_range(u128::MAX/2..u128::MAX);
    let mut iters_remaining = iters;
    while  iters_remaining > 0 {           
        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        let in_ms = since_the_epoch.as_millis();
        let message = format!("{} {} {}", instance_id, in_ms, rand::thread_rng().gen_range(1..=100));
        println!("{message}");
        thread::sleep(Duration::from_millis(rand::thread_rng().gen_range(1..50)));

        let message_bytes = message.as_bytes();
        socket.send_to(message_bytes, "127.0.0.1:8080")?;
        iters_remaining -= 1;
    }
    Ok(())
}