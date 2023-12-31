use whatsapp_cloud_api::{
    models::{Component, Message, Parameter, Template},
    WhatasppClient,
};

use super::whatsapp_model::whatsapp_payload::WhatsappPaylaod;

pub async fn send_whatsapp_message(payload: WhatsappPaylaod) -> bool {
    let access_token = "EAAEB5VSLZBjYBO5E3nhSeM8W1V9S5D7bLbj57ngnf3ZAzKl1y8PfXEX3t0ro3QF2gfXVH5ONIvyD10Q7II1RJFAffVxugykwQGpPZCfxzN86RF2OwxWMqilvWQXpEQeCJKKpdtxuKWpqfbgBO4Gr3BuMh6gjyoWevwQlZAZBU7t0v6ztZCxY1QiV2ph3appESp3KyU4dXSWkjewzK7AxCiGNRq47SSCAOncTkZD";
    let phone_number_id = "194360767095944";
    let to = "+50254723883";
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

    response.is_err()
}
