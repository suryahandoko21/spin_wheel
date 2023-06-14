use core::fmt;


pub enum Option {
    Add,
    Update,
    Delete,
    NotFound
}


impl fmt::Display for Option  {
    fn fmt(&self,f:&mut fmt::Formatter)->fmt::Result{
        match self{
            Option::Add=>write!(f, "Data Added"),
            Option::Update=>write!(f, "Data Updated"),
            Option::Delete=>write!(f, "Data Deleted"),
            Option::NotFound=>write!(f, "Data Not Found"),

        }
    }
}
pub enum Status {
    Success,
    Fail
}


impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Status::Success => write!(f, "Success"),
            Status::Fail => write!(f, "Failure"),
          
        }
    }
}