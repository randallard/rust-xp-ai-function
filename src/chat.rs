use crate::Result;
use async_openai::types::{
    ChatCompletionRequestMessage, ChatCompletionRequestUserMessageArgs,
};


pub fn user_msg(content: impl Into<String>) -> Result<ChatCompletionRequestMessage> {
    let msg = ChatCompletionRequestUserMessageArgs::default()
        .content(content.into())
        .build()?;
    Ok(msg.into())
}