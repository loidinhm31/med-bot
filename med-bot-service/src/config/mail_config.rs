use dotenv::dotenv;
use std::env;
use lettre::transport::smtp::authentication::Credentials;

#[derive(Debug, Clone)]
pub struct MailClient {
    pub smtp_host: String,
    pub smtp_username: String,
    pub smtp_password: String,
    pub credentials: Credentials,
    pub from_email: String,
    pub target_email: String,
}

impl MailClient {
    pub fn builder() -> MailClientBuilder {
        MailClientBuilder::new()
    }
}

pub struct MailClientBuilder {
    pub smtp_host: String,
    pub smtp_username: String,
    pub smtp_password: String,
    pub credentials: Credentials,
    pub from_email: String,
    pub target_email: String,
}

impl MailClientBuilder {
    pub fn new() -> MailClientBuilder {
        dotenv().ok();
        let smtp_host = match env::var("SMTP_HOST") {
            Ok(v) => v.to_string(),
            Err(_) => {
                eprintln!("Error loading SMTP HOST from env");
                "UNKNOWN".to_string()
            }
        };

        let smtp_username = match env::var("SMTP_USERNAME") {
            Ok(v) => v.to_string(),
            Err(_) => {
                eprintln!("Error loading SMTP_USERNAME from env");
                "UNKNOWN".to_string()
            }
        };

        let smtp_password = match env::var("SMTP_PASSWORD") {
            Ok(v) => v.to_string(),
            Err(_) => {
                eprintln!("Error loading SMTP_PASSWORD from env");
                "UNKNOWN".to_string()
            }
        };

        let from_email = match env::var("FROM_EMAIL") {
            Ok(v) => v.to_string(),
            Err(_) => {
                eprintln!("Error loading FROM_EMAIL from env");
                "UNKNOWN".to_string()
            }
        };

        let target_email = match env::var("TARGET_EMAIL") {
            Ok(v) => v.to_string(),
            Err(_) => {
                eprintln!("Error loading TARGET_EMAIL from env");
                "UNKNOWN".to_string()
            }
        };

        // Create SMTP client credentials using username and password
        let creds = Credentials::new(
            smtp_username.clone().to_string(),
            smtp_password.clone().to_string()
        );

        MailClientBuilder {
            smtp_host,
            smtp_username,
            smtp_password,
            credentials: creds,
            from_email,
            target_email,
        }
    }


    pub fn build(self) -> MailClient {
        MailClient {
            smtp_host: self.smtp_host,
            smtp_username: self.smtp_username,
            smtp_password: self.smtp_password,
            credentials: self.credentials,
            from_email: self.from_email,
            target_email: self.target_email,
        }
    }
}