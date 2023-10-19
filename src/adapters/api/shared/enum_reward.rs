use core::fmt;

pub enum RewardEnum {
    ZONK,
    PRODUCT,
    CASH,
}
impl fmt::Display for RewardEnum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RewardEnum::ZONK => write!(f, "NONE"),
            RewardEnum::PRODUCT => write!(f, "PRODUCT"),
            RewardEnum::CASH => write!(f, "MONEY"),
        }
    }
}
