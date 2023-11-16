pub mod attachment;
pub mod credentials;
pub mod form;
pub mod queries;

use async_graphql::Result;
use infer::Infer;
use lettre::{Message, SmtpTransport, Transport};
use lettre::message::{header::ContentType, Attachment, MultiPart, SinglePart};
use lettre::transport::smtp::authentication::Credentials;
use serde_json::Value;

use crate::Errors;
use crate::MailerAttachment;
use crate::MailerCredentials;

pub use form::{MailerForm, MailerError};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Mailer {
    pub credentials: MailerCredentials,
    pub template: Option<String>,
    pub context: Option<Value>,
    pub attachments: Option<Vec<MailerAttachment>>
}

impl From<MailerCredentials> for Mailer {
    fn from(credentials: MailerCredentials) -> Self {
        Self {
            credentials,
            ..Default::default()
        }
    }
}

impl Mailer {
    pub fn to<T: From<Self>>(&self) -> T {
        T::from(self.clone())
    }

    pub fn filter(&mut self) -> &mut Self {
        self.context = None;
        self.template = None;
        self.attachments = None;

        self
    }

    pub fn mutate(&mut self, form: &Self) -> &mut Self {
        self.credentials = form.credentials.clone();

        self
    }

    pub fn set_attachments(&mut self, attachments: Vec<MailerAttachment>) -> &mut Self {
        self.attachments = Some(attachments);

        self
    }

    pub fn set_context(&mut self, context: Value) -> &mut Self {
        self.context = Some(context);

        self
    }

    pub fn set_template<T>(&mut self, template: T) -> &mut Self
        where T: ToString
    {
        self.template = Some(template.to_string());

        self
    }

    pub fn send<F, T, S>(&mut self, from: F, to: T, subject: S) -> Result<String>
        where F: ToString,
              T: ToString,
              S: ToString
    {
        // Make sure values were properly decrypted
        let credentials = self.credentials
            .decrypt()
            .unwrap_or(self.credentials.clone());

        // Set bindings
        let from = from.to_string();
        let to = to.to_string();
        let subject = subject.to_string();

        // Set html email body
        let name = self.template.clone().unwrap_or_default();
        let data = self.context.clone().unwrap_or(Value::Null);
        let body = config::template::template()
            .render(&name, &data)
            .map_err(Errors::internal_server_error)?;

        let username = credentials.username.clone();
        let password = credentials.password.clone();
        let smtp_host = credentials.smtp_host.clone();

        // Create multipart body
        let mut multipart = MultiPart::alternative()
            .singlepart(
                SinglePart::builder()
                    .header(ContentType::TEXT_HTML)
                    .body(body)
            );

        // Check if attachment is available
        if let Some(attachments) = self.attachments.clone() {
            for attachment in attachments {
                let filename = attachment.filename.clone();
                let name = attachment.name.clone();

                if let Ok(file) = std::fs::read(&filename) {
                    let info = Infer::new()
                        .get(&file.clone())
                        .map_or(
                            String::default(),
                            |t| String::from(t.mime_type())
                        );

                    if let Ok(content_type) = ContentType::parse(&info) {
                        multipart = multipart.singlepart(
                            Attachment::new(name).body(file, content_type)
                        );
                    }
                }
            }
        }

        // Create email builder
        let builder = match Message::builder()
            .from(from.parse().unwrap())
            .to(to.parse().unwrap())
            .subject(subject)
            .multipart(multipart) {
            Ok(builder) => builder,
            Err(error) => return Err(Errors::bad_request(error))
        };

        // Set credentials
        let credentials = Credentials::new(username, password);

        // Set smtp transport relay
        let relay = match SmtpTransport::relay(smtp_host.as_str()) {
            Ok(relay) => relay,
            Err(error) => return Err(Errors::bad_request(error))
        };

        // Open a remote connection
        let mailer = relay.credentials(credentials).build();

        // Send the email
        if let Err(error) = mailer.send(&builder) {
            return Err(Errors::bad_request(error));
        }

        // Return success message
        Ok(format!("Email sent successfully to {to}"))
    }
}
