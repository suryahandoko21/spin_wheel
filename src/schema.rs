// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "prizes_categories"))]
    pub struct PrizesCategories;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::PrizesCategories;

    tb_spin_prizes (id) {
        id -> Int4,
        prize_weight -> Int4,
        prize_name -> Varchar,
        prize_note -> Varchar,
        prize_category -> Varchar,
        prize_amount -> Int4,
    }
}
