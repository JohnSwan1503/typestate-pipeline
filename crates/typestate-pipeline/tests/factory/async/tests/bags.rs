use typestate_pipeline::TypestateFactory;

use super::error::BadInput;

// ---------------------------------------------------------------------------
// UserProfile — async setters in standalone (non-pipeline) form.
// ---------------------------------------------------------------------------

#[derive(TypestateFactory)]
#[factory(error = BadInput)]
pub struct UserProfile {
    #[field(required, setter = normalize_name_async, async_fn)]
    pub name: String,
    #[field(required, setter = validate_email_async, async_fn, fallible)]
    pub email: String,
}

pub async fn normalize_name_async(value: String) -> String {
    // Simulate an async normalization step (e.g., consulting an i18n
    // service).
    tokio::task::yield_now().await;
    value.trim().to_owned()
}

pub async fn validate_email_async(value: String) -> Result<String, BadInput> {
    tokio::task::yield_now().await;
    if value.is_empty() {
        Err(BadInput::Empty)
    } else {
        Ok(value.to_lowercase())
    }
}

// ---------------------------------------------------------------------------
// User — `finalize_async` hook.
// ---------------------------------------------------------------------------

#[derive(Debug, PartialEq, Eq)]
pub struct ConfirmedUser {
    pub name: String,
    pub confirmation_token: String,
}

pub async fn confirm_user(raw: User) -> Result<ConfirmedUser, BadInput> {
    tokio::task::yield_now().await;
    Ok(ConfirmedUser {
        name: raw.name.clone(),
        confirmation_token: format!("token-for-{}", raw.name),
    })
}

#[derive(TypestateFactory)]
#[factory(
    error = BadInput,
    finalize_async(via = confirm_user, into = ConfirmedUser, error = BadInput),
)]
pub struct User {
    #[field(required)]
    pub name: String,
}

