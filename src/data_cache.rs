use std::collections::HashMap;
use jid::BareJid;
use prose_core_client::types::roster::Item;
use prose_core_client::types::{AccountSettings, AvatarMetadata, MessageLike, Page};
use prose_core_client::{ContactsCache, DataCache, MessageCache};
use prose_core_domain::{Contact, UserProfile};
use prose_core_lib::stanza::avatar::ImageId;
use prose_core_lib::stanza::message::ChatState;
use prose_core_lib::stanza::{message, presence};
use std::sync::Mutex;

pub struct InMemoryDataCache {
    messages: Mutex<HashMap<message::Id, MessageLike>>,
}

impl Default for InMemoryDataCache {
    fn default() -> Self {
        InMemoryDataCache {
            messages: Mutex::new(HashMap::new()),
        }
    }
}

impl ContactsCache for InMemoryDataCache {
    fn has_valid_roster_items(&self) -> anyhow::Result<bool> {
        Ok(false)
    }

    fn insert_roster_items(&self, _items: &[Item]) -> anyhow::Result<()> {
        Ok(())
    }

    fn insert_user_profile(&self, _jid: &BareJid, _profile: &UserProfile) -> anyhow::Result<()> {
        Ok(())
    }

    fn load_user_profile(&self, _jid: &BareJid) -> anyhow::Result<Option<UserProfile>> {
        Ok(None)
    }

    fn delete_user_profile(&self, _jid: &BareJid) -> anyhow::Result<()> {
        Ok(())
    }

    fn insert_avatar_metadata(
        &self,
        _jid: &BareJid,
        _metadata: &AvatarMetadata,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn load_avatar_metadata(&self, _jid: &BareJid) -> anyhow::Result<Option<AvatarMetadata>> {
        Ok(None)
    }

    fn insert_presence(
        &self,
        _jid: &BareJid,
        _kind: Option<presence::Type>,
        _show: Option<presence::Show>,
        _status: Option<String>,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn insert_chat_state(&self, _jid: &BareJid, _chat_state: &ChatState) -> anyhow::Result<()> {
        Ok(())
    }

    fn load_chat_state(&self, _jid: &BareJid) -> anyhow::Result<Option<ChatState>> {
        Ok(None)
    }

    fn load_contacts(&self) -> anyhow::Result<Vec<(Contact, Option<ImageId>)>> {
        Ok(vec![])
    }
}

impl MessageCache for InMemoryDataCache {
    fn insert_messages<'a>(
        &self,
        messages: impl IntoIterator<Item = &'a MessageLike>,
    ) -> anyhow::Result<()> {
        let mut cached_messages = self.messages.lock().unwrap();

        for message in messages.into_iter() {
            cached_messages.insert(message.id.clone(), message.clone());
        }
        Ok(())
    }

    fn load_messages_targeting<'a>(
        &self,
        _conversation: &BareJid,
        targets: &[message::Id],
        _newer_than: impl Into<Option<&'a message::Id>>,
        _include_targeted_messages: bool,
    ) -> anyhow::Result<Vec<MessageLike>> {
        let all_messages = self
            .messages
            .lock()
            .unwrap();

        let mut messages = vec![];
        for id in targets {
            if let Some(message) = all_messages.get(id) {
                messages.push(message.clone())
            }
        }

        Ok(messages)
    }

    fn load_messages_before(
        &self,
        _conversation: &BareJid,
        _older_than: Option<&message::Id>,
        _max_count: u32,
    ) -> anyhow::Result<Option<Page<MessageLike>>> {
        Ok(None)
    }

    fn load_messages_after(
        &self,
        _conversation: &BareJid,
        _newer_than: &message::Id,
        _max_count: Option<u32>,
    ) -> anyhow::Result<Vec<MessageLike>> {
        Ok(vec![])
    }

    fn load_stanza_id(
        &self,
        _conversation: &BareJid,
        _message_id: &message::Id,
    ) -> anyhow::Result<Option<prose_core_lib::stanza::message::stanza_id::Id>> {
        Ok(None)
    }

    fn save_draft(&self, _conversation: &BareJid, _text: Option<&str>) -> anyhow::Result<()> {
        Ok(())
    }

    fn load_draft(&self, _conversation: &BareJid) -> anyhow::Result<Option<String>> {
        Ok(None)
    }
}

impl DataCache for InMemoryDataCache {
    fn delete_all(&self) -> anyhow::Result<()> {
        Ok(())
    }

    fn save_account_settings(&self, _settings: &AccountSettings) -> anyhow::Result<()> {
        Ok(())
    }

    fn load_account_settings(&self) -> anyhow::Result<Option<AccountSettings>> {
        Ok(None)
    }
}
