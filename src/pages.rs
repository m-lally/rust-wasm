use leptos::*;
use axum::{extract::Json, response::IntoResponse};
use serde::{Deserialize, Serialize};

#[component]
pub fn app(cx: Scope) -> impl IntoView {
    let (username, set_username) = create_signal(cx, String::new());
    let (password, set_password) = create_signal(cx, String::new());
    let (message, set_message) = create_signal(cx, None::<String>);

    let on_submit = move |ev: leptos_dom::ev::SubmitEvent| {
        ev.prevent_default();
        let username = username();
        let password = password();

        // Simple client-side validation
        if username.is_empty() || password.is_empty() {
            set_message(Some("Username and password are required.".to_string()));
            return;
        }

        // Send login request to API
        wasm_bindgen_futures::spawn_local(async move {
            let client = gloo_net::http::Request::post("/api/login")
                .header("Content-Type", "application/json")
                .body(serde_json::to_string(&LoginRequest { username, password }).unwrap())
                .unwrap();

            match client.send().await {
                Ok(resp) => {
                    let api_resp: ApiResponse = resp.json().await.unwrap_or(ApiResponse {
                        success: false,
                        message: "Invalid response".to_string(),
                    });
                    set_message(Some(api_resp.message));
                }
                Err(_) => set_message(Some("Network error.".to_string())),
            }
        });
    };

    view! { cx,
        <h1>"Login"</h1>
        <form on:submit=on_submit>
            <label>
                "Username:"
                <input type="text" prop:value=username on:input=move |ev| set_username(event_target_value(&ev)) />
            </label>
            <br/>
            <label>
                "Password:"
                <input type="password" prop:value=password on:input=move |ev| set_password(event_target_value(&ev)) />
            </label>
            <br/>
            <button type="submit">"Login"</button>
        </form>
        <div>
            {move || message().map(|msg| view! { cx, <p>{msg}</p> })}
        </div>
    }
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct ApiResponse {
    pub success: bool,
    pub message: String,
}

pub async fn api_login(Json(payload): Json<LoginRequest>) -> impl IntoResponse {
    // In production, replace this with real authentication logic
    if payload.username == "admin" && payload.password == "password123" {
        axum::Json(ApiResponse {
            success: true,
            message: "Login successful!".to_string(),
        })
    } else {
        axum::Json(ApiResponse {
            success: false,
            message: "Invalid username or password.".to_string(),
        })
    }
}
