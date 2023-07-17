use core::fmt;


pub enum Option {
    Add,
    Update,
    Delete,
    NotFound,
    Processed,
    Unprocessed
}


impl fmt::Display for Option  {
    fn fmt(&self,f:&mut fmt::Formatter)->fmt::Result{
        match self{
            Option::Add=>write!(f, "Data Aded"),
            Option::Update=>write!(f, "Data Updated"),
            Option::Delete=>write!(f, "Data Deleted"),
            Option::NotFound=>write!(f, "Data Not Found"),
            Option::Processed=>write!(f, "Processed"),
            Option::Unprocessed=>write!(f, "Unprocessed")

        }
    }
}
pub enum Status {
    Success,
    Fail,
    Successall,
    Partial
}


impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Status::Success => write!(f, "Success"),
            Status::Fail => write!(f, "Failed"),
            Status::Successall =>write!(f,"Success All"),
            Status::Partial =>write!(f,"Partial")
        }
    }
}