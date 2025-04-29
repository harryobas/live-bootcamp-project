use super::Email;

#[async_trait::async_trait]
pub trait EmailClient: Send + Sync {
    async fn send_mail(
        &self,
        recipient: &Email,
        subject: &str,
        content: &str,
    ) -> Result<(), String>;
}

