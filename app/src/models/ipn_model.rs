use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Ipn {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub data: String,
    pub created_date: Option<i64>,
}

// impl Default for Ipn {
//     fn default() -> Self {
//         Ipn {
//             created_date: Utc::today(),
//             data: String::from(""),
//             id: Option::None,
//         }
//     }
// }
  

