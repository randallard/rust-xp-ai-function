use xp_ai_function::oa_client::{self, new_oa_client};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // -- Init AI Client
    let oa_client = new_oa_client()?;

    // -- User question
    let question = "Why is the sky red? (be concise)";

    // -- Build messages

    // -- Exec Chat Request

    // -- Display response

    Ok(())
}
