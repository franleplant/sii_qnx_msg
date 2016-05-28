
type Id = u32;
type Payload = String;

#[derive(Debug, PartialEq, Eq)]
enum State {
    READY,
    RECEIVE,
    SEND,
    REPLY
}

//#[derive(Debug)]
//enum EventType {
    //Ready,
    //Receive,
    //Send,
    //Reply
//}

#[derive(Debug)]
struct Thread {
    id: Id,
    state: State
}


//#[derive(Debug)]
//stuct Event {
    //target: Id,
//}

impl Thread {
    fn new(id: Id) -> Thread {
        Thread {
            id: id,
            state: State::READY
        }
    }

    fn send(&mut self, target_id: Id, payload: Payload) {
        if self.state != State::READY {
            panic!("WRONG STATE");
        }

        self.state = State::SEND;
    }

    fn receive(&mut self, target_id: Id, payload: Payload) {
        if self.state != State::READY {
            panic!("WRONG STATE");
        }

        self.state = State::RECEIVE;
    }

    fn reply(&mut self, target_id: Id, payload: Payload) {
        if self.state != State::READY {
            panic!("WRONG STATE");
        }

        self.state = State::READY;
    }
}


fn main() {
    let mut t = Thread::new(123);
    t.send(123, "hi!".to_string());
    println!("Hello, world! {:?}", t);
}




