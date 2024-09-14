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
        Self {
            state: VendingMachineState::WaitingForCoin,
        }
    }

    fn insert_coin(&mut self) -> ActionResult {
        match self.state {
            VendingMachineState::WaitingForCoin => {
                self.state = VendingMachineState::WaitingForSelection;
                ActionResult::Success
            }
            _ => ActionResult::Error("You can't insert a coin now.",into()),
        }
    }
}
