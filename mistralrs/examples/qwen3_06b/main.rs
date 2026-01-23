use anyhow::Result;
use mistralrs::{IsqType, TextMessageRole, TextMessages, TextModelBuilder, UqffTextModelBuilder};

#[tokio::main]
async fn main() -> Result<()> {
    let model =
        UqffTextModelBuilder::new("EricB/Qwen3-0.6B-UQFF", vec!["qwen30.6b-q4k-0.uqff".into()])
            .into_inner()
            .with_isq(IsqType::Q4K)
            .with_logging()
            .build()
            .await?;

    let mut messages = TextMessages::new();

    messages = messages.add_message(TextMessageRole::User, "Hello! How many rs in strawberry?");
    let response = model.send_chat_request(messages.clone()).await?;

    println!("{}", response.choices[0].message.content.as_ref().unwrap());

    Ok(())
}
