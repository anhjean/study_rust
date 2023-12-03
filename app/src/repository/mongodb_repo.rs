use std::env;
extern crate dotenv;

use dotenv::dotenv;

use futures::stream::TryStreamExt;
use mongodb::{
    bson::{doc, extjson::de::Error, oid::ObjectId},
    results::{DeleteResult, InsertOneResult, UpdateResult},
    Client, Collection,
};

use crate::models::user_model::User;
use crate::models::ipn_model::Ipn;

pub struct MongoRepo {
    user_col: Collection<User>,
    ipn_col: Collection<Ipn>,
}

impl MongoRepo {
    pub async fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGO_URI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };
        let dbname= match env::var("DB_NAME") {
            Ok(v)=>v.to_string(),
            Err(_)=>format!("ipnDB"),
        };
        let client = Client::with_uri_str(uri)
            .await
            .expect("error connecting to database");
        let db = client.database(dbname.as_str());
        let user_col: Collection<User> = db.collection("User");
        let ipn_col: Collection<Ipn> = db.collection("Ipn");
        MongoRepo { user_col, ipn_col }
    }

    // User
    pub async fn create_user(&self, new_user: User) -> Result<InsertOneResult, Error> {
        let new_doc = User {
            id: None,
            name: new_user.name,
            location: new_user.location,
            title: new_user.title,
        };
        let user = self
            .user_col
            .insert_one(new_doc, None)
            .await
            .ok()
            .expect("Error creating user");

        Ok(user)
    }

    pub async fn get_user(&self, id: &String) -> Result<User, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let user_detail = self
            .user_col
            .find_one(filter, None)
            .await
            .ok()
            .expect("Error getting user's detail");

        Ok(user_detail.unwrap())
    }

    pub async fn update_user(&self, id: &String, new_user: User) -> Result<UpdateResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let new_doc = doc! {
            "$set":
                {
                    "id": new_user.id,
                    "name": new_user.name,
                    "location": new_user.location,
                    "title": new_user.title
                },
        };
        let updated_doc = self
            .user_col
            .update_one(filter, new_doc, None)
            .await
            .ok()
            .expect("Error updating user");
        Ok(updated_doc)
    }

    pub async fn delete_user(&self, id: &String) -> Result<DeleteResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let user_detail = self
            .user_col
            .delete_one(filter, None)
            .await
            .ok()
            .expect("Error deleting user");

        Ok(user_detail)
    }

    pub async fn get_all_users(&self) -> Result<Vec<User>, Error> {
        let mut cursors = self
            .user_col
            .find(None, None)
            .await
            .ok()
            .expect("Error getting list of users");
        let mut users: Vec<User> = Vec::new();
        while let Some(user) = cursors
            .try_next()
            .await
            .ok()
            .expect("Error mapping through cursor")
        {
            users.push(user)
        }
        Ok(users)
    }

    pub async fn create_ipn(&self, ipn_data: Ipn) -> Result<InsertOneResult, Error> {
        // let new_doc = Ipn {
        //     id: None,
        //     data: ipn_data.data,
        //     created_date: Utc::now().timestamp(),
        // };
        let ipn_record = self
            .ipn_col
            .insert_one(ipn_data, None)
            .await
            .ok()
            .expect("Error creating user");

        Ok(ipn_record)
    }

     pub async fn get_ipn(&self, id: &String) -> Result<Option<Ipn>, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let ipn_detail = self
            .ipn_col
            .find_one(filter, None)
            .await
            .ok()
            .expect("Error getting ipn's detail");

        Ok(ipn_detail)
    }

    pub async fn update_ipn(&self, id: &String, ipn: Ipn) -> Result<UpdateResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let new_doc = doc! {
            "$set":
                {
                    "id": ipn.id,
                    "data": ipn.data,
                },
        };
        let updated_doc = self
            .ipn_col
            .update_one(filter, new_doc, None)
            .await
            .ok()
            .expect("Error updating user");
        Ok(updated_doc)
    }
    pub async fn delete_ipn(&self, id: &String) -> Result<DeleteResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let ipn_detail = self
            .ipn_col
            .delete_one(filter, None)
            .await
            .ok()
            .expect("Error deleting user");

        Ok(ipn_detail)
    }
}
