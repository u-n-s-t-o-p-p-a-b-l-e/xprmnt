use std::io;

struct BankAccount {
    balance: f64,
}

impl BankAccount {
    fn new() -> BankAccount {
        BankAccount { balance: 0.0 }
    }

    fn deposit(&mut self, amount: f64) {
        self.balance += amount;
        println!("Deposited: ${:.2}", amount);
    }

    fn withdraw(&mut self, amount: f64) -> Result<(), String> {
        if amount > self.balance {
            Err("Insufficient funds".to_string())
        } else {
            self.balance -= amount;
            println!("Withdrew: ${:.2}", amount);
            Ok(())
        }
    }
}
