use super::DbPool;
use crate::models::users::{
    ApiKey, Company, EmailLog, Industry, NewApiKey, NewCompany, NewEmailLog, NewIndustry,
    NewSmtpProfile, NewTeamMember, NewUser, SmtpProfile, TeamMember, User, Template, NewTemplate,
};
use crate::schema::{api_keys, companies, emaillog, industries, smtpprofiles, team_members, users, templates};
use diesel::prelude::*;

pub trait UserRepository {
    fn create_user(&self, new_user: NewUser) -> Result<User, diesel::result::Error>;
    fn get_user_by_id(&self, user_id: i64) -> Result<User, diesel::result::Error>;
    fn get_user_by_email(&self, email: &str) -> Result<User, diesel::result::Error>;
    fn update_user(&self, user_id: i64, user: &User) -> Result<User, diesel::result::Error>;
    fn delete_user(&self, user_id: i64) -> Result<usize, diesel::result::Error>;

    fn create_company(&self, new_company: NewCompany) -> Result<Company, diesel::result::Error>;
    fn get_company_by_id(&self, company_id: i64) -> Result<Company, diesel::result::Error>;
    fn get_companies_by_owner(&self, owner_id: i64) -> Result<Vec<Company>, diesel::result::Error>;

    fn create_industry(&self, new_industry: NewIndustry)
        -> Result<Industry, diesel::result::Error>;
    fn get_all_industries(&self) -> Result<Vec<Industry>, diesel::result::Error>;

    fn create_api_key(&self, new_api_key: NewApiKey) -> Result<ApiKey, diesel::result::Error>;
    fn get_api_keys_by_company(
        &self,
        company_id: i64,
    ) -> Result<Vec<ApiKey>, diesel::result::Error>;
    fn get_api_key_by_key(&self, key: &str) -> Result<ApiKey, diesel::result::Error>;
    fn delete_api_key(
        &self,
        api_key_id: i64,
        company_id: i64,
    ) -> Result<usize, diesel::result::Error>;
    fn get_user_role_in_company(
        &self,
        user_id: i64,
        company_id: i64,
    ) -> Result<String, diesel::result::Error>;

    fn create_team_member(
        &self,
        new_member: NewTeamMember,
    ) -> Result<TeamMember, diesel::result::Error>;
    fn get_team_members_by_company(
        &self,
        company_id: i64,
    ) -> Result<Vec<TeamMember>, diesel::result::Error>;
    fn get_team_members_by_user(
        &self,
        user_id: i64,
    ) -> Result<Vec<TeamMember>, diesel::result::Error>;
    fn verify_user_email(&self, email: &str) -> Result<User, diesel::result::Error>;
    fn verify_user_by_id(&self, user_id: i64) -> Result<User, diesel::result::Error>;
    fn update_company_credits(
        &self,
        company_id: i64,
        credits: i64,
    ) -> Result<Company, diesel::result::Error>;
    fn reset_company_credits(
        &self,
        company_id: i64,
        tier: &str,
    ) -> Result<Company, diesel::result::Error>;
    fn deduct_api_credit(&self, company_id: i64) -> Result<Company, diesel::result::Error>;

    fn create_smtp_profile(
        &self,
        new_profile: NewSmtpProfile,
    ) -> Result<SmtpProfile, diesel::result::Error>;
    fn get_smtp_profiles_by_company(
        &self,
        company_id: i64,
    ) -> Result<Vec<SmtpProfile>, diesel::result::Error>;
    fn get_smtp_profile_by_id(&self, profile_id: i64)
        -> Result<SmtpProfile, diesel::result::Error>;
    fn update_smtp_profile(
        &self,
        profile_id: i64,
        profile: &SmtpProfile,
    ) -> Result<SmtpProfile, diesel::result::Error>;
    fn delete_smtp_profile(
        &self,
        profile_id: i64,
        company_id: i64,
    ) -> Result<usize, diesel::result::Error>;
    fn set_default_smtp_profile(
        &self,
        profile_id: i64,
        company_id: i64,
    ) -> Result<SmtpProfile, diesel::result::Error>;

    fn create_email_log(&self, new_log: NewEmailLog) -> Result<EmailLog, diesel::result::Error>;
    fn get_default_smtp_profile(
        &self,
        company_id: i64,
    ) -> Result<SmtpProfile, diesel::result::Error>;
    fn update_email_log_status(
        &self,
        log_id: i64,
        status: &str,
    ) -> Result<EmailLog, diesel::result::Error>;
    fn get_email_logs_by_company(
        &self,
        company_id: i64,
    ) -> Result<Vec<EmailLog>, diesel::result::Error>;
    fn get_email_log_stats(
        &self,
        company_id: i64,
    ) -> Result<(i64, i64, i64, i64), diesel::result::Error>;
    
    fn create_template(&self, new_template: NewTemplate) -> Result<Template, diesel::result::Error>;
    fn get_templates_by_company(&self, company_id: i64) -> Result<Vec<Template>, diesel::result::Error>;
    fn get_template_by_id(&self, template_id: i64, company_id: i64) -> Result<Template, diesel::result::Error>;
    fn update_template(&self, template_id: i64, template: &Template) -> Result<Template, diesel::result::Error>;
    fn delete_template(&self, template_id: i64, company_id: i64) -> Result<usize, diesel::result::Error>;
    fn update_company(&self, company_id: i64, company: &Company) -> Result<Company, diesel::result::Error>;
}

#[derive(Clone)]
pub struct UserRepositoryImpl {
    pool: DbPool,
}

impl UserRepositoryImpl {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

impl UserRepository for UserRepositoryImpl {
    fn create_user(&self, new_user: NewUser) -> Result<User, diesel::result::Error> {
        log::debug!("Creating user with email: {}", new_user.email);
        let mut conn = self.pool.get().expect("Failed to get DB connection");

        let result: Result<User, diesel::result::Error> = diesel::insert_into(users::table)
            .values(&new_user)
            .get_result(&mut conn);

        match &result {
            Ok(user) => log::info!("User created successfully with ID: {}", user.id),
            Err(e) => log::error!("Failed to create user: {:?}", e),
        }

        result
    }

    fn get_user_by_id(&self, user_id: i64) -> Result<User, diesel::result::Error> {
        log::debug!("Fetching user by ID: {}", user_id);
        let mut conn = self.pool.get().expect("Failed to get DB connection");

        let result = users::table.find(user_id).first::<User>(&mut conn);

        match &result {
            Ok(_) => log::debug!("User found with ID: {}", user_id),
            Err(diesel::result::Error::NotFound) => {
                log::warn!("User not found with ID: {}", user_id)
            }
            Err(e) => log::error!("Database error fetching user {}: {:?}", user_id, e),
        }

        result
    }

    fn get_user_by_email(&self, email: &str) -> Result<User, diesel::result::Error> {
        log::debug!("Fetching user by email: {}", email);
        let mut conn = self.pool.get().expect("Failed to get DB connection");

        let result = users::table
            .filter(users::email.eq(email))
            .first::<User>(&mut conn);

        match &result {
            Ok(user) => log::debug!("User found with email: {} (ID: {})", email, user.id),
            Err(diesel::result::Error::NotFound) => {
                log::warn!("User not found with email: {}", email)
            }
            Err(e) => log::error!("Database error fetching user by email {}: {:?}", email, e),
        }

        result
    }

    fn update_user(&self, user_id: i64, user: &User) -> Result<User, diesel::result::Error> {
        log::debug!("Updating user with ID: {}", user_id);
        let mut conn = self.pool.get().expect("Failed to get DB connection");

        let result = diesel::update(users::table.find(user_id))
            .set((
                users::email.eq(&user.email),
                users::firstname.eq(&user.firstname),
                users::lastname.eq(&user.lastname),
                users::is_active.eq(user.is_active),
                users::email_verified.eq(user.email_verified),
                users::user_type.eq(&user.user_type),
            ))
            .get_result::<User>(&mut conn);

        match &result {
            Ok(_) => log::info!("User updated successfully with ID: {}", user_id),
            Err(e) => log::error!("Failed to update user {}: {:?}", user_id, e),
        }

        result
    }

    fn delete_user(&self, user_id: i64) -> Result<usize, diesel::result::Error> {
        log::debug!("Deleting user with ID: {}", user_id);
        let mut conn = self.pool.get().expect("Failed to get DB connection");

        let result = diesel::delete(users::table.find(user_id)).execute(&mut conn);

        match &result {
            Ok(count) => {
                if *count > 0 {
                    log::info!("User deleted successfully with ID: {}", user_id);
                } else {
                    log::warn!("No user found to delete with ID: {}", user_id);
                }
            }
            Err(e) => log::error!("Failed to delete user {}: {:?}", user_id, e),
        }

        result
    }

    fn create_company(&self, new_company: NewCompany) -> Result<Company, diesel::result::Error> {
        log::debug!("Creating company: {}", new_company.company_name);
        let mut conn = self.pool.get().expect("Failed to get DB connection");

        let result = diesel::insert_into(companies::table)
            .values(&new_company)
            .get_result::<Company>(&mut conn);

        match &result {
            Ok(company) => log::info!("Company created successfully with ID: {}", company.id),
            Err(e) => log::error!("Failed to create company: {:?}", e),
        }

        result
    }

    fn get_company_by_id(&self, company_id: i64) -> Result<Company, diesel::result::Error> {
        log::debug!("Fetching company by ID: {}", company_id);
        let mut conn = self.pool.get().expect("Failed to get DB connection");
        companies::table
            .find(company_id)
            .first::<Company>(&mut conn)
    }

    fn get_companies_by_owner(&self, owner_id: i64) -> Result<Vec<Company>, diesel::result::Error> {
        log::debug!("Fetching companies for owner: {}", owner_id);
        let mut conn = self.pool.get().expect("Failed to get DB connection");
        companies::table
            .filter(companies::owner_id.eq(owner_id))
            .load::<Company>(&mut conn)
    }

    fn create_industry(
        &self,
        new_industry: NewIndustry,
    ) -> Result<Industry, diesel::result::Error> {
        log::debug!("Creating industry: {}", new_industry.name);
        let mut conn = self.pool.get().expect("Failed to get DB connection");
        diesel::insert_into(industries::table)
            .values(&new_industry)
            .get_result::<Industry>(&mut conn)
    }

    fn get_all_industries(&self) -> Result<Vec<Industry>, diesel::result::Error> {
        log::debug!("Fetching all industries");
        let mut conn = self.pool.get().expect("Failed to get DB connection");
        industries::table.load::<Industry>(&mut conn)
    }

    fn create_api_key(&self, new_api_key: NewApiKey) -> Result<ApiKey, diesel::result::Error> {
        log::debug!("Creating API key: {}", new_api_key.name);
        let mut conn = self.pool.get().expect("Failed to get DB connection");
        diesel::insert_into(api_keys::table)
            .values(&new_api_key)
            .get_result::<ApiKey>(&mut conn)
    }

    fn get_api_keys_by_company(
        &self,
        company_id: i64,
    ) -> Result<Vec<ApiKey>, diesel::result::Error> {
        log::debug!("Fetching API keys for company: {}", company_id);
        let mut conn = self.pool.get().expect("Failed to get DB connection");
        api_keys::table
            .filter(api_keys::company_id.eq(company_id))
            .load::<ApiKey>(&mut conn)
    }

    fn get_api_key_by_key(&self, key: &str) -> Result<ApiKey, diesel::result::Error> {
        log::debug!("Fetching API key by key");
        let mut conn = self.pool.get().expect("Failed to get DB connection");
        api_keys::table
            .filter(api_keys::api_key.eq(key))
            .first::<ApiKey>(&mut conn)
    }

    fn create_team_member(
        &self,
        new_member: NewTeamMember,
    ) -> Result<TeamMember, diesel::result::Error> {
        log::debug!("Creating team member with role: {}", new_member.role);
        let mut conn = self.pool.get().expect("Failed to get DB connection");
        diesel::insert_into(team_members::table)
            .values(&new_member)
            .get_result::<TeamMember>(&mut conn)
    }

    fn get_team_members_by_company(
        &self,
        company_id: i64,
    ) -> Result<Vec<TeamMember>, diesel::result::Error> {
        log::debug!("Fetching team members for company: {}", company_id);
        let mut conn = self.pool.get().expect("Failed to get DB connection");
        team_members::table
            .filter(team_members::company_id.eq(company_id))
            .load::<TeamMember>(&mut conn)
    }

    fn get_team_members_by_user(
        &self,
        user_id: i64,
    ) -> Result<Vec<TeamMember>, diesel::result::Error> {
        log::debug!("Fetching team memberships for user: {}", user_id);
        let mut conn = self.pool.get().expect("Failed to get DB connection");
        team_members::table
            .filter(team_members::user_id.eq(user_id))
            .load::<TeamMember>(&mut conn)
    }

    fn verify_user_email(&self, email: &str) -> Result<User, diesel::result::Error> {
        log::debug!("Verifying email for user: {}", email);
        let mut conn = self.pool.get().expect("Failed to get DB connection");

        let result = diesel::update(users::table.filter(users::email.eq(email)))
            .set(users::email_verified.eq(true))
            .get_result::<User>(&mut conn);

        match &result {
            Ok(user) => log::info!(
                "Email verified successfully for user: {} (ID: {})",
                email,
                user.id
            ),
            Err(e) => log::error!("Failed to verify email for {}: {:?}", email, e),
        }

        result
    }

    fn verify_user_by_id(&self, user_id: i64) -> Result<User, diesel::result::Error> {
        log::debug!("Verifying email for user ID: {}", user_id);
        let mut conn = self.pool.get().expect("Failed to get DB connection");

        let result = diesel::update(users::table.filter(users::id.eq(user_id)))
            .set(users::email_verified.eq(true))
            .get_result::<User>(&mut conn);

        match &result {
            Ok(user) => log::info!(
                "Email verified successfully for user ID: {} ({})",
                user_id,
                user.email
            ),
            Err(e) => log::error!("Failed to verify email for user ID {}: {:?}", user_id, e),
        }

        result
    }

    fn update_company_credits(
        &self,
        company_id: i64,
        credits: i64,
    ) -> Result<Company, diesel::result::Error> {
        log::debug!(
            "Updating credits for company ID: {} to {}",
            company_id,
            credits
        );
        let mut conn = self.pool.get().expect("Failed to get DB connection");

        diesel::update(companies::table.filter(companies::id.eq(company_id)))
            .set(companies::api_credits.eq(credits))
            .get_result::<Company>(&mut conn)
    }

    fn reset_company_credits(
        &self,
        company_id: i64,
        tier: &str,
    ) -> Result<Company, diesel::result::Error> {
        use crate::utils::pricing::{get_next_reset_date, PricingTier};

        let pricing_tier = PricingTier::from_str(tier);
        let new_credits = pricing_tier.monthly_credits();
        let next_reset = get_next_reset_date();

        log::debug!(
            "Resetting credits for company ID: {} to {} ({})",
            company_id,
            new_credits,
            tier
        );
        let mut conn = self.pool.get().expect("Failed to get DB connection");

        diesel::update(companies::table.filter(companies::id.eq(company_id)))
            .set((
                companies::api_credits.eq(new_credits),
                companies::credits_reset_date.eq(next_reset),
            ))
            .get_result::<Company>(&mut conn)
    }

    fn deduct_api_credit(&self, company_id: i64) -> Result<Company, diesel::result::Error> {
        log::debug!("Deducting 1 API credit for company ID: {}", company_id);
        let mut conn = self.pool.get().expect("Failed to get DB connection");

        diesel::update(companies::table.filter(companies::id.eq(company_id)))
            .set(companies::api_credits.eq(companies::api_credits - 1))
            .get_result::<Company>(&mut conn)
    }

    fn delete_api_key(
        &self,
        api_key_id: i64,
        company_id: i64,
    ) -> Result<usize, diesel::result::Error> {
        log::debug!(
            "Deleting API key ID: {} for company: {}",
            api_key_id,
            company_id
        );
        let mut conn = self.pool.get().expect("Failed to get DB connection");

        diesel::delete(
            api_keys::table
                .filter(api_keys::id.eq(api_key_id))
                .filter(api_keys::company_id.eq(company_id)),
        )
        .execute(&mut conn)
    }

    fn get_user_role_in_company(
        &self,
        user_id: i64,
        company_id: i64,
    ) -> Result<String, diesel::result::Error> {
        log::debug!(
            "Getting user role for user: {} in company: {}",
            user_id,
            company_id
        );
        let mut conn = self.pool.get().expect("Failed to get DB connection");

        // Check if user is company owner
        let company = companies::table
            .find(company_id)
            .first::<Company>(&mut conn)?;
        if company.owner_id == user_id {
            return Ok("Owner".to_string());
        }

        // Check team member role
        let team_member = team_members::table
            .filter(team_members::user_id.eq(user_id))
            .filter(team_members::company_id.eq(company_id))
            .first::<TeamMember>(&mut conn)?;

        Ok(team_member.role)
    }

    fn create_smtp_profile(
        &self,
        new_profile: NewSmtpProfile,
    ) -> Result<SmtpProfile, diesel::result::Error> {
        log::debug!(
            "Creating SMTP profile for company: {}",
            new_profile.company_id
        );
        let mut conn = self.pool.get().expect("Failed to get DB connection");
        diesel::insert_into(smtpprofiles::table)
            .values(&new_profile)
            .get_result::<SmtpProfile>(&mut conn)
    }

    fn get_smtp_profiles_by_company(
        &self,
        company_id: i64,
    ) -> Result<Vec<SmtpProfile>, diesel::result::Error> {
        log::debug!("Fetching SMTP profiles for company: {}", company_id);
        let mut conn = self.pool.get().expect("Failed to get DB connection");
        smtpprofiles::table
            .filter(smtpprofiles::company_id.eq(company_id))
            .load::<SmtpProfile>(&mut conn)
    }

    fn get_smtp_profile_by_id(
        &self,
        profile_id: i64,
    ) -> Result<SmtpProfile, diesel::result::Error> {
        log::debug!("Fetching SMTP profile by ID: {}", profile_id);
        let mut conn = self.pool.get().expect("Failed to get DB connection");
        smtpprofiles::table
            .find(profile_id)
            .first::<SmtpProfile>(&mut conn)
    }

    fn update_smtp_profile(
        &self,
        profile_id: i64,
        profile: &SmtpProfile,
    ) -> Result<SmtpProfile, diesel::result::Error> {
        log::debug!("Updating SMTP profile: {}", profile_id);
        let mut conn = self.pool.get().expect("Failed to get DB connection");
        diesel::update(smtpprofiles::table.find(profile_id))
            .set((
                smtpprofiles::smtp_username.eq(&profile.smtp_username),
                smtpprofiles::smtp_password.eq(&profile.smtp_password),
                smtpprofiles::smtp_server.eq(&profile.smtp_server),
                smtpprofiles::smtp_port.eq(profile.smtp_port),
                smtpprofiles::is_default.eq(profile.is_default),
                smtpprofiles::updated_at.eq(chrono::Utc::now()),
            ))
            .get_result::<SmtpProfile>(&mut conn)
    }

    fn delete_smtp_profile(
        &self,
        profile_id: i64,
        company_id: i64,
    ) -> Result<usize, diesel::result::Error> {
        log::debug!(
            "Deleting SMTP profile: {} for company: {}",
            profile_id,
            company_id
        );
        let mut conn = self.pool.get().expect("Failed to get DB connection");
        diesel::delete(
            smtpprofiles::table
                .filter(smtpprofiles::id.eq(profile_id))
                .filter(smtpprofiles::company_id.eq(company_id)),
        )
        .execute(&mut conn)
    }

    fn set_default_smtp_profile(
        &self,
        profile_id: i64,
        company_id: i64,
    ) -> Result<SmtpProfile, diesel::result::Error> {
        log::debug!(
            "Setting default SMTP profile: {} for company: {}",
            profile_id,
            company_id
        );
        let mut conn = self.pool.get().expect("Failed to get DB connection");

        // First, unset all defaults for the company
        diesel::update(smtpprofiles::table.filter(smtpprofiles::company_id.eq(company_id)))
            .set(smtpprofiles::is_default.eq(false))
            .execute(&mut conn)?;

        // Then set the specified profile as default
        diesel::update(
            smtpprofiles::table
                .filter(smtpprofiles::id.eq(profile_id))
                .filter(smtpprofiles::company_id.eq(company_id)),
        )
        .set((
            smtpprofiles::is_default.eq(true),
            smtpprofiles::updated_at.eq(chrono::Utc::now()),
        ))
        .get_result::<SmtpProfile>(&mut conn)
    }

    fn create_email_log(&self, new_log: NewEmailLog) -> Result<EmailLog, diesel::result::Error> {
        log::debug!("Creating email log for company: {}", new_log.company_id);
        let mut conn = self.pool.get().expect("Failed to get DB connection");
        diesel::insert_into(emaillog::table)
            .values(&new_log)
            .get_result::<EmailLog>(&mut conn)
    }

    fn get_default_smtp_profile(
        &self,
        company_id: i64,
    ) -> Result<SmtpProfile, diesel::result::Error> {
        log::debug!("Fetching default SMTP profile for company: {}", company_id);
        let mut conn = self.pool.get().expect("Failed to get DB connection");
        smtpprofiles::table
            .filter(smtpprofiles::company_id.eq(company_id))
            .filter(smtpprofiles::is_default.eq(true))
            .first::<SmtpProfile>(&mut conn)
    }

    fn update_email_log_status(
        &self,
        log_id: i64,
        status: &str,
    ) -> Result<EmailLog, diesel::result::Error> {
        log::debug!("Updating email log status for ID: {} to {}", log_id, status);
        let mut conn = self.pool.get().expect("Failed to get DB connection");
        diesel::update(emaillog::table.find(log_id))
            .set(emaillog::status.eq(Some(status.to_string())))
            .get_result::<EmailLog>(&mut conn)
    }

    fn get_email_logs_by_company(
        &self,
        company_id: i64,
    ) -> Result<Vec<EmailLog>, diesel::result::Error> {
        log::debug!("Fetching email logs for company: {}", company_id);
        let mut conn = self.pool.get().expect("Failed to get DB connection");
        emaillog::table
            .filter(emaillog::company_id.eq(company_id))
            .order(emaillog::created_at.desc())
            .limit(100)
            .load::<EmailLog>(&mut conn)
    }

    fn get_email_log_stats(
        &self,
        company_id: i64,
    ) -> Result<(i64, i64, i64, i64), diesel::result::Error> {
        log::debug!("Fetching email log stats for company: {}", company_id);
        let mut conn = self.pool.get().expect("Failed to get DB connection");

        let total: i64 = emaillog::table
            .filter(emaillog::company_id.eq(company_id))
            .count()
            .get_result(&mut conn)?;

        let sent: i64 = emaillog::table
            .filter(emaillog::company_id.eq(company_id))
            .filter(emaillog::status.eq(Some("Success".to_string())))
            .count()
            .get_result(&mut conn)?;

        let queued: i64 = emaillog::table
            .filter(emaillog::company_id.eq(company_id))
            .filter(emaillog::status.eq(Some("Queued".to_string())))
            .count()
            .get_result(&mut conn)?;

        let failed: i64 = emaillog::table
            .filter(emaillog::company_id.eq(company_id))
            .filter(emaillog::status.eq(Some("Failed".to_string())))
            .count()
            .get_result(&mut conn)?;

        Ok((total, sent, queued, failed))
    }

    fn create_template(&self, new_template: NewTemplate) -> Result<Template, diesel::result::Error> {
        log::debug!("Creating template: {} for company: {}", new_template.name, new_template.company_id);
        let mut conn = self.pool.get().expect("Failed to get DB connection");
        diesel::insert_into(templates::table)
            .values(&new_template)
            .get_result::<Template>(&mut conn)
    }

    fn get_templates_by_company(&self, company_id: i64) -> Result<Vec<Template>, diesel::result::Error> {
        log::debug!("Fetching templates for company: {}", company_id);
        let mut conn = self.pool.get().expect("Failed to get DB connection");
        templates::table
            .filter(templates::company_id.eq(company_id))
            .order(templates::date_created.desc())
            .load::<Template>(&mut conn)
    }

    fn get_template_by_id(&self, template_id: i64, company_id: i64) -> Result<Template, diesel::result::Error> {
        log::debug!("Fetching template by ID: {} for company: {}", template_id, company_id);
        let mut conn = self.pool.get().expect("Failed to get DB connection");
        templates::table
            .filter(templates::id.eq(template_id))
            .filter(templates::company_id.eq(company_id))
            .first::<Template>(&mut conn)
    }

    fn update_template(&self, template_id: i64, template: &Template) -> Result<Template, diesel::result::Error> {
        log::debug!("Updating template: {}", template_id);
        let mut conn = self.pool.get().expect("Failed to get DB connection");
        diesel::update(templates::table.find(template_id))
            .set((
                templates::name.eq(&template.name),
                templates::subject.eq(&template.subject),
                templates::content.eq(&template.content),
                templates::template_type.eq(&template.template_type),
                templates::date_updated.eq(chrono::Utc::now()),
            ))
            .get_result::<Template>(&mut conn)
    }

    fn delete_template(&self, template_id: i64, company_id: i64) -> Result<usize, diesel::result::Error> {
        log::debug!("Deleting template: {} for company: {}", template_id, company_id);
        let mut conn = self.pool.get().expect("Failed to get DB connection");
        diesel::delete(
            templates::table
                .filter(templates::id.eq(template_id))
                .filter(templates::company_id.eq(company_id))
        ).execute(&mut conn)
    }

    fn update_company(&self, company_id: i64, company: &Company) -> Result<Company, diesel::result::Error> {
        log::debug!("Updating company: {}", company_id);
        let mut conn = self.pool.get().expect("Failed to get DB connection");
        diesel::update(companies::table.find(company_id))
            .set((
                companies::company_name.eq(&company.company_name),
                companies::website.eq(&company.website),
                companies::company_address.eq(&company.company_address),
                companies::default_from_email.eq(&company.default_from_email),
                companies::default_from_name.eq(&company.default_from_name),
            ))
            .get_result::<Company>(&mut conn)
    }
}
