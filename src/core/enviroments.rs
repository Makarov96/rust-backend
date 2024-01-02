use std::env;
pub struct Enviroment {}

impl Enviroment {
    pub fn init() {
        dotenvy::dotenv().unwrap();
    }

    pub fn whatsapp_phone_number_id() -> String {
        let result = env::var("WHATSAPP_PHONE_NUMBER_ID")
            .ok()
            .unwrap_or_default();

        result
    }

    pub fn whatsapp_api() -> String {
        let result = env::var("WHATSAPP_API").ok().unwrap_or_default();

        result
    }

    pub fn whatsapp_personal_phone_number() -> String {
        let result = env::var("WHATSAPP_PERSONAL_PHONE_NUMBER")
            .ok()
            .unwrap_or_default();

        result
    }

    pub fn database_url() -> String {
        let result = env::var("DATABASE_URL").ok().unwrap_or_default();

        result
    }
}
