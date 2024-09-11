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

    fn display_balance(&self) {
        println!("Current balance: ${:.2}", self.balance);
    }
}

fn main() {
    let mut account = BankAccount::new();

    loop {
        println!("Choose an option: ");
        println!("1: Deposit");
        println!("2: Withdraw");
        println!("3: Check Balance");
        println!("4: Exit");

        let mut option = String::new();
        io::stdin().read_line(&mut option).expect("Failed to read input");

        let option = option.trim().parse::<u32>().unwrap_or(0);
    }
}
