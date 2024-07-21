use lettre::{
    message::header::ContentType, transport::smtp::authentication::Credentials, Message,
    SmtpTransport, Transport,
};

pub struct Email {
    pub to: String,
    pub subject: String,
    pub body: String,
    pub host: String,
    pub sender: String,
    pub password: String,
}

impl Into<Email> for crate::config::EmailConfig {
    fn into(self) -> Email {
        Email {
            to: self.to,
            subject: self.subject,
            body: self.body,
            host: self.host,
            sender: self.sender,
            password: self.password,
        }
    }
}

impl Email {
    pub fn send_email(self) {
        let email = Message::builder()
            .from(self.sender.parse().unwrap())
            .to(self.to.parse().unwrap())
            .subject(&self.subject)
            .body(self.body.to_string())
            .unwrap();

        let creds = Credentials::new(self.sender, self.password);
        let mailer = SmtpTransport::relay(&self.host)
            .unwrap()
            .credentials(creds)
            .build();

        // Send the email
        match mailer.send(&email) {
            Ok(_) => println!("Email sent successfully!"),
            Err(e) => panic!("Could not send email: {e:?}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Email;
    use lettre::message::header::{Header, Subject};
    use lettre::message::MultiPart;
    use lettre::Message;

}