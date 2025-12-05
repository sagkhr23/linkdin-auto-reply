use axum::{routing::post, Json, Router};
use serde::{Deserialize, Serialize};
use std::{env, net::SocketAddr, sync::Arc};
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};

#[derive(Deserialize)]
struct GenerateRequest {
    message: String,
    thread_context: Option<String>,
    sender_headline: Option<String>,
}

#[derive(Serialize)]
struct GenerateResponse {
    reply: String,
    reason: String,
}

#[derive(Clone)]
struct AppState {
    resume: String,
    profile: String,
    phone_number: String,
    resume_link: String,
    user_name: String,
}

#[derive(Serialize)]
struct OllamaRequest {
    model: String,
    prompt: String,
    stream: bool,
}

#[derive(Deserialize)]
struct OllamaResponse {
    response: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load .env file if it exists (optional for development)
    let _ = dotenvy::dotenv();

    // Load personal information from environment variables
    let phone_number = env::var("PHONE_NUMBER")
        .unwrap_or_else(|_| "YOUR_PHONE_NUMBER".to_string());
    let resume_link = env::var("RESUME_LINK")
        .unwrap_or_else(|_| "YOUR_RESUME_LINK".to_string());
    let user_name = env::var("USER_NAME")
        .unwrap_or_else(|_| "Your Name".to_string());

    let resume = std::fs::read_to_string("../resume-summary.txt")?;
    let profile = std::fs::read_to_string("../linkdin-about-section.txt")?;

    let state = Arc::new(AppState {
        resume,
        profile,
        phone_number,
        resume_link,
        user_name,
    });

    let cors = CorsLayer::new()
        .allow_origin(Any)    // you can later restrict this if you like
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/generate_reply", post(generate_reply))
        .with_state(state)
        .layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    println!("Listening on http://{}", addr);

    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn generate_reply(
    axum::extract::State(state): axum::extract::State<Arc<AppState>>,
    Json(body): Json<GenerateRequest>,
) -> Result<Json<GenerateResponse>, axum::http::StatusCode> {
    let prompt = format!(
        "You are an assistant that drafts LinkedIn replies to recruiter messages on behalf of {}.\n\
         Use the resume summary and LinkedIn About section below as the ONLY source of truth.\n\
         Never invent details (years of experience, current location, notice period, technologies, compensation, dates, or companies)\n\
         that are not clearly present in the resume/about text.\n\n\
         HARD RULES:\n\
         - Reply ONLY if this is clearly a recruiter or hiring-related message. If not, reply exactly with: SKIP_AUTOREPLY.\n\
         - Always write in first person as {}.\n\
         - Address the recruiter by their first name ONLY if it is clearly visible in the message; otherwise start with a generic\n\
           greeting like 'Hi' or 'Hi there' and NEVER guess or fabricate a name.\n\
         - When relocation is mentioned:\n\
           - If the role/location appears to be in India, talk only about being open to opportunities within India.\n\
           - Only mention being open to relocating outside India if the recruiter explicitly talks about roles or locations outside India.\n\
           - NEVER say that the user is not willing to relocate.\n\
         - If asked about current or expected CTC/salary/compensation, do NOT provide exact numbers and do NOT say it is confidential.\n\
           Instead say something like: \"I'm happy to discuss compensation expectations over a call ({}) once we've aligned on the role and responsibilities.\"\n\
         - If the recruiter asks for a CV/resume or it is relevant to share, include this exact sentence once in the reply:\n\
           \"You can find my resume here: {}\".\n\
           Do NOT say that a resume is attached; always point to this link instead.\n\
         - If the recruiter asks about notice period or LWD, you may mention it briefly in ONE sentence, but do not over-explain.\n\
         - FORMAT STRICTLY AS FOLLOWS:\n\
           1) Greeting line (e.g., \"Hi <name>,\").\n\
           2) Blank line.\n\
           3) 1–2 short paragraphs, each separated by a blank line, no wall of text.\n\
           4) Closing line (e.g., \"Best regards,\\n{}\").\n\
           Use \"\\n\\n\" between paragraphs so the message is easy to read in LinkedIn.\n\
           Do NOT wrap the reply in quotes, backticks, Markdown, or code blocks.\n\
           Do NOT prefix the reply with any explanation or label such as\n\
           \"Here is the draft LinkedIn reply:\", \"Answer:\", or similar.\n\
           The first characters of your output MUST be the greeting itself (e.g., \"Hi ...,\").\n\n\
         === RESUME SUMMARY ===\n{}\n\n\
         === LINKEDIN ABOUT ===\n{}\n\n\
         === SENDER HEADLINE ===\n{}\n\n\
         === THREAD CONTEXT ===\n{}\n\n\
         === LATEST MESSAGE ===\n{}\n\n\
         Write only the reply text (or SKIP_AUTOREPLY), following all rules above.",
        state.user_name,
        state.user_name,
        state.phone_number,
        state.resume_link,
        state.user_name,
        state.resume,
        state.profile,
        body.sender_headline.clone().unwrap_or_default(),
        body.thread_context.clone().unwrap_or_default(),
        body.message
    );

    let client = reqwest::Client::new();
    let ollama_req = OllamaRequest {
        model: "llama3".to_string(), // or whatever model you have pulled
        prompt,
        stream: false,
    };

    let resp = client
        .post("http://localhost:11434/api/generate")
        .json(&ollama_req)
        .send()
        .await
        .map_err(|_| axum::http::StatusCode::BAD_GATEWAY)?;

    if !resp.status().is_success() {
        return Err(axum::http::StatusCode::BAD_GATEWAY);
    }

    let ollama_resp: OllamaResponse = resp
        .json()
        .await
        .map_err(|_| axum::http::StatusCode::BAD_GATEWAY)?;

    let reply = ollama_resp.response.trim().to_string();
    let reason = if reply == "SKIP_AUTOREPLY" {
        "non_recruiter_or_unclear".to_string()
    } else {
        "recruiter_message".to_string()
    };

    Ok(Json(GenerateResponse { reply, reason }))
}