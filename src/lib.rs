//! Unofficial library to use the Stych email flow in Rust
//!
//! # Usage
//!
//! ```no_run
//! use stytch::Stytch;
//!
//! // Store credentials
//! let stytch = Stytch::new(
//!     "project_id",
//!     "secret",
//!     "redirect_for_login",
//!     "redirect_for_signup"
//! );
//!
//! // Create new user
//! let user = stytch.login_or_create("root@ogriffiths.com").await?;
//!
//! // Authenticate
//! let authenticated = stytch.auth(user.token).await?;
//! if authenticated.is_ok() {
//!     println!("This user is good!");
//! } else {
//!     println!("Nope!");
//! }
//! ```

use core::fmt;

use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

/// Credential storage and link management
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Stytch {
    /// Project identifier credential
    pub project_id: String,
    /// Secret credential
    pub secret: String,
    /// Link to use to redirect to your login page
    pub link_login: String,
    /// Link to use to redirect to your signup page
    pub link_signup: String,
    /// Link to the API to contact
    api: String,
}

impl Stytch {
    /// Creates typical new credential store
    pub fn new(
        project_id: impl Into<String>,
        secret: impl Into<String>,
        link_login: impl Into<String>,
        link_signup: impl Into<String>,
    ) -> Self {
        Self::new_url(
            project_id,
            secret,
            link_login,
            link_signup,
            String::from("https://stytch.com"),
        )
    }

    /// Creates credential store with url
    pub fn new_url(
        project_id: impl Into<String>,
        secret: impl Into<String>,
        link_login: impl Into<String>,
        link_signup: impl Into<String>,
        api: impl Into<String>,
    ) -> Self {
        Self {
            project_id: project_id.into(),
            secret: secret.into(),
            link_login: link_login.into(),
            link_signup: link_signup.into(),
            api: api.into(),
        }
    }

    /// Enacts the "login or create" flow
    pub async fn login_or_create(&self, email: impl Into<String>) -> Result<User> {
        #[derive(Serialize)]
        struct RequestJson<'a> {
            email: String,
            login_magic_link_url: &'a str,
            signup_magic_link_url: &'a str,
        }

        let request_json = RequestJson {
            email: email.into(),
            login_magic_link_url: &self.link_login,
            signup_magic_link_url: &self.link_signup,
        };

        let client = reqwest::Client::new();
        let resp = client
            .post(self.api.clone() + "/v1/magic_links/email/login_or_create")
            .basic_auth(&self.project_id, Some(&self.secret))
            .json(&request_json)
            .send()
            .await?;

        let status = resp.status();
        if status != StatusCode::OK {
            return Err(Error::LoginOrCreate(status));
        }

        Ok(resp.json().await?)
    }

    /// Authorises a token, returning `Ok(())` if all is well
    pub async fn auth(&self, token: impl Into<Token>) -> Result<()> {
        #[derive(Serialize)]
        struct RequestJson {
            token: String,
        }

        let request_json = RequestJson {
            token: token.into(),
        };

        let client = reqwest::Client::new();
        let resp = client
            .post(self.api.clone() + "/v1/magic_links/authenticate")
            .basic_auth(&self.project_id, Some(&self.secret))
            .json(&request_json)
            .send()
            .await?;

        let status = resp.status();
        if status != StatusCode::OK {
            return Err(Error::Auth(status));
        }

        Ok(())
    }
}

/// Representation of a user
#[derive(Debug, PartialEq, Eq, Clone, Deserialize)]
pub struct User {
    /// The user's identifier
    pub id: String,
    /// Current token created for the user
    pub token: Token,
}

/// Type alias for tokens, with them really just being strings
pub type Token = String;

/// Crate-wide dissemination of results for ease of use
pub type Result<T> = std::result::Result<T, Error>;

/// Errors which arise from the usage of this library
#[derive(Debug)]
pub enum Error {
    /// Whilst requesting or decoding a request
    Request(reqwest::Error),
    /// Couldn't login or create because of a bad response
    LoginOrCreate(StatusCode),
    /// Couldn't authorise because of a bad response
    Auth(StatusCode),
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Self::Request(err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Request(err) => write!(f, "Request error, {}", err),
            // FIXME: better error
            Self::LoginOrCreate(_) => {
                write!(f, "Couldn't login or create because of a bad response")
            }
            // FIXME: better error
            Self::Auth(_) => write!(f, "Couldn't authorise because of a bad response"),
        }
    }
}
