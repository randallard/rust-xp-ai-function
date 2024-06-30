use tokio::task::JoinSet;
use xp_ai_function::{chat, conv, oa_client::new_oa_client, tools::{new_ai_tools, AiTools}};
use rpc_router::{router_builder, RpcParams};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // -- Init AI Client
    let oa_client = new_oa_client()?;

    // -- Get the AI Tools
    let ai_tools = new_ai_tools(None)?;

    // -- User question
    let questions = &[
        "What is the weather in California's best city and Paris?",
        "Why is the sky red? (be concise)",
        "What is the weather in Italy's capital",
    ];

    // -- Execute questions concurrently
    let mut join_set: JoinSet<(String, Result<String, xp_ai_function::Error>)>
        = JoinSet::new();

    for &question in questions {
        let oa_client = oa_client.clone();
        let ai_tools = ai_tools.clone();
        join_set.spawn(async move {
            // Execute user question.
            let result = conv::send_user_msg(oa_client, ai_tools, question).await;
            (question.to_string(), result)
        });
    }

    while let Some(join_result) = join_set.join_next().await {
        let (question, send_result) = join_result?;
        let response = send_result?;
        println!(
            r#"
== Question: {question}
            
{response}
            "#
        );
    }

    Ok(())
}
