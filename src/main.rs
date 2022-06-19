mod account;
use std::io;

/*
Print menu
prompt user input
*/

fn register() -> account::Account{
    println!("\nWelcome to the registration page.\nPlease enter your name to create an account:");
    let mut name= String::new();
    io::stdin().read_line(&mut name).expect("msg");
    let user: account::Account = account::Account::new(&name);
    println!("\tCreated user {}\tAccount balance: 0",name);
    user
}

fn update(){}
fn delete(){}

fn deposit(user: account::Account) -> f32{
    println!("\nEnter amount you wish to deposit:");
    let mut amount= String::new();
    io::stdin().read_line(&mut amount).expect("msg");
    let amount = amount.trim().parse::<f32>().expect("invalid input");
    let balance: f32 = user.getBalance(); 
    println!("\tSuccesfuly deposited {}$\n\tNew account balance {}$",amount,balance+&amount);
    balance+&amount
}

fn withdraw(user: account::Account) -> f32{
    //TODO: Code to identify user and respective balance

    println!("\nEnter amount you wish to withdraw:");
    let mut amount= String::new();
    io::stdin().read_line(&mut amount).expect("msg");
    let amount = amount.trim().parse::<f32>().expect("invalid input");
    
    let balance: f32 = user.getBalance(); 
    if balance-amount>0.0 {
        println!("\tSuccesfuly withdrew {}$\n\tNew account balance {}$",amount,balance-&amount);
    }else {
        println!("\tAvailable amount {}$, insufficient account balance",balance);
    }
    balance-&amount
}
fn transfer(){}
fn quit(){}

fn main() {

    let menu = ["Register new account","Update account information","Delete account records","Deposit Money","Withdraw Money","Transfer Money","Exit"];
    println!("Welcome to the rust bank");    
    
    while 1==1 {
        println!("\nPlease select an option from below:");
        let mut index=0;
        for item in &menu  {
            index+=1;
            println!("\t{}. {}",index,item);
        }
        let mut option= String::new();
        io::stdin().read_line(&mut option).expect("msg");
        let option = option.trim().parse::<i32>().expect("invalid input");
        let mut user:account::Account = account::Account::new("name");
        match option {
            1 => {user = register();}
            2 => {update()}
            3 => {delete()}
            4 => {
                let new_balance:f32 = deposit(user);
                //&user.reset_amount(new_balance); TODO: fix commented error
            }
            5 => {
                let new_balance:f32 = withdraw(user);
                //&user.reset_amount(new_balance);
            }
            6 => {transfer()}
            7 => {quit();break;}
            _ => {println!("\nInvalid input!!\n\tTry again")}
        }
    }
}