use crate::services::chat;
use crate::tests;
use elysium_rust::chat::v1::chat_service_server::ChatService;
use elysium_rust::chat::v1::{
    ChannelPermission, Content, CreateChannelRequest, SendMessageRequest, content,
    create_channel_response, send_message_response,
};
use elysium_rust::common::v1::Timestamp;
use std::collections::HashMap;

#[tokio::test]
async fn chat() {
    let (state, token) = tests::init().await;
    let service = chat::Service::new(state);

    let channel_args = CreateChannelRequest {
        name: "Chat with admin".to_string(),
        description: "A private chat with admin".to_string(),
        members: HashMap::from_iter([("admin".to_string(), ChannelPermission::Manager as i32)]),
    };

    tracing::info!("Creating channel...");
    let channel = match service
        .create_channel(tests::request(
            channel_args.clone(),
            [("Authorization", token.as_str())],
        ))
        .await
        .unwrap()
        .into_inner()
        .result
        .unwrap()
    {
        create_channel_response::Result::Channel(channel) => {
            assert_eq!(channel.members, channel_args.members);
            assert_eq!(channel.name, channel_args.name);
            assert_eq!(channel.description, channel_args.description);

            channel
        }
        create_channel_response::Result::Error(err) => panic!("{err:?}"),
    };

    tracing::info!("Sending message...");
    match service
        .send_message(tests::request(
            SendMessageRequest {
                channel_id: channel.channel_id.clone(),
                content: Some(Content {
                    // Gets fixed internally
                    created_at: Some(Timestamp { millis: 0 }),
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
        send_message_response::Result::Message(msg) => {
            assert_eq!(msg.channel_id, channel.channel_id);
            assert_eq!(
                msg.content.unwrap().content.unwrap(),
                content::Content::Text("Hello!".to_string())
            );
        }
        send_message_response::Result::Error(err) => panic!("{err:?}"),
    }
}
