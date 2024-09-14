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

    fn dispense(&mut self) -> ActionResult {
        match self.state {
            VendingMachineState::Dispensing =>  {
                println!("Dispensing item...");
                self.state = VendingMachineState::WaitingForCoin;
                ActionResult::Success
            }
            _ => ActionResult::Error("You can't dispense anything now."into()),
        }
    }
}

fn main() {
    let mut machine =VendingMachine::new();

    if let ActionResult::Error(err) = machine.insert_coin() {
        println!("Error: {}", err);
    }

    if let ActionResult::Error(err) = machine.make_selection("Soda") {
        println!("Error: {}", err);
    }


}
