use thread::{State, Thread, ThreadId};
use std::collections::HashMap;

pub type Payload = String;
pub type ThreadMap = HashMap<ThreadId, Thread>;


#[derive(Debug)]
pub struct ThreadPool {
    pub pool: ThreadMap,
    pub id_count: ThreadId
}


impl ThreadPool {
    pub fn new() -> ThreadPool {
        ThreadPool {
            pool: HashMap::new(),
            id_count: 1
        }
    }

    pub fn print(&self) {
        for id in 1..self.id_count {
            println!("Thread {}, {}", id, self.pool.get(&id).unwrap().state);
        }
        println!("");
    }

    pub fn create(&mut self) -> ThreadId {
        let thread_id = self.id_count;
        let thread = Thread::new(thread_id);
        self.id_count = self.id_count + 1;
        self.pool.insert(thread_id, thread);
        thread_id
    }

    /// # send msg
    ///
    /// - If the client thread calls MsgSend(), and the server thread hasn't yet called
    /// MsgReceive(), then the client thread becomes SEND blocked. Once the server
    /// thread calls MsgReceive(), the kernel changes the client thread's state to be REPLY
    /// blocked, which means that server thread has received the message and now must
    /// reply. When the server thread calls MsgReply(), the client thread becomes READY.
    /// - If the client thread calls MsgSend(), and the server thread is already blocked on
    /// the MsgReceive(), then the client thread immediately becomes REPLY blocked,
    /// skipping the SEND-blocked state completely.
    pub fn send(&mut self, client_id: ThreadId, server_id: ThreadId, payload: Payload) {
        println!("Send {}, {}", client_id, server_id);
        let server_state: State = {
            let server: &Thread = self.pool.get(&server_id).unwrap();
            server.state.clone()
        };

        {
            let client: &mut Thread = self.pool.get_mut(&client_id).unwrap();
            client.send(&server_state)
        }

        {
            let server: &mut Thread = self.pool.get_mut(&server_id).unwrap();
            server.client_sent();
        }

    }

    /// receive msg
    ///
    /// - If the server thread calls MsgReceive(), and no other thread has sent to it, then
    /// the server thread becomes RECEIVE blocked. When another thread sends to it, the
    /// server thread becomes READY.
    /// - If the server thread calls MsgReceive(), and another thread has already sent to it,
    /// then MsgReceive() returns immediately with the message. In this case, the server
    /// thread doesn't block.
    /// - If the server thread calls MsgReply(), it doesn't become blocked.
    ///
    pub fn receive(&mut self, server_id: ThreadId, client_id: ThreadId) {
        println!("Receive {}, {}", server_id, client_id);
        let client_state: State = {
            let client: &Thread = self.pool.get(&client_id).unwrap();
            client.state.clone()
        };

        {
            let server: &mut Thread = self.pool.get_mut(&server_id).unwrap();
            server.receive(&client_state);
        }

        {
            let client: &mut Thread = self.pool.get_mut(&client_id).unwrap();
            client.server_received();
        }

    }

    pub fn reply(&mut self, server_id: ThreadId, client_id: ThreadId, payload: Payload) {
        println!("Reply {}, {}", server_id, client_id);
        {
            let server: &mut Thread = self.pool.get_mut(&server_id).unwrap();
            server.reply();
        }

        {
            let client: &mut Thread = self.pool.get_mut(&client_id).unwrap();
            client.server_replied();
        }
    }
}




#[cfg(test)]
mod thread_pool {
    use super::{ThreadPool};
    use thread::{State, Thread};

    #[test]
    fn send_not_received_test() {
        let mut thread_pool = ThreadPool::new();
        let id1 = thread_pool.create();
        let id2 = thread_pool.create();

        let thread2_state: State = {
            let t = thread_pool.pool.get(&id2).unwrap();
            t.state.clone()
        };

        thread_pool.send(id1, id2, "Hi".to_string());

        let thread1: &Thread = thread_pool.pool.get(&id1).unwrap();
        let thread2: &Thread = thread_pool.pool.get(&id2).unwrap();
        assert_eq!(thread1.state, State::SEND);
        assert_eq!(thread2.state, thread2_state);
    }

    #[test]
    fn send_received_test() {
        let mut thread_pool = ThreadPool::new();
        let id1 = thread_pool.create();
        let id2 = thread_pool.create();

        {
            let thread2: &mut Thread = thread_pool.pool.get_mut(&id2).unwrap();
            thread2.state = State::RECEIVE;
        }

        thread_pool.send(id1, id2, "Hi".to_string());

        let thread1: &Thread = thread_pool.pool.get(&id1).unwrap();
        let thread2: &Thread = thread_pool.pool.get(&id2).unwrap();
        assert_eq!(thread1.state, State::REPLY);
        assert_eq!(thread2.state, State::READY);
    }

    #[test]
    fn receive_not_send_test() {
        let mut thread_pool = ThreadPool::new();
        let id1 = thread_pool.create();
        let id2 = thread_pool.create();

        let thread2_state: State = {
            let t = thread_pool.pool.get(&id2).unwrap();
            t.state.clone()
        };

        thread_pool.receive(id1, id2);

        let thread1: &Thread = thread_pool.pool.get(&id1).unwrap();
        let thread2: &Thread = thread_pool.pool.get(&id2).unwrap();
        assert_eq!(thread1.state, State::RECEIVE);
        assert_eq!(thread2.state, thread2_state);
    }

    #[test]
    fn receive_sent_test() {
        let mut thread_pool = ThreadPool::new();
        let id1 = thread_pool.create();
        let id2 = thread_pool.create();

        {
            let thread2: &mut Thread = thread_pool.pool.get_mut(&id2).unwrap();
            thread2.state = State::SEND;
        }

        thread_pool.receive(id1, id2);

        let thread1: &Thread = thread_pool.pool.get(&id1).unwrap();
        let thread2: &Thread = thread_pool.pool.get(&id2).unwrap();
        assert_eq!(thread1.state, State::READY);
        assert_eq!(thread2.state, State::REPLY);
    }

    #[test]
    fn reply_test() {
        let mut thread_pool = ThreadPool::new();
        let id1 = thread_pool.create();
        let id2 = thread_pool.create();

        {
            let thread1: &mut Thread = thread_pool.pool.get_mut(&id1).unwrap();
            thread1.state = State::REPLY;
        }

        thread_pool.reply(id2, id1, "Welcome back".to_string());

        let thread1: &Thread = thread_pool.pool.get(&id1).unwrap();
        let thread2: &Thread = thread_pool.pool.get(&id2).unwrap();
        assert_eq!(thread1.state, State::READY);
        assert_eq!(thread2.state, State::READY);
    }
}
