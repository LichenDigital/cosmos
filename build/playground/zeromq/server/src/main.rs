use std::thread;
use std::time::Duration;


fn main() {
    let context = zmq::Context::new();
    let responder = context.socket(zmq::REP).unwrap();
    
    assert!(responder.bind("tcp://*:5555").is_ok());
    
    let mut msg = zmq::Message::new();
    loop {
        responder.recv(&mut msg, 0).unwrap();
        println!("Hello there! We just recieved {}", msg.as_str().unwrap());
        thread::sleep(Duration::from_millis(1000));
        responder.send("We here you loud and clear!", 0).unwrap();
    }
}