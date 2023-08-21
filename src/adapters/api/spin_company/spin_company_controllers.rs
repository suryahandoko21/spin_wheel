use crate::{
    adapters::{api::{    
        shared::{app_state::AppState,error_presenter::ErrorReponse, response::GenericResponse}, spin_lists::{spin_list_mappers::SpinListPrizesPresenterMapper, spin_list_presenters::SpinListsPrizesPresenter, spin_list_payloads::{SpinListPayload, SpinPostPayload}}
    }},
    application::{
        mappers::api_mapper::ApiMapper,
        usecases::{interfaces::AbstractUseCase, spin_lists::{get_all_spin_lists_usecase::GetAllSpinListsUseCase, find_by_id_usecase::GetOneSpinListsByIdUseCase, post_one_spin_list_usecase::PostSpinListsUseCase,delete_by_id_usecase::DeleteOneSpinListsByIdUseCase,update_one_spin_list_usecase::UpdateSpinListsUseCase, post_spin_by_uuid::PostSpinByUuidUseCase}}
        
    },
    domain::{error::ApiError, spin_lists_entity::{ SpinListsPrizesEntity}},


};

use actix_http::ContentEncoding;
use actix_web::{get, web::{self, Json}, HttpResponse,post, delete,patch};


/*  collection route for spin_lists */
// pub fn routes(cfg: &mut web::ServiceConfig) {
//     cfg.service(get_company_by_code);
//     // .service(delete_one_spin_list_by_id)
//     // .service(updated_one_spin_list);
// }

// #[get("/list/company/{company_id}")]
// async fn get_all_spin_prizes(data: web::Data<AppState>,path:web::Path<(String,)>) -> Result<HttpResponse, ErrorReponse> {


// }
