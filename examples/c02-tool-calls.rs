use async_openai::types::{
    CreateChatCompletionRequest, ChatCompletionToolChoiceOption
};
use xp_ai_function::{chat, gpts, oa_client::{self, new_oa_client}};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // -- Init AI Client
    let oa_client = new_oa_client()?;
    let chat_client = oa_client.chat();
    let model = gpts::MODEL.to_string();

    // -- User question
    let question = "What is the weather in California's best city and Paris? (be concise)";

    // -- Build messages
    let messages = vec![chat::user_msg(question)?];

    let tool_weather = chat::tool_fn(
        "get_weather",
        "get the weather for a city",
        json!({
            "type": "object",
            "properties": {
                "location": {
                    "type": "string",
                    "description": "The city and state, e.g. San Francisco, CA"
                },
                "country": {
                    "type": "string",
                    "description": "The full country name of the city"
                },
                "unit": {
                    "type": "string", "enum": ["celcius","fahrenheit"],
                    "descriotion": "Unit respecting the country of the city"
                },
            },
            "required": ["location", "country", "unit"],
        }),
    )?;
    let tools = Some(vec![tool_weather]);

    // -- Exec Chat Request
    let msg_req = CreateChatCompletionRequest {
        model,
        messages,
        tools,
        tool_choice: Some(ChatCompletionToolChoiceOption::Auto),
        ..Default::default()
    };
    let chat_response = chat_client.create(msg_req).await?;
    let first_choice = chat::first_choice(chat_response)?;

    // -- Extract and print the tool calls
    if let Some(tool_calls) = first_choice.message.tool_calls {
        for tool in tool_calls {
            println!(
                r#"
===   function: '{}'
    arguments: {}"#,
                tool.function.name, tool.function.arguments
            );
        }
    }

    Ok(())
}
