use core::fmt;


pub enum Status {
    success,
    fail
}


impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Status::success => write!(f, "Success"),
            Status::fail => write!(f, "Failure"),
          
        }
    }
}