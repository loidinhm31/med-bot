use crate::config::mail_config::MailClient;
use crate::models::doctor_appointment::AppointmentPicking;
use lettre::message::{Mailbox, MultiPart, SinglePart};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

#[derive(Debug, Clone)]
pub struct MailService {
    mail_client: MailClient,
}

impl MailService {
    pub fn builder(mail_client: MailClient) -> MailServiceBuilder {
        MailServiceBuilder::new(mail_client)
    }

    pub fn send_email(&self, appointment: &AppointmentPicking) -> Result<(), &str> {
        let doctor_name = appointment.doctor_name.clone().unwrap_or("Unknown".to_string());

        let slots = appointment.available_slot.as_ref().map(|slots| {
            slots.iter().map(|slot| {
                format!(
                    "<tr><td>{}</td><td>{}</td><td>{}</td><td>{}</td></tr>",
                    slot.start_time.clone(),
                    slot.end_time.clone(),
                    slot.max_slot.unwrap_or(0),
                    slot.available_slot.unwrap_or(0)
                )
            }).collect::<Vec<String>>().join("")
        }).unwrap_or_else(|| "<tr><td colspan='3'>No available slots</td></tr>".to_string());


        let html_content = format!(
            r#"
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <style>
                body {{
                    font-family: Arial, sans-serif;
                    background-color: #f4f4f4;
                    margin: 0;
                    padding: 20px;
                }}
                .container {{
                    background-color: #ffffff;
                    padding: 20px;
                    border-radius: 8px;
                    box-shadow: 0 0 10px rgba(0, 0, 0, 0.1);
                    max-width: 600px;
                    margin: 0 auto;
                }}
                h2 {{
                    color: #4CAF50;
                    text-align: center;
                }}
                .doctor-info {{
                    margin-bottom: 20px;
                }}
                .doctor-info p {{
                    font-size: 16px;
                    color: #333;
                }}
                table {{
                    width: 100%;
                    border-collapse: collapse;
                    margin-bottom: 20px;
                }}
                table, th, td {{
                    border: 1px solid #ddd;
                }}
                th, td {{
                    padding: 12px;
                    text-align: center;
                }}
                th {{
                    background-color: #4CAF50;
                    color: white;
                }}
                td {{
                    color: #333;
                }}
                .footer {{
                    text-align: center;
                    margin-top: 20px;
                    color: #888;
                    font-size: 12px;
                }}
            </style>
        </head>
        <body>

        <div class="container">
            <h2>Appointment Notification</h2>

            <div class="doctor-info">
                <p><strong>Doctor Name:</strong> {doctor_name}</p>
                <p><strong>Appointment Date:</strong> {appointment_date}</p>
                <p><strong>Appointment Day:</strong> {appointment_day}</p>
            </div>

            <table>
                <thead>
                    <tr>
                        <th>Start Time</th>
                        <th>End Time</th>
                        <th>Max Slots</th>
                        <th>Available Slots</th>
                    </tr>
                </thead>
                <tbody>
                    {slots}
                </tbody>
            </table>

            <p>Please contact us if you need further assistance.</p>

            <div class="footer">
                <p>&copy; 2024 Medical Bot, All Rights Reserved</p>
            </div>
        </div>

        </body>
        </html>
        "#,
            doctor_name = doctor_name,
            appointment_day = appointment.appointment_day.clone().unwrap_or(String::new()),
            appointment_date = appointment.appointment_date.clone().unwrap(),
            slots = slots
        );

        log::info!("Sending email from: {}", self.mail_client.from_email.clone());

        let from_email = format!(r#"MED bot <{}>"#,
                                 self.mail_client.from_email.to_string())
            .parse::<Mailbox>().unwrap_or_else(|err| {
            panic!("Failed to parse email into Mailbox: {:?}", err);
        });

        let mut email_builder = Message::builder()
            .from(from_email)
            .subject("Appointment Event");

        let target_emails = self.mail_client.target_email
            .split(";")
            .collect::<Vec<&str>>();

        for recipient in target_emails {
            email_builder = email_builder.to(recipient.parse::<Mailbox>().unwrap());
        }

        let email_builder_content = email_builder.multipart(
            MultiPart::alternative().singlepart(SinglePart::html(html_content.to_string())),
        ).unwrap();


        // Open a secure connection to the SMTP server using STARTTLS
        let mailer = SmtpTransport::starttls_relay(self.mail_client.smtp_host.clone().as_str())
            .unwrap()  // Unwrap the Result, panics in case of error
            .credentials(self.mail_client.credentials.clone())
            .build();

        // Attempt to send the email via the SMTP transport
        match mailer.send(&email_builder_content) {
            // If email was sent successfully, print confirmation message
            Ok(_) => {
                log::info!("Email sent successfully!");
                Ok(())
            }
            // If there was an error sending the email, print the error
            Err(e) => {
                log::error!("Could not send email: {:?}", e);
                Err("Could not send email")
            }
        }
    }
}

pub struct MailServiceBuilder {
    mail_client: MailClient,
}

impl MailServiceBuilder {
    pub fn new(mail_client: MailClient) -> MailServiceBuilder {
        MailServiceBuilder { mail_client }
    }

    pub fn build(self) -> MailService {
        MailService {
            mail_client: self.mail_client
        }
    }
}
