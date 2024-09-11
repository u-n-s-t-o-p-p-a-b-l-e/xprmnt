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

        match option {
            1 => {
                println!("Enter amount to deposit: ");
                let mut amount = String::new();
                io::stdin().read_line(&mut amount).expect("Failed to read input");

                let amount: f64 = amount.trim().parse().expect("Invalid amount");
                account.deposit(amount);
            }
            2 => {
                println!("Enter amount to withdraw: ");
                let mut amount = String::new();
                io::stdin().read_line(&mut amount).expect("Failed to read input");

                let amount: f64 = amount.trim().parse().expect("Invalid amount");
                match account.withdraw(amount) {
                    Ok(_) => {}
                    Err(e) => println!("Error: {}", e);
                }
            }
            3 => {
                account.display_balance();
            }
            4 => {
                println!("Exiting...");
                break;
            }
            _ => {
                println!("Invalid option, please try again.");
            }
        }
    }
}
