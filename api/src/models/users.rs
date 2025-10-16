use crate::schema::{api_keys, companies, industries, users, team_members};
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Queryable, Identifiable, Serialize, Deserialize)]
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
    pub email_verified: bool,
    pub user_type: String,
    pub date_joined: DateTime<Utc>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub password: String,
    pub email: String,
    pub firstname: Option<String>,
    pub lastname: Option<String>,
    pub user_type: String,
    pub is_active: bool,
    pub is_staff: bool,
    pub is_superuser: bool,
    pub mfa_enabled: bool,
    pub email_verified: bool,
    pub date_joined: DateTime<Utc>,
}

#[derive(Debug, Queryable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = industries)]
pub struct Industry {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = industries)]
pub struct NewIndustry {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Queryable, Identifiable, Associations, Serialize, Deserialize)]
#[diesel(table_name = companies)]
#[diesel(belongs_to(User, foreign_key = owner_id))]
#[diesel(belongs_to(Industry, foreign_key = industry_id))]
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

#[derive(Debug, Insertable)]
#[diesel(table_name = companies)]
pub struct NewCompany {
    pub company_name: String,
    pub company_address: Option<String>,
    pub website: Option<String>,
    pub sending_domain: Option<String>,
    pub default_from_name: Option<String>,
    pub default_from_email: Option<String>,
    pub owner_id: i64,
    pub industry_id: Option<i64>,
}

#[derive(Debug, Queryable, Identifiable, Associations, Serialize, Deserialize)]
#[diesel(table_name = api_keys)]
#[diesel(belongs_to(Company))]
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

#[derive(Debug, Insertable)]
#[diesel(table_name = api_keys)]
pub struct NewApiKey {
    pub name: String,
    pub api_key: String,
    pub company_id: i64,
    pub permission: Option<String>,
}





#[derive(Debug, Queryable, Identifiable, Associations, Serialize, Deserialize)]
#[diesel(table_name = team_members)]
#[diesel(belongs_to(User, foreign_key = user_id))]
#[diesel(belongs_to(Company, foreign_key = company_id))]
pub struct TeamMember {
    pub id: i64,
    pub role: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub company_id: i64,
    pub user_id: i64,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = team_members)]
pub struct NewTeamMember {
    pub role: String,
    pub company_id: i64,
    pub user_id: i64,
}