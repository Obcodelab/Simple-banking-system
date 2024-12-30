# Simple Banking System in Rust

## Overview

This project implements a command-line banking system in Rust. The program allows users to create an account, deposit funds, withdraw funds, and display their account balance. It is a simple and interactive banking interface suitable for learning Rust concepts like:

- Structs for modeling accounts.
- Enums for handling menu options.
- Input handling and validation.
- Basic arithmetic operations.
- Loops and control flow.

## Features

1. Account Creation:

   - Users create an account by providing their name and initial balance.

2. Deposit:

   - Add a specified amount to the account balance.

3. Withdraw:

   - Deduct a specified amount from the account balance, with a check for insufficient funds.

4. Display Account Details:

   - Show the account name and current balance.

5. Menu-Driven Interface:

   - Easily navigate through deposit, withdraw, and display options.

6. Repeat Transactions:

   - Perform multiple transactions until the user chooses to exit.

## Example User Interaction

```plaintext
Welcome to the Simple Banking System
To bank with us you need to create an account
Enter your name: John Doe
Enter your initial balance: 1000
What would you like to do?

 ____________________
| 1. Deposit         |
| 2. Withdraw        |
| 3. Display balance |
 ____________________

Enter your choice: 1
Enter the amount you want to deposit: 500
Account Name: John Doe
Account Balance: 1500

Do you want to perform another transaction? (y/n): y

 ____________________
| 1. Deposit         |
| 2. Withdraw        |
| 3. Display balance |
 ____________________

Enter your choice: 2
Enter the amount you want to withdraw: 2000
Insufficient funds
Account Name: John Doe
Account Balance: 1500

Do you want to perform another transaction? (y/n): n
Thank you for banking with us
```

## Input Validation

- The `input` function prompts users for input and trims unnecessary whitespace.
- The `number_check` function ensures numeric input for financial transactions, printing an error message for invalid entries and defaulting to `0.0`.

## How to Use

### Prerequisites

- Ensure you have Rust and Cargo installed. [Install Rust here](https://www.rust-lang.org/tools/install).

### Steps to Run

1. Clone the repository:

```sh
git clone https://github.com/Obcodelab/Simple-banking-system.git
```

2. Navigate to the project directory:

```sh
cd Simple-banking-system
```

3. Run the program:

```sh
cargo run
```

## Code Explanation

1. **Structs**:

- The `Account` struct represents a user account with a name and balance.
- Methods for `Account` include:
  - `create`: Initialize a new account.
  - `display`: Show account details.
  - `deposit`: Add funds to the account.
  - `withdraw`: Deduct funds with a check for insufficient balance.

2. **Enums**:

   - `MenuOption` represents user choices for menu navigation, ensuring structured and readable code.

3. **Input Module**:

   - The `input` function facilitates user input.
   - The `number_check` function validates numerical input to prevent runtime errors.

4. **Main Function**:

   - The entry point for the program handles user interactions, processes menu choices, and calls appropriate methods on the `Account` struct.

## Future Enhancements

- **Transaction History**:

  - Maintain a log of all transactions.

- **Multiple Accounts**:

  - Add functionality for creating and managing multiple accounts.

- **Authentication**:

  - Introduce PIN-based authentication for account security.

- **Persistent Storage**:

  - Save account details to a file and reload them on subsequent program runs.

## Contributing

Contributions are welcome! Feel free to open an issue or submit a pull request with suggestions or improvements.
