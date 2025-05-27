use std::io::{self, Write};

const ERROR_INPUT: &str = "!! Error reading user input.";

enum TransactionType {
    Credit,
    Debit,
}

struct Transaction {
    amount: i32,
    description: String,
    transaction_type: TransactionType,
}

fn display_account(account: &Vec<Transaction>) {
    let mut total_amount: i32 = 0;
    println!("\n-------- Account --------\n");

    if account.len() > 0 {
        for transaction in account {
            match transaction.transaction_type {
                TransactionType::Debit => {
                    println!("- {} | {}", transaction.amount, transaction.description);
                    total_amount -= transaction.amount;
                }
                TransactionType::Credit => {
                    println!("+ {} | {}", transaction.amount, transaction.description);
                    total_amount += transaction.amount;
                }
            }
        }
        println!("\nTotal Amount : {}\n", total_amount);
    }

    println!("1. Add transaction.\n");
}

fn add_transaction(account: &mut Vec<Transaction>) {
    let mut user_input = String::new();

    loop {
        println!("\n(Écris 'exit' pour revenir au menu)\n");

        print!("Montant de la transaction : ");
        io::stdout().flush().unwrap();
        user_input.clear();
        io::stdin().read_line(&mut user_input).expect(ERROR_INPUT);
        if user_input.trim() == "exit" {
            break;
        }

        let amount: i32 = match user_input.trim().parse() {
            Ok(val) => val,
            Err(_) => {
                println!("!! Error, a integer is required.");
                continue;
            }
        };

        print!("Description : ");
        io::stdout().flush().unwrap();
        let mut description = String::new();
        io::stdin().read_line(&mut description).expect(ERROR_INPUT);

        print!("Type (credit/debit) : ");
        io::stdout().flush().unwrap();
        let mut trans_type_input = String::new();
        io::stdin().read_line(&mut trans_type_input).expect(ERROR_INPUT);

        let transaction_type = match trans_type_input.trim().to_lowercase().as_str() {
            "credit" => TransactionType::Credit,
            "debit" => TransactionType::Debit,
            _ => {
                println!("Invalide type. use 'credit' or 'debit'.");
                continue;
            }
        };

        account.push(Transaction {
            amount,
            description: description.trim().to_string(),
            transaction_type,
        });

        println!("Transaction added!");
    }
}

fn main() {
    let mut user_input = String::new();
    let mut account: Vec<Transaction> = Vec::new();

    while user_input.trim() != "exit" {
        display_account(&account);

        user_input.clear();
        io::stdin().read_line(&mut user_input).expect(ERROR_INPUT);

        match user_input.trim() {
            "1" => add_transaction(&mut account),
            "exit" => return,
            _ => continue,
        }
    }
}
