use crate::{models::ipn_model::Ipn, repository::mongodb_repo::MongoRepo};
use actix_web::{
    delete, get, post, put,
    web::{Data, Json, Path},
    HttpResponse
};
// use mongodb::bson::{ extjson::de::Error, oid::ObjectId};
use mongodb::bson::oid::ObjectId;

use chrono::Utc;

// fn err_handle(err: Error) -> HttpResponse{
//     let err_data = err.to_string();
//     println!("{}",err_data);
//     return HttpResponse::NotFound().body("No record");
// } 

// fn none_handle() -> HttpResponse{
//     return HttpResponse::NotFound().body("Nothing to show");
// } 

#[get("/ipn/{id}")]
pub async fn get_ipn(db:Data<MongoRepo>,path:Path<String>) -> HttpResponse{
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    }
    let ipn_detail = db.get_ipn(&id).await;

    // match ipn_detail {
    //     Ok(ipn) => HttpResponse::Ok().json(ipn),
    //     // None => none_handle()
    //     Err(_err) => HttpResponse::NotFound().body("No record")
    //     //HttpResponse::InternalServerError().body(err.to_string()),
    // }
    match ipn_detail {
        Ok(ipn) => {
            if ipn.is_some() {
                return HttpResponse::Ok().json(ipn);
            } else {
                return HttpResponse::NotFound().json("IPN with specified ID not found!");
            }
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}



#[put("/ipn/{id}")]
pub async fn update_ipn(
    db: Data<MongoRepo>,
    path: Path<String>,
    ipn_data: Json<Ipn>,
) -> HttpResponse {
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    };
    let data = Ipn {
        id: Some(ObjectId::parse_str(&id).unwrap()),
        data: ipn_data.data.to_owned(),
        created_date: ipn_data.created_date.to_owned(),
    };

    let update_result = db.update_ipn(&id, data).await;

    match update_result {
        Ok(update) => {
            if update.matched_count == 1 {
                let updated_ipn_info = db.get_ipn(&id).await;

                return match updated_ipn_info {
                    Ok(ipn) => HttpResponse::Ok().json(ipn),
                    Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
                };
            } else {
                return HttpResponse::NotFound().body("No ipn found with specified ID");
            }
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[delete("/ipn/{id}")]
pub async fn delete_ipn(db: Data<MongoRepo>, path: Path<String>) -> HttpResponse {
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    };
    let result = db.delete_ipn(&id).await;

    match result {
        Ok(res) => {
            if res.deleted_count == 1 {
                return HttpResponse::Ok().json("IPN successfully deleted!");
            } else {
                return HttpResponse::NotFound().json("IPN with specified ID not found!");
            }
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[post("/ipn")]
pub async fn create_ipn(db:Data<MongoRepo>,new_ipn:Json<Ipn>)->HttpResponse{

    let timestamp = Some(match new_ipn.created_date {
        None => Utc::now().timestamp(), // Current timestamp if None
        Some(value) => value, // Use provided timestamp value
    });

    let data = Ipn {
        id: None,
        data: new_ipn.data.to_owned(),
        created_date: timestamp,
    };

    let user_detail = db.create_ipn(data).await;

    match user_detail {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}