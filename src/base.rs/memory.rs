use serenity::{http::{Http, AttachmentType::{Bytes, self}}, model::{id::MessageId, channel::{Message, Channel::*, MessageReference}}};
use aes_gcm::{Aes256Gcm, Key, Nonce, aead::{Aead, NewAead}};
use std::{sync::Arc, iter::{FromIterator, Iterator, repeat, }, borrow::Cow,};
use crate::base::{secret::{KEY, USER_ID}, service::Service, command::get_service, error::MemoryError::{self, *}};
use serde::{Serialize, de::DeserializeOwned};
use bincode;

pub type Data = Vec<u8>;

pub fn to_struct<T>(data : Data) -> T where T: DeserializeOwned {
    bincode::deserialize(&data).unwrap()
}

pub fn from_struct<T>(instance : T) -> Data where T : Serialize {
    bincode::serialize(&instance).expect("Error serialising.")
}

fn encrypt(data : Data, msg_id : MessageId) -> Data {
    let cipher = Aes256Gcm::new(Key::from_slice(KEY));
    let nonce = Nonce::from_iter(msg_id.0.to_be_bytes().iter().copied().chain(repeat(0u8).take(4)));

    let encrypted = cipher.encrypt(&nonce, data.as_ref()).expect("Decryption did not work.");
    
    encrypted
}

fn create_attachment(data : Data, msg_id : MessageId, identifier : &str) -> AttachmentType {
    let encrypted = encrypt(data, msg_id);

    let attachment = Bytes { data:  Cow::Owned(encrypted), filename: identifier.to_string() };
    
    attachment
}

pub fn attachment_from_struct<T>(instance : T, msg_id : MessageId, identifier : &str) -> AttachmentType where T : Serialize {
    create_attachment(from_struct(instance), msg_id, identifier) 
}

fn decrypt(data : Data, data_parent_msg_id : MessageId) -> Result<Data, MemoryError> {
    let cipher = Aes256Gcm::new(Key::from_slice(KEY));
    let nonce = Nonce::from_iter(data_parent_msg_id.0.to_be_bytes().iter().copied().chain(repeat(0u8).take(4))); //Check that this works...

    let decrypted = cipher.decrypt(&nonce, data.as_ref()).or(Err(FailDecrypt));

    decrypted
} 

pub async fn get_data(http : &Arc<Http>, msg_ref : &MessageReference) -> Result<(Service, Data), MemoryError> {
    let data_msg = get_ref_msg(http, msg_ref).await?;

    if data_msg.author.id != USER_ID {
        return Err(NotBotData);
    }

    let data_msg_parent_id = if let Some(MessageReference { message_id : Some(val), .. })  = data_msg.message_reference {
        val
    } 
    else {
        return Err(NotBotData);
    };
    
    let (service, encrypted) = download_data(&data_msg).await?;
    
    let decrypted = decrypt(encrypted, data_msg_parent_id)?;//Look into these

    Ok((service, decrypted))
}

async fn get_ref_msg(http : &Arc<Http>, msg_ref : &MessageReference) -> Result<Message, MemoryError> {
    let (data_msg_id, data_channel_id) = if let MessageReference{message_id : Some(data_msg_id), channel_id : data_channel_id, ..} = msg_ref {
        (data_msg_id, data_channel_id)
    } 
    else {
        unreachable!()
    };

    let data_msg = match data_channel_id.to_channel(&http).await.or(Err(NotChannel))? {
        Guild(guild_channel) => guild_channel.message(&http, data_msg_id).await,
        Private(private_channel) => private_channel.message(&http, data_msg_id).await,
        _ => unreachable!(),
    }.or(Err(NotMessage))?;


    return Ok(data_msg);
}

async fn download_data(msg : &Message) -> Result<(Service, Data), MemoryError> {
    for attachment in &msg.attachments {
        if let Some(service) = get_service(&attachment.filename) {
            let data = attachment.download().await.or(Err(FailDownload))?;

            return Ok((service, data));
        }
    }
    Err(NotService)
}

