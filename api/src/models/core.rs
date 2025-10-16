use crate::models::users::Company;
use crate::schema::{emaillog, smtpprofiles, templates, webhooks};
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Queryable, Identifiable, Associations, Serialize, Deserialize)]
#[diesel(belongs_to(Company))]
#[diesel(table_name = smtpprofiles)]
pub struct SMTPProfile {
    pub id: i64,
    pub company_id: i64,
    pub smtp_username: String,
    pub smtp_password: String,
    pub smtp_server: String,
    pub smtp_port: i32,
    pub is_default: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = smtpprofiles)]
pub struct NewSMTPProfile {
    pub company_id: i64,
    pub smtp_username: String,
    pub smtp_password: String,
    pub smtp_server: String,
    pub smtp_port: i32,
    pub is_default: bool,
}

#[derive(Debug, Queryable, Identifiable, Associations, Serialize, Deserialize)]
#[diesel(table_name = templates)]
#[diesel(belongs_to(Company))]
pub struct Template {
    pub id: i64,
    pub company_id: i64,
    pub name: String,
    pub subject: String,
    pub content: String,
    pub template_type: String,
    pub date_created: DateTime<Utc>,
    pub date_updated: DateTime<Utc>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = templates)]
pub struct NewTemplate {
    pub company_id: i64,
    pub name: String,
    pub subject: String,
    pub content: String,
    pub template_type: String,
}

#[derive(Debug, Queryable, Identifiable, Associations, Serialize, Deserialize)]
#[diesel(table_name = webhooks)]
#[diesel(belongs_to(Company))]
pub struct Webhook {
    pub id: i64,
    pub name: String,
    pub url: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub is_active: bool,
    pub last_delivered: Option<DateTime<Utc>>,
    pub success_rate: Option<f64>,
    pub events: Option<Vec<Option<String>>>,
    pub status: Option<String>,
    pub company_id: i64,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = webhooks)]
pub struct NewWebhook {
    pub name: String,
    pub url: String,
    pub company_id: i64,
    pub events: Option<Vec<Option<String>>>,
    pub status: Option<String>,
}

#[derive(Debug, Queryable, Identifiable, Associations, Serialize, Deserialize)]
#[diesel(table_name = emaillog)]
#[diesel(belongs_to(Company))]
pub struct EmailLog {
    pub id: i64,
    pub from_email: String,
    pub to_email: String,
    pub subject: String,
    pub body: String,
    pub status: Option<String>,
    pub created_at: DateTime<Utc>,
    pub company_id: i64,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = emaillog)]
pub struct NewEmailLog {
    pub from_email: String,
    pub to_email: String,
    pub subject: String,
    pub body: String,
    pub status: Option<String>,
    pub company_id: i64,
}
