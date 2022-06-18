mod account;
use std::io;

use crate::account::Account;
/*
Print menu
prompt user input
*/

fn register(){
    println!("\nWelcome to the registration page.\nPlease enter your name to create an account:");
    let mut name= String::new();
    io::stdin().read_line(&mut name).expect("msg");
    let user:account::Account = Account::new(&name);
    //TODO: Create data structure to hold multiple objects
}
fn login() -> String{
    println!("Please login to continue => enter new to register account");
    println!("Username");
    let mut username= String::new();
    io::stdin().read_line(&mut username).expect("msg");
    println!("Password");
    let mut password= String::new();
    io::stdin().read_line(&mut password).expect("msg");
    //TODO: Complete login feature

    "new".to_string()
}
fn update(){}
fn delete(){}
fn deposit(){
    println!("\nEnter amount you wish to deposit:");
    let mut amount= String::new();
    io::stdin().read_line(&mut amount).expect("msg");
    let amount = amount.trim().parse::<i32>().expect("invalid input");
    //TODO: Code to deposit

}
fn withdraw(){
    //TODO: Code to identify user and respective balance

    println!("\nEnter amount you wish to withdraw:");
    let mut amount= String::new();
    io::stdin().read_line(&mut amount).expect("msg");
    let amount = amount.trim().parse::<f32>().expect("invalid input");
    
    let balance: f32 = 79.89; //TODO assign value from user object
    if balance-amount>0.0 {
        println!("\tSuccesfuly withdrew {}$\n\tNew account balance {}$",amount,balance-&amount);
    }else {
        println!("{}$, insufficient account balance",balance);
    }
}
fn transfer(){}
fn quit(){}
fn main() {

    let menu = ["Register new account","Update account information","Delete account records","Deposit Money","Withdraw Money","Transfer Money","Exit"];
    println!("Welcome to the rust bank");    
    
    //let user=login();    

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
            1 => {register()}
            2 => {update()}
            3 => {delete()}
            4 => {deposit()}
            5 => {withdraw()}
            6 => {transfer()}
            7 => {quit();break;}
            _ => {println!("\nInvalid input!!\n\tTry again")}
        }
    }
}
