use std::{collections::HashMap, usize};
use axum::{
    extract::{Path, Query, Request},
    http::StatusCode, response::IntoResponse,
    routing::{get, post}, 
    Json, Router,
};
use serde::{Deserialize, Serialize};

/// 🚀 Entry Point: Launches the HTTP server
#[tokio::main]
async fn main(){
    let app = Router::new()
    // 📌 Routes
    .route("/", get(root_handler))
    .route("/silicon", get(silicon_status_handler))
    .route("/silicon/student", get(silicon_student_query_handler))
    .route("/silicon/student/{sic}/{branch}", get(silicon_student_details_handler))
    .route("/submit", post(submit_form_handler))
    .route("/submit/manual", post(submit_form_manual_handler))
    .fallback(handle_not_found);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
    .await.expect("Failed To Bind!");
    println!("✅ Server running at http://127.0.0.1:3000");
    axum::serve(listener, app).await.expect("Sever Crased!!!");
}

/// 📄 Root Route
async fn root_handler() -> &'static str {
    "👋 Welcome To The Axum API Server"
}

/// 📄 Fallback Route
async fn handle_not_found() -> &'static str {
    "🚫 Route Not Found"
}

/// ✅ Static Response In Json
async fn silicon_status_handler() -> Json<serde_json::Value> {
    let response = serde_json::json!({
        "status" : "success",
        "message" : "Silicon API is running"
    });
    Json(response)
} 

/// 🧭 Path Parameters
/// GET /silicon/student/{sic}/{branch}
async fn silicon_student_details_handler(Path((sic, branch)) : Path<(String, String)>) -> String {
    format!("🎓 Silicon Student :- SIC: {sic}, Branch: {branch}")
}

/// 🔍 Query Parameters
/// GET /silicon/student?sic=123&branch=cs
async fn silicon_student_query_handler(Query(params) : Query<HashMap<String, String>>) -> String {
    let student_sic = params.get("sic").unwrap(); //.unwrap_or(&"NA".to_string());
    let student_branch = params.get("branch").unwrap();  //.unwrap_or(&"Unknown".to_string());

    format!("🎓 Student Details :- SIC: {}, Branch: {}", student_sic, student_branch)
}

/// 📩 Manual Body Parsing
/// POST /submit/manual with raw Json body
async fn submit_form_manual_handler(req: Request) -> impl IntoResponse {
    println!("📥 Received raw request: {:?}", req);  

    let body = axum::body::to_bytes(req.into_body(), usize::MAX).await.unwrap();
    println!("📦 Raw Body Bytes: {:?}", body);

    let user: Result<User, _> = serde_json::from_slice(&body);

    match user {
        Ok(user) => {
            println!("✅ Parsed User Manually: {:?}", user);
            (StatusCode::OK, "✅ Form Submitted Successfully!").into_response()
        }
        Err(err) => {
            eprintln!("❌ JSON Parse Error: {}", err);
            (StatusCode::BAD_REQUEST, "❌ Invalid JSON Format!").into_response()
        }
    }
}

/// 📩 Structured JSON Form Handling
/// POST /submit with JSON body
async fn submit_form_handler(Json(payload) : Json<InputUser>) -> impl IntoResponse {
    if payload.name.is_none() || payload.age.is_none() {
        return (StatusCode::BAD_REQUEST, "🚫 Invalid Input: Name & Age Are Required!").into_response();
    }

    println!("✅ Valid Form Submission: {:?}", payload);
    (StatusCode::CREATED, "✅ Form Submitted Successfully!").into_response()
}

/// 🎯 Expected JSON Structure (optional fields)
#[derive(Deserialize, Serialize, Debug)]
struct InputUser {
    name: Option<String>,
    age: Option<u32>,
}

/// 🧾 Expected JSON Structure (strict form)
#[derive(Deserialize, Serialize, Debug)]
struct User {
    name: String,
    age: u32,
}