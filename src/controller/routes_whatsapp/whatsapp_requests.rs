use whatsapp_cloud_api::{
    models::{Component, Message, Parameter, Template},
    WhatasppClient,
};

use crate::core::enviroments::Enviroment;

use super::whatsapp_model::whatsapp_payload::WhatsappPaylaod;

pub async fn send_whatsapp_message(payload: WhatsappPaylaod) -> bool {
    let access_token = Enviroment::whatsapp_api();
    let phone_number_id = Enviroment::whatsapp_phone_number_id();
    let to = Enviroment::whatsapp_personal_phone_number();
    let template_name = "message_request";
    let language = "en_US";
    let header_parameters = Vec::from([Parameter::from_text(&payload.from)]);
    let body_parameters = Vec::from([
        Parameter::from_text(&payload.from),
        Parameter::from_text(&payload.cellphone),
        Parameter::from_text(&payload.email),
    ]);
    let components = Vec::from([
        Component::with_parameters("header", header_parameters),
        Component::with_parameters("body", body_parameters),
    ]);
    let template = Template::with_components(template_name, language, components);
    let message = Message::from_template(&to, template);
    let client = WhatasppClient::new(&access_token, &phone_number_id);
    let response = client.send_message(&message).await;
    print!("{:?}", response);
    response.is_err()
}
