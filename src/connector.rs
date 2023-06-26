use crate::console_log;
use async_trait::async_trait;
use jid::JidParseError;
use minidom::Element;
use prose_core_lib::{
    Connection, ConnectionConfiguration, ConnectionError, ConnectionEvent, ConnectionHandler,
    Connector, StanzaHandler, TimedHandler,
};
use std::str::FromStr;
use std::sync::Arc;
use wasm_bindgen::prelude::*;

// The JavaScript XMPPClient which wraps StropheJS
#[wasm_bindgen(module = "/web/xmpp_client.js")]
extern "C" {
    type JSXMPPClient;

    #[wasm_bindgen(constructor)]
    fn new() -> JSXMPPClient;

    #[wasm_bindgen(method)]
    fn set_handlers(this: &JSXMPPClient, handlers: RustHandlers);

    #[wasm_bindgen(method)]
    async fn connect(this: &JSXMPPClient, jid: String, password: String);

    #[wasm_bindgen(method)]
    fn disconnect(this: &JSXMPPClient);

    #[wasm_bindgen(method)]
    fn send_stanza(this: &JSXMPPClient, stanza: String);
}

unsafe impl Send for JSXMPPClient {}
unsafe impl Sync for JSXMPPClient {}

#[wasm_bindgen]
pub struct RustHandlers {
    connection: Arc<Box<dyn Connection>>,
    connection_handler: ConnectionHandler,
    stanza_handler: StanzaHandler,
    timeout_handler: TimedHandler,
    ping_handler: TimedHandler,
}

impl RustHandlers {
    pub fn new(connection: Arc<Box<dyn Connection>>, config: ConnectionConfiguration) -> Self {
        RustHandlers {
            connection,
            connection_handler: config.connection_handler,
            stanza_handler: config.stanza_handler,
            timeout_handler: config.timeout_handler,
            ping_handler: config.ping_handler,
        }
    }
}

#[wasm_bindgen]
impl RustHandlers {
    pub fn handle_connect(&mut self) {
        (self.connection_handler)(self.connection.clone(), &ConnectionEvent::Connect)
    }

    pub fn handle_disconnect(&mut self, error: String) {
        (self.connection_handler)(
            self.connection.clone(),
            &ConnectionEvent::Disconnect {
                error: ConnectionError::Generic { msg: error },
            },
        )
    }

    pub fn handle_timeout(&mut self) {
        (self.timeout_handler)(self.connection.clone());
    }

    pub fn handle_ping_timeout(&mut self) {
        (self.ping_handler)(self.connection.clone());
    }

    pub fn handle_stanza(&mut self, stanza: String) {
        console_log!("Received stanza {}", stanza);
        (self.stanza_handler)(
            self.connection.clone(),
            Element::from_str(&stanza).expect("Failed to parse received stanza"),
        );
    }
}

pub struct StropheJSConnector {}

impl Default for StropheJSConnector {
    fn default() -> Self {
        StropheJSConnector {}
    }
}

#[async_trait(?Send)]
impl Connector for StropheJSConnector {
    async fn connect(
        &self,
        config: ConnectionConfiguration,
    ) -> Result<Box<dyn Connection>, ConnectionError> {
        let client = Arc::new(JSXMPPClient::new());
        let (jid, password) = (config.jid.to_string(), config.password.clone());

        client.set_handlers(RustHandlers::new(
            Arc::new(Box::new(StropheJSConnection {
                client: client.clone(),
            })),
            config,
        ));

        client.connect(jid, password).await;

        console_log!("Connected!");

        Ok(Box::new(StropheJSConnection { client }))
    }
}

struct StropheJSConnection {
    client: Arc<JSXMPPClient>,
}

impl Connection for StropheJSConnection {
    fn disconnect(&self) {
        self.client.disconnect()
    }

    fn send_stanza(&self, stanza: Element) -> anyhow::Result<()> {
        self.client.send_stanza(String::from(&stanza));
        Ok(())
    }
}

pub struct JSConnectionError(ConnectionError);

impl From<ConnectionError> for JSConnectionError {
    fn from(value: ConnectionError) -> Self {
        JSConnectionError(value)
    }
}

impl From<JidParseError> for JSConnectionError {
    fn from(_value: JidParseError) -> Self {
        JSConnectionError(ConnectionError::Generic {
            msg: "Failed to parse JID".to_string(),
        })
    }
}

impl From<JSConnectionError> for JsValue {
    fn from(value: JSConnectionError) -> Self {
        match value.0 {
            ConnectionError::TimedOut => "Connection timed out".into(),
            ConnectionError::InvalidCredentials => "Invalid credentials".into(),
            ConnectionError::Generic { msg } => msg.into(),
        }
    }
}
