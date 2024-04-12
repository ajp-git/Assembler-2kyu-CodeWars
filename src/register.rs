pub struct Register {
    pub val:i32,
}
impl Register {
    pub fn set_value(&mut self, new_val:i32 ) -> i32 {
        self.val=new_val;
        self.val
    }

    pub fn get_value(&self ) -> i32 {
        self.val
    }

    pub fn inc(&mut self) -> i32 {
        self.val+=1;
        self.val
    }

    pub fn dec(&mut self) -> i32 {
        self.val-=1;
        self.val
    }
}