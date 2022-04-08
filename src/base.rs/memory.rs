use serenity::{http::{Http, AttachmentType::{Bytes, self}}, model::{id::MessageId, channel::{Message, Channel, MessageReference, self, Attachment}}};
use aes_gcm::{Aes256Gcm, Key, Nonce, aead::{Aead, NewAead}};
use std::{sync::Arc, iter::{FromIterator, Iterator, repeat, }, borrow::Cow,};
use crate::base::{secret::{KEY, USER_ID}};
use serde::{Serialize, Deserialize, de::DeserializeOwned};
use bincode;

pub type Data = Vec<u8>;

pub fn to_struct<T>(data : Data) -> T where T: DeserializeOwned {
    bincode::deserialize(&data).unwrap()
}

pub fn from_struct<T>(instance : T) -> Data where T : Serialize {
    bincode::serialize(&instance).expect("Error serialising.")
}

pub async fn parent_data(http : &Arc<Http>, msg_ref : &Option<MessageReference>, identifier : &str) -> Option<Data> {
    let data_msg = get_ref_msg(http, &msg_ref).await?;
    if data_msg.author.id != USER_ID {
        return None;
    }
    
    let encrypted = download_data(&data_msg, identifier).await?;
    
    let decrypted = decrypt(encrypted, data_msg.message_reference?.message_id?);

    Some(decrypted)
}

async fn get_ref_msg(http : &Arc<Http>, msg_ref : &Option<MessageReference>) -> Option<Message>{
    let (data_channel_id, data_msg_id) = match msg_ref {
        Some(MessageReference{message_id : Some(msg_id), channel_id, ..}) => (channel_id, msg_id),
        _ => return None,
    };

    let data_channel = match data_channel_id.to_channel(&http).await {
        Ok(Channel::Guild((guild_channel))) => guild_channel,
        _ => unreachable!(),
    };

    let data_msg = data_channel.message(&http, data_msg_id).await.expect("Error getting reference message.");
    return Some(data_msg);
}

async fn download_data(msg : &Message, identifier : &str) -> Option<Data> {
    return Some(
    msg.attachments
    .iter().find(|attachment| attachment.filename.starts_with(identifier))?
    .download().await.expect("Error downloading data."));
}

fn decrypt(data : Data, data_parent_msg_id : MessageId) -> Data {
    let cipher = Aes256Gcm::new(Key::from_slice(KEY));
    let nonce = Nonce::from_iter(data_parent_msg_id.0.to_be_bytes().iter().map(|x| *x).chain(repeat(0u8).take(4))); //Check that this works...

    let decrypted = cipher.decrypt(&nonce, data.as_ref()).expect("Decryption did not work.");

    decrypted
} 


pub fn create_attachment(data : Data, msg_id : MessageId, identifier : &str) -> AttachmentType {
    let encrypted = encrypt(data, msg_id);

    let attachment = Bytes { data:  Cow::Owned(encrypted), filename: identifier.to_string() };
    
    attachment
}

fn encrypt(data : Data, msg_id : MessageId) -> Data {
    let cipher = Aes256Gcm::new(Key::from_slice(KEY));
    let nonce = Nonce::from_iter(msg_id.0.to_be_bytes().iter().map(|x| *x).chain(repeat(0u8).take(4)));

    let encrypted = cipher.encrypt(&nonce, data.as_ref()).expect("Decryption did not work.");
    
    encrypted
}