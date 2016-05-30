pub type ThreadId = u32;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum State {
    READY,
    RECEIVE,
    SEND,
    REPLY
}

#[derive(Debug)]
pub struct Thread {
    pub id: ThreadId,
    pub state: State,
}


impl Thread {
    pub fn new(id: ThreadId) -> Thread {
        Thread {
            id: id,
            state: State::READY,
        }
    }

    pub fn send(&mut self, server_state: &State) {
        if self.state != State::READY {
            panic!("Cant transtion from {:?} to {:?}", self.state, State::SEND);
        }

        if *server_state == State::RECEIVE {
            self.state = State::REPLY;
        } else {
            self.state = State::SEND;
        }
    }

    pub fn client_sent(&mut self) {
        if self.state == State::RECEIVE {
            self.state = State::READY
        }
    }

    pub fn receive(&mut self, client_state: &State) {
        if self.state != State::READY {
            panic!("Cant transtion from {:?} to {:?}", self.state, State::RECEIVE);
        }

        if *client_state == State::SEND {
            self.state = State::READY;
        } else {
            self.state = State::RECEIVE;
        }
    }

    pub fn server_received(&mut self) {
        if self.state == State::SEND {
            self.state = State::REPLY
        }
    }

    pub fn reply(&mut self) {
        if self.state != State::READY {
            panic!("Cant reply when in current state: {:?}", self.state);
        }

        self.state = State::READY;
    }

    pub fn server_replied(&mut self) {
        if self.state != State::REPLY {
            panic!("Thread not waiting for reply, something went wrong. State: {:?}", self.state);
        }

        self.state = State::READY;
    }
}



#[cfg(test)]
mod thread_test {
    use super::{State, Thread};

    #[test]
    fn send_test() {
        let mut t = Thread { id: 1, state: State::READY};
        t.send(&State::RECEIVE);
        assert_eq!(t.state, State::REPLY);

        let mut t = Thread { id: 1, state: State::READY};
        t.send(&State::READY);
        assert_eq!(t.state, State::SEND);
    }

    #[test]
    fn receive_test() {
        let mut t = Thread { id: 1, state: State::READY};
        t.receive(&State::SEND);
        assert_eq!(t.state, State::READY);

        let mut t = Thread { id: 1, state: State::READY};
        t.receive(&State::READY);
        assert_eq!(t.state, State::RECEIVE);
    }

    #[test]
    fn reply_test() {
        let mut t = Thread { id: 1, state: State::READY};
        t.reply();
        assert_eq!(t.state, State::READY);
    }

    #[test]
    fn server_replied_test() {
        let mut t = Thread { id: 1, state: State::REPLY};
        t.server_replied();
        assert_eq!(t.state, State::READY);
    }
}
