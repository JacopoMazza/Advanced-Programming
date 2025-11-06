use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct Date{
    pub day: u8,
    pub month: u8,
    pub year: u16,
}

#[derive(Debug)]
pub struct Hour {
    pub hour: u8,
    pub minute: u8,
}

#[derive(Debug)]
pub struct BoxShipping{
    pub name: String,
    pub barcode:String,
    pub shipping_date: Date,
    pub shipping_hour: Hour,
}

impl Display for BoxShipping{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[ {}, {}, {} , {}",self.name, self.barcode, self.shipping_date, self.shipping_hour)
    }
}

impl Display for Hour{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}:{:02}",self.hour,self.minute)
    }
}

impl Display for Date{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}/{:02}/{:04}",self.day,self.month,self.year)
    }
}