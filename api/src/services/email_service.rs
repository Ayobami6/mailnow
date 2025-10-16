use lettre::{
    message::header::ContentType, transport::smtp::authentication::Credentials, AsyncSmtpTransport,
    AsyncTransport, Message, Tokio1Executor,
};

pub struct EmailService;

impl EmailService {
    // email service constructor
    pub fn new() -> Self {
        EmailService
    }

    pub fn create_mailer(
        smtp_server: &str,
        smtp_username: &str,
        smtp_password: &str,
    ) -> Result<AsyncSmtpTransport<Tokio1Executor>, Box<dyn std::error::Error + Send + Sync>> {
        let creds = Credentials::new(smtp_username.to_string(), smtp_password.to_string());
        let mailer = AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(smtp_server)?
            .credentials(creds)
            .port(587)
            .build();
        Ok(mailer)
    }

    pub async fn send_email(
        &self,
        smtp_server: &str,
        smtp_username: &str,
        smtp_password: &str,
        from: &str,
        to: &str,
        subject: &str,
        content: &str,
        is_html: bool,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mailer = Self::create_mailer(smtp_server, smtp_username, smtp_password)?;

        let content_type = if is_html {
            ContentType::TEXT_HTML
        } else {
            ContentType::TEXT_PLAIN
        };

        let email = Message::builder()
            .from(smtp_username.parse()?)
            .to(to.parse()?)
            .subject(subject)
            .header(content_type)
            .body(content.to_string())?;

        let err = mailer.send(email).await;
        if let Err(e) = err {
            println!("Failed to send email: {}", e);
            return Err(Box::new(e));
        }
        Ok(())
    }
}
