// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "prizes_categories"))]
    pub struct PrizesCategories;
}

diesel::table! {
    tb_spin_lists (id) {
        id -> Int8,
        company_code -> Varchar,
        list_status -> Nullable<Varchar>,
        quantity -> Int4,
        prize_id -> Int4,
        created_at -> Nullable<Timestamp>,
        update_at -> Nullable<Timestamp>,
        created_by -> Nullable<Timestamp>,
        update_by -> Nullable<Timestamp>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::PrizesCategories;

    tb_spin_prizes (id) {
        id -> Int4,
        prize_name -> Varchar,
        prize_note -> Nullable<Varchar>,
        prize_category -> Nullable<PrizesCategories>,
        prize_amount -> Nullable<Int4>,
    }
}




diesel::table! {
    use diesel::sql_types::*;

    tb_spin_reward (id) {
        id -> Int4,
        reward_name -> Varchar,
        reward_note -> Nullable<Varchar>,
        reward_category -> String,
        prize_amount -> Nullable<Int4>,
    }
}
diesel::table! {
    tb_spin_promos (id) {
        id -> Int4,
        promo_amount -> Nullable<Int4>,
        code_user -> Varchar,
        company_code -> Varchar,
        expired_at -> Nullable<Timestamp>,
        point_currention_time -> Nullable<Timestamp>,
        created_at -> Nullable<Timestamp>,
        update_at -> Nullable<Timestamp>,
        created_by -> Nullable<Varchar>,
        update_by -> Nullable<Varchar>,
    }
}

diesel::joinable!(tb_spin_lists -> tb_spin_prizes (prize_id));

diesel::allow_tables_to_appear_in_same_query!(
    tb_spin_lists,
    tb_spin_prizes,
    tb_spin_promos,
);
