pub mod sql_types {
    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "prizes_categories"))]
    pub struct PrizesCategories;
}

diesel::table! {
    use diesel::sql_types::*;
    tb_spin_rewards (id) {
        id -> Int4,
        reward_name -> Varchar,
        reward_note -> Varchar,
        reward_category -> Varchar,
        reward_amount -> Int4,
        reward_money -> Int4,
        reward_status ->Varchar,
        reward_order->Int4,
        companies_code ->Varchar,
        percentage ->Double,
        reward_image -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        created_by -> Varchar,
        updated_by -> Varchar,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    tb_spin_used (id) {
        id -> Int4,
        user_id -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        created_by -> Varchar,
        updated_by -> Varchar,
        used_status -> Varchar,
        prize_id -> Int4,
        companies_code ->Varchar,
        ticket_uuid ->Varchar,
        remote_ip->Varchar
    }
}

diesel::table! {
    use diesel::sql_types::*;
    tb_companies (id) {
        id -> Int4,
        uuid -> Varchar,
        companies_code -> Varchar,
        companies_name -> Varchar,
        companies_address ->Varchar,
        is_company_enabled->Bool,
        max_credit ->Int4,
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
        userid -> Varchar,
        username -> Varchar,
        company_code->Varchar,
        ticket_id -> Int4,
        ticket_uuid -> Varchar,
        status -> Varchar,
        pointrule_id ->Int4,
        expired_date->VarChar,
        pointrule_name->VarChar,
        ticket_number ->Varchar,
        expired_type->Varchar,
        expired_value->Int4,
        created_date->Varchar,
        is_payment_gateway->Bool,
        send_be->Bool,
        }
}

diesel::table! {
    tb_spin_failed_process (id) {
        id -> Int4,
        ticket_uuid -> Varchar,
        user_id -> Varchar,
        reward_name -> Varchar,
        status -> Varchar,
        reward_type -> Varchar,
        reward_description ->VarChar,
        money -> Int4,
        post_status -> Varchar,
        url_address->VarChar,
        failed_message->Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        remote_ip ->Varchar
    }
}

diesel::table! {
    tb_spin_success_process (id) {
        id -> Int4,
        ticket_uuid -> Varchar,
        user_id -> Varchar,
        reward_name -> Varchar,
        status -> Varchar,
        reward_type -> Varchar,
        money -> Int4,
        post_status -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    tb_content (id) {
        id -> Int4,
        companies_code -> Varchar,
        content_title -> Varchar,
        content_description ->Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        created_by -> Varchar,
        updated_by -> Varchar,
        default ->Bool,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    tb_spin_logs (id) {
        id -> Int4,
        companies_code -> Varchar,
        reward_before -> Text,
        reward_after ->Text,
        reward_change ->Text,
        remote_ip ->Varchar,
        action_change ->Varchar,
        entity_type->Varchar,
        created_at -> Timestamp,
        created_by -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    tb_companies,
    tb_spin_tickets,
    tb_spin_used,
    tb_spin_rewards,
    tb_spin_success_process,
    tb_spin_failed_process,
    tb_content,
    tb_spin_logs
);
