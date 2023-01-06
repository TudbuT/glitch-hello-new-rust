use microasync_util::io::read::tcpstream;
use microasync_util::io::{ReadAsync, WriteAsync};
use microasync_util::{get_current_runtime, QueuedRuntime};
use std::fs;
use std::net::{Shutdown, TcpListener};
use std::str;

fn main() {
    // MicroAsync is a library using edition 2021 and some new features.
    // This will not work in 2018, which is the edition glitch normally uses.
    microasync::sync_with(QueuedRuntime::new_with(async_main()), 50); // use a higher number to lower CPU usage
}

async fn async_main() {
    println!("Hello, world!");

    // Let's make a *TINY* and very bad but async web server
    let mut listener = TcpListener::bind(("0.0.0.0", 4000)).unwrap(); // bind to all interfaces on port 4000 (glitch's port)
    while let Ok((mut socket, addr)) = tcpstream::accept(&mut listener).await {
        println!(
            "We got a connection from {addr:?}! BTW, our current CPU load is {}.",
            fs::read_to_string("/proc/loadavg")
                .unwrap()
                .split(" ")
                .next()
                .unwrap()
        );
        get_current_runtime().await.push(async move {
            let mut full = Vec::new();
            let mut buf = [0_u8; 256];
            loop {
                let read = socket.read(&mut buf).await.expect("connection broken");
                full.append(&mut buf[..read].to_vec());
                if let Ok(true) = str::from_utf8(&full).map(|x| x.contains("\r\n\r\n")) {
                    break;
                }
            }
            let index = fs::read_to_string("index.html").unwrap();
            socket
                .write(
                    include_str!("../index.html.http")
                        .replace("{length}", index.as_bytes().len().to_string().as_str())
                        .replace("\n", "\r\n")
                        .replace("{content}", &index)
                        .as_bytes(),
                )
                .await
                .unwrap();
            socket.shutdown(Shutdown::Both).unwrap();
        });
    }
}
