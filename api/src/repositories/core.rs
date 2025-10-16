use crate::models::core::{SMTPProfile, NewSMTPProfile, Template, NewTemplate, Webhook, NewWebhook, EmailLog, NewEmailLog};
use crate::schema::{smtpprofiles, templates, webhooks, emaillog};
use diesel::prelude::*;
use super::DbPool;

pub trait CoreRepository {
    fn create_smtp_profile(&self, new_profile: NewSMTPProfile) -> Result<SMTPProfile, diesel::result::Error>;
    fn get_smtp_profiles_by_company(&self, company_id: i64) -> Result<Vec<SMTPProfile>, diesel::result::Error>;
    fn create_template(&self, new_template: NewTemplate) -> Result<Template, diesel::result::Error>;
    fn get_templates_by_company(&self, company_id: i64) -> Result<Vec<Template>, diesel::result::Error>;
    fn create_webhook(&self, new_webhook: NewWebhook) -> Result<Webhook, diesel::result::Error>;
    fn get_webhooks_by_company(&self, company_id: i64) -> Result<Vec<Webhook>, diesel::result::Error>;
    fn create_email_log(&self, new_log: NewEmailLog) -> Result<EmailLog, diesel::result::Error>;
    fn get_email_logs_by_company(&self, company_id: i64) -> Result<Vec<EmailLog>, diesel::result::Error>;
}

#[derive(Clone)]
pub struct CoreRepositoryImpl {
    pool: DbPool,
}

impl CoreRepositoryImpl {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

impl CoreRepository for CoreRepositoryImpl {
    fn create_smtp_profile(&self, new_profile: NewSMTPProfile) -> Result<SMTPProfile, diesel::result::Error> {
        let mut conn = self.pool.get().expect("Failed to get DB connection");
        diesel::insert_into(smtpprofiles::table)
            .values(&new_profile)
            .get_result(&mut conn)
    }

    fn get_smtp_profiles_by_company(&self, company_id: i64) -> Result<Vec<SMTPProfile>, diesel::result::Error> {
        let mut conn = self.pool.get().expect("Failed to get DB connection");
        smtpprofiles::table
            .filter(smtpprofiles::company_id.eq(company_id))
            .load(&mut conn)
    }

    fn create_template(&self, new_template: NewTemplate) -> Result<Template, diesel::result::Error> {
        let mut conn = self.pool.get().expect("Failed to get DB connection");
        diesel::insert_into(templates::table)
            .values(&new_template)
            .get_result(&mut conn)
    }

    fn get_templates_by_company(&self, company_id: i64) -> Result<Vec<Template>, diesel::result::Error> {
        let mut conn = self.pool.get().expect("Failed to get DB connection");
        templates::table
            .filter(templates::company_id.eq(company_id))
            .load(&mut conn)
    }

    fn create_webhook(&self, new_webhook: NewWebhook) -> Result<Webhook, diesel::result::Error> {
        let mut conn = self.pool.get().expect("Failed to get DB connection");
        diesel::insert_into(webhooks::table)
            .values(&new_webhook)
            .get_result(&mut conn)
    }

    fn get_webhooks_by_company(&self, company_id: i64) -> Result<Vec<Webhook>, diesel::result::Error> {
        let mut conn = self.pool.get().expect("Failed to get DB connection");
        webhooks::table
            .filter(webhooks::company_id.eq(company_id))
            .load(&mut conn)
    }

    fn create_email_log(&self, new_log: NewEmailLog) -> Result<EmailLog, diesel::result::Error> {
        let mut conn = self.pool.get().expect("Failed to get DB connection");
        diesel::insert_into(emaillog::table)
            .values(&new_log)
            .get_result(&mut conn)
    }

    fn get_email_logs_by_company(&self, company_id: i64) -> Result<Vec<EmailLog>, diesel::result::Error> {
        let mut conn = self.pool.get().expect("Failed to get DB connection");
        emaillog::table
            .filter(emaillog::company_id.eq(company_id))
            .load(&mut conn)
    }
}