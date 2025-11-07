use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

type BitRef = Rc<RefCell<bool>>;

pub struct EntangledBit {
    value: BitRef,
}

impl Default for EntangledBit {
    fn default() -> Self {
        Self {
            value: Rc::new(RefCell::new(false))
        }
    }
}
impl EntangledBit {

    pub fn set(&self){
        *self.value.borrow_mut() = true;
    }

    pub fn reset(&self){
        *self.value.borrow_mut() = false;
    }
    pub fn get(&self) -> bool{
        *self.value.borrow()
    }

    pub fn entangle_with(&self, other: &mut Self){
        other.value = self.value.clone();
        println!("{}",Rc::strong_count(&self.value))
    }
}
