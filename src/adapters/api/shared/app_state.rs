use crate::adapters::spi::cfg::db_connection::ConnectionRepository;

pub struct AppState {
    pub app_name: String,
    pub connection_repository: ConnectionRepository,
}
