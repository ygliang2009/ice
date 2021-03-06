use std::io::{ Read, Write };

// Shutdown{ Read, Write, Both }
use std::net::{ TcpListener, TcpStream, UdpSocket, Shutdown };
use std::thread;

// TODO: 切换到高性能网络库
//       https://github.com/carllerche/mio/blob/getting-started/doc/getting-started.md

mod stun;


fn tcp_client() {
    let host       = "127.0.0.1:8000";
    let mut stream = TcpStream::connect(host).unwrap();
    // ignore the Result
    let _ = stream.write(&[71, 69, 84]);

    let mut buffer    = vec![0; 2048];
    let length: usize = stream.read(&mut buffer).unwrap();
    let buffer = &mut buffer[0..length];

    println!("Buffer Len: {:?}", length);
    println!("Buffer Data:\n{:?}", buffer );

    // the stream is closed here
    drop(stream);
}

fn tcp_server() {
    // https://tools.ietf.org/html/rfc5389#section-18.4
    // port: 3478
    let host     = "127.0.0.1:9000";
    let listener = TcpListener::bind(host).unwrap();
    println!("[TCP Server] server running on {} ...", host);
    println!("[TCP Server] listening started, ready to accept ...");
    for stream in listener.incoming() {
        match stream.ok() {
            Some(stream) => {
                let child = thread::spawn(move|| {
                    let mut stream  = stream;
                
                    println!("Addres: {:?} \t {:?}", stream.peer_addr(), stream.local_addr() );

                    // Bytes
                    // [71, 69, 84] => "GET"
                    let mut buffer  = vec![0; 3];
                    let length = stream.read(&mut buffer).unwrap();

                    // Strings
                    // let mut buffer = String::new();
                    // let length = stream.read_to_string(&mut buffer);

                    // Read All
                    // let mut buffer = Vec::new();
                    // let length = stream.read_to_end(&mut buffer);

                    println!("Buffer Len: {:?}", length);
                    println!("Buffer Data:\n{:?}", buffer );


                    // Response
                    stream.write(&[87,87,87,87]);

                    // shutdown connection
                    stream.shutdown(Shutdown::Both);
                });
                let res = child.join();
            },
            None  => {
                println!("[WARN] connection failed " );
            }
        }
    }
    // close the socket server
    drop(listener);
}

fn udp_server(){
    let host = "127.0.0.1:9000";
    let mut socket = UdpSocket::bind(host).unwrap();

    println!("[UDP Server] server running on {} ...", host);

    let child = thread::spawn(move || {
        let mut buffer = vec![0; 2048];
        loop {
            // received data from connection
            let result     = socket.recv_from(&mut buffer).ok();
            match result {
                Some((length, addr)) => {
                    // Send a reply to the socket we received data from
                    let buffer = &mut buffer[0..length];
                    println!("[UDP Server] received data from {:?} :\n{:?}", addr, buffer );

                    socket.send_to(buffer, &addr);
                    println!("[UDP Server] send back.");
                },
                None => {
                    println!("[UDP Server] received data fail !");
                }
            };
        };
        // close the socket
        drop(socket);
    });
    let res = child.join();
}



fn main() {

    // let tcp = thread::spawn(move || {
    //     tcp_server();
    // });
    // let udp = thread::spawn(move || {
    //     udp_server();
    // });
    // let _ = tcp.join();
    // let _ = udp.join();
    
    // udp_server();
    // tcp_client();
    let host = "127.0.0.1:9000";
    stun::run(host);
}
