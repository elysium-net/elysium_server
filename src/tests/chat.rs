use crate::services::chat;
use crate::tests;
use elysium_rust::chat::v1::chat_service_server::ChatService;
use elysium_rust::chat::v1::{
    Channel, Content, CreateChannelRequest, SendMessageRequest, content, send_message_response,
};
use elysium_rust::user::v1::CreateUserRequest;
use tonic::Request;

#[tokio::test]
async fn chat() {
    let (state, token) = tests::init().await;
    let service = chat::Service::new(state);

    tracing::info!("Creating channel...");
    service
        .create_channel(tests::request(
            CreateChannelRequest {
                channel: Some(Channel {
                    channel_id: "to:admin".to_string(),
                    name: "Chat with admin".to_string(),
                    description: "A private chat with admin.".to_string(),
                    members: vec!["admin".to_string()],
                }),
            },
            [("Authorization", token.as_str())],
        ))
        .await
        .unwrap()
        .into_inner()
        .error
        .map(|err| panic!("{err:?}"));

    tracing::info!("Sending message...");
    match service
        .send_message(tests::request(
            SendMessageRequest {
                channel_id: "to:admin".to_string(),
                content: Some(Content {
                    content: Some(content::Content::Text("Hello!".to_string())),
                }),
            },
            [("Authorization", token.as_str())],
        ))
        .await
        .unwrap()
        .into_inner()
        .result
        .unwrap()
    {
        send_message_response::Result::Message(msg) => {}
        send_message_response::Result::Error(err) => panic!("{err:?}"),
    }
}
