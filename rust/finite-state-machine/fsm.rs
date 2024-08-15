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
