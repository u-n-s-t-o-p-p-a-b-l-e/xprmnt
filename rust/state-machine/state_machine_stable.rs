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

    fn make_selection(&mut self, selection: &str) -> ActionResult {
        match self.state {
            VendingMachineState::WaitingForSelection =>  {
                println!("You selected: {}", selection);
                self.state = VendingMachineState::Despensing;
                ActionResult::Success
            }
            _ => ActionResult::Error("You need to insert a coin first.".into()),
        }
    }
}
