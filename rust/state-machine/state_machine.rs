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
