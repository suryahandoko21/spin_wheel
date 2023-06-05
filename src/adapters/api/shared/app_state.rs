use crate::adapters::spi::{prizes::repository::SpinPrizesRepository};

pub struct AppState {
    pub app_name: String,
    pub spin_prize_repository: SpinPrizesRepository,

}
