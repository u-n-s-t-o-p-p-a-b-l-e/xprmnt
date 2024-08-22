#[derive(Debug)]
enum State {
    Start,
    Running,
    Paused,
    Stopped,
}

struct StateMachine {
    state: State,
}

impl StateMachine {
    fn new() -> StateMachine {
        StateMachine {
            state: State::Start,
        }
    }

    
}
