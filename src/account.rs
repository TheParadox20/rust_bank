
pub struct Account {
    name: String,
    //ID: u8,
    balance: f32
}

impl Account {
    pub fn new(name: &str) -> Self{
        Self { name: name.to_string(), balance: 0.0 }
    }

    pub fn deposit(&mut self,amount: f32){
        self.balance+=amount;
    }

    pub fn withdraw(&mut self,amount: f32){
        self.balance-=amount;
    }
}