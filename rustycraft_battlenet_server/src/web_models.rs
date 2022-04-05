pub mod battlenet {
    pub mod json {
        pub mod login {
            use uuid::Uuid;

            #[derive(serde::Serialize, serde::Deserialize, Debug)]
            pub struct ErrorResponse {}
            #[derive(serde::Serialize, serde::Deserialize, Debug)]
            pub struct FormInput {
                pub input_id: String,
                pub r#type: String,
                pub label: String,
                #[serde(skip_serializing_if = "Option::is_none")]
                pub max_length: Option<u32>,
            }
            #[derive(serde::Serialize, serde::Deserialize, Debug)]
            pub struct FormInputs {
                pub r#type: FormType,
                pub inputs: Vec<FormInput>,
            }
            #[derive(serde::Serialize, serde::Deserialize, Debug)]
            pub struct FormInputValue {
                pub input_id: String,
                pub value: String,
            }
            #[derive(serde::Serialize, serde::Deserialize, Debug)]
            pub struct LoginForm {
                pub platform_id: String,
                pub program_id: String,
                pub version: String,
                pub inputs: Vec<FormInputValue>,
            }
            #[derive(serde::Serialize, serde::Deserialize, Debug)]
            pub struct LoginResult {
                pub authentication_state: AuthenticationState,
                #[serde(skip_serializing_if = "Option::is_none")]
                pub error_code: Option<String>,
                #[serde(skip_serializing_if = "Option::is_none")]
                pub error_message: Option<String>,
                #[serde(skip_serializing_if = "Option::is_none")]
                pub url: Option<String>,
                #[serde(skip_serializing_if = "Option::is_none")]
                pub login_ticket: Option<Uuid>,
            }
            #[derive(serde::Serialize, serde::Deserialize, Debug)]
            pub struct LoginRefreshResult {
                pub login_ticket_expiry: u64,
                pub is_expired: Option<bool>,
            }
            #[derive(serde::Serialize, serde::Deserialize, Debug)]
            pub struct GameAccountInfo {
                pub display_name: String,
                pub expansion: u32,
                pub is_suspended: Option<bool>,
                pub is_banned: Option<bool>,
                pub suspension_expires: Option<u64>,
                pub suspension_reason: Option<String>,
            }
            #[derive(serde::Serialize, serde::Deserialize, Debug)]
            pub struct GameAccountList {
                pub game_accounts: Vec<GameAccountInfo>,
            }
            #[derive(serde::Serialize, serde::Deserialize, Debug)]
            #[repr(i32)]
            #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
            pub enum FormType {
                LoginForm = 1,
            }
            #[derive(serde::Serialize, serde::Deserialize, Debug)]
            #[repr(i32)]
            #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
            pub enum AuthenticationState {
                Login = 1,
                Legal = 2,
                Authenticator = 3,
                Done = 4,
            }
        }
    }
}
