
#[derive(Clone)]
pub struct Account {
    pub name: String,
    //ID: u8,
    pub balance: f32
}

impl Account {
    pub fn set_username(&mut self, name: String){
        self.name=name;
    }
    pub fn get_username(&self) -> &String{
        &self.name
    }

    pub fn get_balance(&self) -> f32{
        self.balance
    }


    pub fn reset_amount(&mut self,amount: f32){
        self.balance=amount;
    }
}