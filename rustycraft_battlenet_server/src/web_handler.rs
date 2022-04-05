use crate::utils::Http1Header;
use crate::web_models::battlenet::json::login::{
    AuthenticationState, FormInput, FormInputs, FormType, LoginForm, LoginResult,
};
use axum::extract::Extension;
use axum::http::header::HeaderName;
use axum::http::{HeaderMap, HeaderValue};
use axum::response::{Headers, IntoResponse};
use axum::routing::{get, post};
use axum::{Json, Router};
use axum_server::tls_rustls::RustlsConfig;
use axum_server::HttpConfig;
use log::{debug, info};
use rustls::ServerConfig;
use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::Arc;

pub struct WebServiceHandler {
    pub bind_address: &'static str,
    pub tls_context: ServerConfig,
}

lazy_static! {
    static ref CONTENT_TYPE_HEADERS: (HeaderName, HeaderValue) = (
        Http1Header::unsafe_cast("Content-Type"),
        HeaderValue::from_static("application/json;charset=utf-8"),
    );
}

pub struct Context {}

impl Context {
    pub fn new() -> Self {
        Context {}
    }
}

async fn get_logon(headers: HeaderMap) -> impl IntoResponse {
    debug!("{:?}", headers);
    let resp = FormInputs {
        r#type: FormType::LoginForm,
        inputs: vec![
            FormInput {
                input_id: "account_name".to_owned(),
                r#type: "text".to_owned(),
                label: "E-mail".to_owned(),
                max_length: Some(320),
            },
            FormInput {
                input_id: "password".to_owned(),
                r#type: "password".to_owned(),
                label: "Password".to_owned(),
                max_length: Some(16),
            },
            FormInput {
                input_id: "log_in_submit".to_owned(),
                r#type: "submit".to_owned(),
                label: "Log In".to_owned(),
                max_length: None,
            },
        ],
    };
    (Headers(vec![CONTENT_TYPE_HEADERS.clone()]), Json(resp))
}

pub async fn post_logon(Json(req): Json<LoginForm>, headers: HeaderMap) -> impl IntoResponse {
    debug!("{:?}", req);
    debug!("{:?}", headers);
    (
        Headers(vec![CONTENT_TYPE_HEADERS.clone()]),
        Json(LoginResult {
            authentication_state: AuthenticationState::Done,
            error_code: None,
            error_message: None,
            url: None,
            login_ticket: Some(uuid::Uuid::new_v4()),
        }),
    )
}

impl WebServiceHandler {
    pub async fn serve(self) {
        let config = HttpConfig::new()
            .http1_only(true)
            .http2_only(false)
            .max_buf_size(8192)
            .build();
        let state = Arc::new(Context::new());
        let router = Router::new()
            .route("/bnetserver/login/", get(get_logon))
            .route("/bnetserver/login/", post(post_logon))
            .layer(Extension(state));
        let addr = SocketAddr::from_str(self.bind_address).unwrap();
        info!(target: "WebServiceHandler", "Listening on address: {:?}", addr);
        axum_server::bind_rustls(
            addr,
            RustlsConfig::from_config(Arc::new(self.tls_context.clone())),
        )
        .http_config(config)
        .serve(router.into_make_service())
        .await
        .unwrap();
    }
}
