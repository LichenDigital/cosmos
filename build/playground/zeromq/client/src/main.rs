fn main() {
    println!("Hollering at the server, trying to connect right now!");

    let context = zmq::Context::new();
    let requester = context.socket(zmq::REQ).unwrap();

    assert!(requester.connect("tcp://localhost:5555").is_ok());

    let mut msg = zmq::Message::new();

    for request_nbr in 0..10 {
        println!("Saying hello now! {}", request_nbr);
        requester.send("Hello", 0).unwrap();

        requester.recv(&mut msg, 0).unwrap();
        println!("A message is coming in on the line right now: {}, This is the {} time we've recieved it!", msg.as_str().unwrap(), request_nbr);
    }
}
