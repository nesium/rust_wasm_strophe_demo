use crate::connector::{JSConnectionError, StropheJSConnector};
use crate::console_log;
use crate::data_cache::InMemoryDataCache;
use crate::delegate::{Delegate, JSClientDelegate};
use jid::{BareJid, FullJid, Jid};
use prose_core_client::{Client, ClientBuilder, NoopAvatarCache};
use prose_core_domain::{Availability, MessageId};
use std::str::FromStr;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

// The Rust XMPPClient which interacts with the server through JSXMPPClient
#[wasm_bindgen]
pub struct RustXMPPClient {
    client: Client<InMemoryDataCache, NoopAvatarCache>,
}

#[wasm_bindgen]
impl RustXMPPClient {
    pub async fn init(delegate: JSClientDelegate) -> anyhow::Result<RustXMPPClient, JsValue> {
        let db = InMemoryDataCache::new().await.map_err(|err| JsValue::from(err.to_string()))?;

        let client = RustXMPPClient {
            client: ClientBuilder::<InMemoryDataCache, NoopAvatarCache>::new()
                .set_connector_provider(Box::new(|| Box::new(StropheJSConnector::default())))
                .set_data_cache(db)
                .set_delegate(Some(Box::new(Delegate::new(delegate))))
                .build(),
        };

        Ok(client)
    }

    pub async fn connect(&self, jid: String, password: String) -> Result<(), JSConnectionError> {
        console_log!("Connect {} - {}", jid, password);

        let jid =
            FullJid::from_str(&format!("{}/wasm", jid)).map_err(Into::<JSConnectionError>::into)?;

        self.client
            .connect(&jid, password, Availability::Available, None)
            .await
            .map_err(Into::<JSConnectionError>::into)?;

        Ok(())
    }

    pub async fn send_message(&self, to: String, body: String) -> Result<(), JsValue> {
        console_log!("Sending message to {}…", to);

        let jid = Jid::from_str(&to).map_err(|err| JsValue::from(err.to_string()))?;

        self.client
            .send_message(jid, body)
            .await
            .map_err(|err| JsValue::from(err.to_string()))?;
        Ok(())
    }

    pub async fn load_messages_with_ids(
        &self,
        conversation: String,
        ids: Vec<JsValue>,
    ) -> Result<JsValue, JsValue> {
        console_log!("Loading messages in conversation {}…", conversation);

        let message_ids: Vec<MessageId> = ids
            .into_iter()
            .map(|v| MessageId(v.as_string().unwrap()))
            .collect();

        let messages = self
            .client
            .load_messages_with_ids(&BareJid::from_str(&conversation).unwrap(), &message_ids)
            .await
            .map_err(|err| JsValue::from(err.to_string()))?;

        console_log!("Found {} messages.", messages.len());

        Ok(serde_wasm_bindgen::to_value(&messages).unwrap())
    }
}
