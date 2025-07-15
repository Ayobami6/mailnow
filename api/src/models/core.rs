use crate::models::users::Company;
use crate::schema::webhooks;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

// If the events field is an array of varchar, use Vec<String>
#[derive(Debug, Queryable, Identifiable, Associations, Serialize, Deserialize)]
#[diesel(table_name = webhooks)]
#[diesel(belongs_to(Company, foreign_key = company_id))]
pub struct Webhook {
    pub id: i64,
    pub name: String,
    pub url: String,
    pub created_at: chrono::NaiveDateTime, // If you use chrono for timestamps
    pub updated_at: chrono::NaiveDateTime,
    pub is_active: bool,
    pub last_delivered: Option<chrono::NaiveDateTime>,
    pub success_rate: Option<f64>,
    pub events: Option<Vec<String>>, // vec for arrays
    pub status: Option<String>,
    pub company_id: i64,
}
