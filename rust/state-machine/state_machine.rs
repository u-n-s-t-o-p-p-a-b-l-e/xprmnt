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

    fn start(&mut self) {
        match self.state {
            State::Start | State::Paused => {
                self.state = State::Running;
                println!("State changed to: {:?}", self.state);
            }
            _ => println!("Cannot start. Current state: {:?}", self.state),
        }
    }
}
