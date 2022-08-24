mod account;
use std::io;

/*
Print menu
prompt user input
*/

fn register(user: &mut account::Account){
    println!("\nWelcome to the registration page.\nPlease enter your name to create an account:");
    let mut name= String::new();
    io::stdin().read_line(&mut name).expect("msg");
    user.name=name;
    println!("\tCreated user {}\tAccount balance: 0",user.get_username());
}

fn check_balance(user: &account::Account){
    println!("Balance for {} is: {}",user.get_username(),user.get_balance());
}
fn update(){}
fn delete(){}

fn deposit(user: &mut account::Account) {
    println!("\nEnter amount you wish to deposit:");
    let mut amount= String::new();
    io::stdin().read_line(&mut amount).expect("msg");
    let amount = amount.trim().parse::<f32>().expect("invalid input");
    let balance: f32 = user.get_balance(); 
    user.reset_amount(balance+&amount);
    println!("\tSuccesfuly deposited {}$ to \n\tNew account balance {}$",amount,balance+&amount);
}

fn withdraw(user: &mut account::Account){
    //TODO: Code to identify user and respective balance

    println!("\nEnter amount you wish to withdraw:");
    let mut amount= String::new();
    io::stdin().read_line(&mut amount).expect("msg");
    let amount = amount.trim().parse::<f32>().expect("invalid input");
    
    let balance: f32 = user.get_balance(); 
    if balance-amount>0.0 {
        println!("\tSuccesfuly withdrew {}$\n\tNew account balance {}$",amount,balance-&amount);
        user.reset_amount(balance-&amount);
    }else {
        println!("\tAvailable amount {}$, insufficient account balance",balance);
    }
}
fn transfer(){}
fn quit(){}

fn main() {

    let menu = ["Register new account","Update account information","Delete account records","Deposit Money","Withdraw Money","Transfer Money","Check Balance","Exit"];
    println!("Welcome to the rust bank");
    let mut user = account::Account{
        name: String::from("_"),
        balance: 0.0,
    };
    
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
        match option {
            1 => {register(&mut user);}
            2 => {update()}
            3 => {delete()}
            4 => {
                deposit(&mut user);
            }
            5 => {
                withdraw(&mut user);
            }
            6 => {transfer()}
            7 => {check_balance(&user);}
            8 => {quit();break;}
            _ => {println!("\nInvalid input!!\n\tTry again")}
        }
    }
}