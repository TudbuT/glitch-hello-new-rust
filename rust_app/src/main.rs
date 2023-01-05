use microasync_util::io::read::tcpstream;
use microasync_util::io::{ReadAsync, WriteAsync};
use microasync_util::{get_current_runtime, QueuedRuntime};
use std::net::{Shutdown, TcpListener};
use std::process::Command;
use std::str;
use std::fs;

fn main() {
    // MicroAsync is a library using edition 2021 and some new features.
    // This will not work in 2018, which is the edition glitch normally uses.
    let mut runtime = QueuedRuntime::new();
    runtime.push(async_main());
    microasync::sync_with(runtime, 5); // use a higher number to lower CPU usage
}

async fn async_main() {
    let version = str::from_utf8(
        Command::new("rustc")
            .arg("--version")
            .output()
            .expect("couldn't find rustc, but it should be installed")
            .stdout
            .as_slice(),
    )
    .expect("invalid rustc output")
    .to_owned();
    let version = version[..version.len() - 1].to_owned();
    println!("Hello, world!");
    println!("This runs on rust version {}", version);

    // Let's make a *TINY* and very bad but async web server
    let mut listener = TcpListener::bind(("0.0.0.0", 4000)).unwrap(); // bind to all interfaces on port 4000 (glitch's port)
    while let Ok((mut socket, addr)) = tcpstream::accept(&mut listener).await {
        println!("We got a connection from {addr:?}!");
        let version = version.to_owned();
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
            let index = fs::read_to_string("index.html")
                .unwrap()
                .replace("{rust-version}", version.as_str());
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
