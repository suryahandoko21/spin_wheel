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
        prize_amount -> Int4
    }
}
diesel::table! {
    use diesel::sql_types::*;
    tb_spin_lists (id) {
        id -> Int4,
        company_code -> Varchar,
        list_status -> Varchar,
        quantity -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        created_by -> Varchar,
        updated_by -> Varchar,
        spin_prizes_id -> Int4
    }
}



diesel::joinable!(tb_spin_lists -> tb_spin_prizes (spin_prizes_id));

diesel::allow_tables_to_appear_in_same_query!(
    tb_spin_prizes,
    tb_spin_lists,

);

