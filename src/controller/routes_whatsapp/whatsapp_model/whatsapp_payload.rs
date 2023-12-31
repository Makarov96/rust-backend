use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct WhatsappPaylaod {
    pub from: String,
    pub message: String,
    pub cellphone: String,
    pub email: String,
}
