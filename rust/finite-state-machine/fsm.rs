#[derive(Debug)]
enum State {
    Start,
    Middle,
    End,
}

#[derive(Debug)]
struct FSM {
    state: State,
}

impl FSM {
    fn new() -> Self {
        FSM { state: State::Start }
    }

    fn transition(&mut self) {
        self.state = match self.state {
            State::Start => State::Middle,
            State::Middle => State::End,
            State::End => State::End,
        };
    }

    fn is_finished(&self) -> bool {
        matches!(self.state, State::End)
    }
}

fn main() {
    let mut fsm = FSM::new();

    while !fsm.is_finished() {
        println!("Current state: {:?}", fsm.state);
        fsm.transition();
    }

    println!("Final state: {:?}", fsm.state);
}
