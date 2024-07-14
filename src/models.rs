use serde::{Serialize, Deserialize};
use chrono::{NaiveDate, NaiveDateTime};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct CreateParcel {
    pub sku: String,
    pub description: String,
    pub delivery_address: serde_json::Value,
    pub delivery_date: String, // Use String to parse it properly in the handler
    pub shipping_cost: serde_json::Value,
}

#[derive(Serialize, Deserialize)]
pub struct Parcel {
    pub id: Uuid,
    pub sku: String,
    pub description: String,
    pub delivery_address: serde_json::Value,
    pub delivery_date: NaiveDateTime, // Use NaiveDateTime to be consistent with DB storage
    pub shipping_cost: serde_json::Value,
}
