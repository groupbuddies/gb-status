extern crate "rustic-io" as rustic_io;

use rustic_io::socket::Socket;

pub struct Foo {
    msg: String
}

fn main() {
    let mut server = rustic_io::new_server("127.0.0.1", "1338");
    server.on("some_event", function_to_execute);
    rustic_io::start(server);
}

fn function_to_execute(data: json::Json, socket: Socket) {
    let json_object= data.as_object().unwrap();

    let bar = Foo {
        msg: String::from_str("Hello from rust!")
    };
    
    socket.send("some_event", json::encode(&bar));
}
