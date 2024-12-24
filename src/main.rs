use std::io;

fn input(prompt: &str) -> String {
    print!("{}", prompt);
    io::Write::flush(&mut io::stdout()).expect("Failed to flush stdout");
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");
    user_input.trim().to_string()
}

fn number_check(user_input: &str) -> f64 {
    match user_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number");
            0.0
        }
    }
}

enum MenuOption {
    Deposit,
    Withdraw,
    Display,
    Invalid,
}

impl MenuOption {
    fn from_input(input: &str) -> MenuOption {
        match input {
            "1" => MenuOption::Deposit,
            "2" => MenuOption::Withdraw,
            "3" => MenuOption::Display,
            _ => MenuOption::Invalid,
        }
    }
}

fn main() {
    struct Account {
        name: String,
        balance: f64,
    }
    impl Account {
        fn create(name: String, balance: f64) -> Account {
            Account { name, balance }
        }
        fn display(&self) {
            println!("Account Name: {}", self.name);
            println!("Account Balance: {}", self.balance);
        }
        fn deposit(&mut self, amount: f64) {
            self.balance += amount;
        }
        fn withdraw(&mut self, amount: f64) {
            if amount > self.balance {
                println!("Insufficient funds");
            } else {
                self.balance -= amount;
            }
        }
    }
    println!("Welcome to the Simple Banking System");
    println!("To bank with us you need to create an account");
    let name = input("Enter your name: ");
    let balance = input("Enter your initial balance: ");
    let balance = number_check(&balance);
    let mut account = Account::create(name, balance);
    println!("What would you like to do?");
    let mut active: bool = true;
    while active {
        println!(
            "
             ----------------------------
            | 1. Deposit                 |
            | 2. Withdraw                |
            | 3. Display balance         |
             ----------------------------
            "
        );
        let choice = input("Enter your choice: ");
        match MenuOption::from_input(&choice) {
            MenuOption::Deposit => {
                let amount = input("Enter the amount you want to deposit: ");
                let amount = number_check(&amount);
                account.deposit(amount);
                account.display();
            }
            MenuOption::Withdraw => {
                let amount = input("Enter the amount you want to withdraw: ");
                let amount = number_check(&amount);
                account.withdraw(amount);
                account.display();
            }
            MenuOption::Display => {
                account.display();
            }
            MenuOption::Invalid => {
                println!("Invalid choice");
            }
        }
        let choice = input("Do you want to perform another transaction? (y/n): ");
        if choice.to_lowercase() != "y" {
            active = false;
        }
    }
    println!("Thank you for banking with us");
}
