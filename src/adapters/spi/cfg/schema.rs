pub mod sql_types {
    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "prizes_categories"))]
    pub struct PrizesCategories;
}

diesel::table! {
    use diesel::sql_types::*;
    tb_spin_promos (id) {
        id -> Int4,
        promo_amount -> Int4,
        promo_status -> Varchar,
        user_id -> Varchar,
        username-> Varchar,
        expired_at -> Timestamp,
        point_currention_time -> Timestamp,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        created_by -> Varchar,
        updated_by -> Varchar
    }
}
diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::PrizesCategories;
    tb_spin_prizes (id) {
        id -> Int4,
        prize_name -> Varchar,
        prize_note -> Varchar,
        prize_category -> Varchar,
        prize_amount -> Int4,
        prize_money -> Int4,
        companies_id -> Int4,
        percentage ->Int4
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::PrizesCategories;
    tb_spin_used (id) {
        id -> Int4,
        user_id -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        created_by -> Varchar,
        updated_by -> Varchar,
        used_status -> Varchar,
        prize_id -> Int4,
        company_id ->Int4
    }
}
diesel::table! {
    use diesel::sql_types::*;
    tb_spin_lists (id) {
        id -> Int4,
        company_code -> Varchar,
        list_status -> Varchar,
        percentage -> Int4,
        roleid-> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        created_by -> Varchar,
        updated_by -> Varchar,
        spin_prizes_id -> Int4
    }
}


diesel::table! {
    use diesel::sql_types::*;
    tb_companies (id) {
        id -> Int4,
        uuid -> Varchar,
        companies_code -> Varchar,
        companies_name -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        created_by -> Varchar,
        updated_by -> Varchar,
    }
}


diesel::table! {
    use diesel::sql_types::*;
    tb_spin_tickets(id){
        id -> Int4,
        user_uuid -> Varchar,
        ruleid -> Varchar,
        userid -> Varchar,
        username -> Varchar,
        ticket_id -> Int4,
        ticket_uuid -> Varchar,
        status -> Varchar,
        pointrule_id ->Int4,
        expired_date->VarChar
    }
}



diesel::joinable!(tb_spin_lists -> tb_spin_prizes (spin_prizes_id));


diesel::joinable!(tb_spin_used -> tb_spin_prizes (prize_id));
diesel::joinable!(tb_spin_used -> tb_companies (company_id));
diesel::joinable!(tb_spin_prizes -> tb_companies (companies_id));

diesel::allow_tables_to_appear_in_same_query!(
    tb_spin_prizes,
    tb_spin_lists,
    tb_spin_promos,
    tb_companies,
    tb_spin_tickets,
    tb_spin_used

);

