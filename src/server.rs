use std::net::TcpListener;
use tungstenite::accept;

/// A WebSocket echo server
fn main() {
    let server = TcpListener::bind("127.0.0.1:9001").unwrap();
    for stream in server.incoming() {
    	std::thread::spawn (move || {
	    let mut websocket = accept(stream.unwrap()).unwrap();
	    loop {
		let msg = websocket.read_message().unwrap();
		// We do not want to send back ping/pong messages.
		if msg.is_binary() || msg.is_text() {
		    println!("Echoing message: {}", msg);
		    websocket.write_message(msg).unwrap();
		}
	    }
	});
    }
}
