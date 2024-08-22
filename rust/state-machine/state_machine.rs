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

    fn pause(&mut self) {
        match self.state {
            State::Running => {
                self.state = State::Paused;
                println!("State changed to: {:?}", self.state);
            }
            _ => println!("Cannot pause. Current state: {:?}", self.state),
        }
    }

    fn stop(&mut self) {
        match self.state {
            State::Running | State::Paused => {
                self.state = State::Stopped;
                println!("State changed to: {:?}", self.state);
            }
            _ => println!("Cannot stop. Current state: {:?}", self.state);
        }
    }

    fn reset(&mut self) {
        self.state = State::Start;
        println!("State reset to: {:?}", self.state);
    }
}

fn main() {
    let mut machine = StateMachine::new();

    machine.start();
    machine.pause();
    machine.start();
    machine.stop();
    machine.reset();
}
