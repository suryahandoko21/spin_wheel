use core::fmt;


pub enum Options {
    Add,
    Update,
    Delete,
    NotFound,
    Processed,
    Unprocessed
}


impl fmt::Display for Options  {
    fn fmt(&self,f:&mut fmt::Formatter)->fmt::Result{
        match self{
            Options::Add=>write!(f, "Data Added"),
            Options::Update=>write!(f, "Data Updated"),
            Options::Delete=>write!(f, "Data Deleted"),
            Options::NotFound=>write!(f, "Data Not Found"),
            Options::Processed=>write!(f, "Processed"),
            Options::Unprocessed=>write!(f, "Unprocessed")

        }
    }
}
pub enum Status {
    Success,
    Fail,
    Successall,
    Partial,
    Failall,
    PercentageMismatch,
    DataAdd,
    DataExist,
    DataUpdated,
    DataNotExist
}


impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Status::Success => write!(f, "Success"),
            Status::Fail => write!(f, "Failed"),
            Status::Successall =>write!(f,"Success All"),
            Status::Partial =>write!(f,"Partial"),
            Status::Failall =>write!(f,"Failed All"),
            Status::PercentageMismatch =>write!(f,"Not allowed,Percentage must 100 %"),
            Status::DataAdd => write!(f, "Data Add"),
            Status::DataExist=> write!(f,"Data Exist"),
            Status::DataUpdated=> write!(f,"Data Updated"),
            Status::DataNotExist=> write!(f,"Data Not Exist"),
        }
    }
}