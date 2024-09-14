enum VendingMachineState {
    WaitingForCoin,
    WaitingForSelection,
    Dispensing,
}

enum ActionResult {
    Success,
    Error(String),
} 
