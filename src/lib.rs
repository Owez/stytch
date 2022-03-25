use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

pub struct Stych {
    pub project_id: String,
    pub secret: String,
    pub link_login: String,
    pub link_signup: String,
    pub api: String,
}

impl Stych {
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

    pub async fn auth(&self, token: impl Into<String>) -> Result<()> {
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

#[derive(Deserialize)]
pub struct User {
    pub id: String,
    pub token: String, // TODO: check return of login_or_create
}

pub type Result<T> = std::result::Result<T, Error>;

pub enum Error {
    Request(reqwest::Error),
    LoginOrCreate(StatusCode),
    Auth(StatusCode),
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Self::Request(err)
    }
}
