enum VendingMachineState {
    WaitingForCoin,
    WaitingForSelection,
    Dispensing,
}

enum ActionResult {
    Success,
    Error(String),
} 

struct  VendingMachine {
    state: VendingMachineState,
}

impl VendingMachine {
    fn new() -> Self {
        state: VendingMachineState::WaitingForCoin,
    }
}
