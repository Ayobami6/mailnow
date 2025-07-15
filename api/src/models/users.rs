use crate::schema::{api_keys, companies, industries, users};
use chrono::{DateTime, Utc};
use diesel::prelude::*;

#[derive(Debug, Queryable, Identifiable, Insertable)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i64,
    pub password: String,
    pub last_login: Option<DateTime<Utc>>,
    pub is_superuser: bool,
    pub email: String,
    pub firstname: Option<String>,
    pub lastname: Option<String>,
    pub is_active: bool,
    pub is_staff: bool,
    pub mfa_enabled: bool,
    pub email_verifield: bool,
    pub date_joined: DateTime<Utc>,
}

#[derive(Queryable, Insertable, Debug)]
#[diesel(table_name = industries)]
pub struct Industry {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Queryable, Identifiable, Associations, Debug)]
#[belongs_to(User, foreign_key = "owner_id")]
#[belongs_to(Industry, foreign_key = "industry_id")]
#[diesel(table_name = companies)]
pub struct Company {
    pub id: i64,
    pub company_name: String,
    pub company_address: Option<String>,
    pub website: Option<String>,
    pub sending_domain: Option<String>,
    pub default_from_name: Option<String>,
    pub default_from_email: Option<String>,
    pub owner_id: i64,
    pub industry_id: Option<i64>,
}

#[derive(Queryable, Identifiable, Associations)]
#[diesel(belongs_to(Company))]
#[diesel(table_name = api_keys)]
pub struct ApiKey {
    pub id: i64,
    pub name: String,
    pub api_key: String,
    pub created_at: DateTime<Utc>,
    pub last_used: Option<DateTime<Utc>>,
    pub expires_at: Option<DateTime<Utc>>,
    pub is_active: bool,
    pub company_id: i64,
    pub permission: Option<String>,
}
