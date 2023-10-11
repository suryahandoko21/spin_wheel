use actix_web::{ web::{self, Json}, HttpResponse, get, HttpRequest, post};
use crate::{adapters::api::{shared::{app_state::AppState, validate_request::validate_request}, spin_reward::query_string::QstringCompany, content::{content_mappers::ContentPresenterMapper, content_payload::ContentPayload, content_presenter::ContentPresenter}}, 
application::{usecases::{interfaces::AbstractUseCase, content::{content_by_company_code::ContentByCompannyCodeUseCase, post_content_by_company_code::PostContentByCompannyCodeUseCase}}, mappers::api_mapper::ApiMapper}};

/*  collection route for spin_rewards */
pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_content_info).
    service(post_content);

}


#[get("/info")]
async fn get_content_info(data: web::Data<AppState>,req: HttpRequest) ->HttpResponse {
    let qstring = web::Query::<QstringCompany>::from_query(req.query_string()).unwrap();
    let header_authorization =  req.headers().get("Authorization");
    let company;
    if qstring.company_code.is_none() || !header_authorization.is_none(){
        let (validate_status_code, company_code, error_request,error_validate) = validate_request(header_authorization); 
        if error_request {
            return HttpResponse::build(validate_status_code).json(error_validate);
        }  
        company= company_code;
    }
    else{
        company= qstring.company_code.as_ref().unwrap().to_string();
    } 
    let data = ContentByCompannyCodeUseCase::new(&company, &data.connection_repository);
    let  values= data.execute().await;
    let empty =ContentPresenter{
        id:0,
        title:"".to_string(),
        description:"".to_string()
    };
    if values.is_err(){
        return HttpResponse::Ok().json(empty);
    }
    return HttpResponse::Ok().json(ContentPresenterMapper::to_api(values.unwrap()));
}

#[post("/store")]
async fn post_content(data: web::Data<AppState>,post:Json<ContentPayload>,req: HttpRequest) ->HttpResponse {
   
    let header_authorization =  req.headers().get("Authorization");
    let (validate_status_code, company_code, error_request,error_validate) = validate_request(header_authorization); 
    if error_request {
        return HttpResponse::build(validate_status_code).json(error_validate);
    }  
    let data = PostContentByCompannyCodeUseCase::new(&company_code,&post, &data.connection_repository);
    let result= data.execute().await;
  
    return HttpResponse::Ok().json(result.unwrap());
}

