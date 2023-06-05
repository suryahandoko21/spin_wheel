#[derive(Debug, Clone)]
pub struct SpinPrizesEntity {
    pub prize_id: i32,
    pub prize_weight: i32,
    pub prize_name: String,
    pub prize_note: String,
    pub prize_category: String,
    pub prize_amount: i32
}

impl SpinPrizesEntity {
    pub fn new(
        prize_id: i32,
        prize_weight: i32,
        prize_name: String,
        prize_note: String,
        prize_category: String,
        prize_amount: i32
    )->Self{
        SpinPrizesEntity{
            prize_id,
            prize_weight,
            prize_name,
            prize_note,
            prize_category,
            prize_amount
        }

    }
}

