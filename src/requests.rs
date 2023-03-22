use std::collections::HashMap;

use crate::openai::OpenAI;

#[cfg(not(test))]
use log::{error, info};

#[cfg(test)]
use std::{println as info, println as error};

pub type Json = serde_json::Value;
pub type ApiResult<T> = Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    ApiError(String),
    RequestError(String),
}

pub trait Requests {
    fn post(&self, url: &str, body: Json) -> ApiResult<Json>;
    fn get(&self, url: &str) -> ApiResult<Json>;
}

impl Requests for OpenAI {
    fn post(&self, sub_url: &str, body: Json) -> ApiResult<Json> {
        let mut headers = HashMap::new();
        headers.insert("Authorization", &format!("Bearer {}", self.auth.api_key));

        info!("===> 🚀 Post api: {sub_url}, body: {body}");

        let response = self
            .agent
            .post(&(self.api_url.clone() + sub_url))
            .set("Content-Type", "application/json")
            .set(
                "OpenAI-Organization",
                &self.auth.organization.clone().unwrap_or_default(),
            )
            .set("Authorization", &format!("Bearer {}", self.auth.api_key))
            .send_json(body);

        match response {
            Ok(resp) => {
                let json = resp.into_json::<Json>();
                info!("<== ✔️\n\tDone api: {sub_url}, resp: {:?}", json);
                Ok(json.unwrap())
            }
            Err(err) => {
                error!("<== ❌\n\tError api: {sub_url}, info: {err}");
                Err(Error::RequestError(err.to_string()))
            }
        }
    }

    fn get(&self, sub_url: &str) -> ApiResult<Json> {
        let mut headers = HashMap::new();
        headers.insert("Authorization", &format!("Bearer {}", self.auth.api_key));

        info!("===> 🚀 Get api: {sub_url}");

        let response = self
            .agent
            .get(&(self.api_url.clone() + sub_url))
            .set("Content-Type", "application/json")
            .set(
                "OpenAI-Organization",
                &self.auth.organization.clone().unwrap_or_default(),
            )
            .set("Authorization", &format!("Bearer {}", self.auth.api_key))
            .call();

        match response {
            Ok(resp) => {
                let json = resp.into_json::<Json>();
                info!("<== ✔️\n\tDone api: {sub_url}, resp: {:?}", json);
                Ok(json.unwrap())
            }
            Err(err) => {
                error!("<== ❌\n\t Error api: {sub_url}, info: {err}");
                Err(Error::RequestError(err.to_string()))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::openai;
    use ureq::json;

    #[test]
    fn test_post() {
        let openai = openai::new_test_openai();
        let body = json!({
            "model": "gpt-3.5-turbo",
            "messages": [{"role": "user", "content": "Say this is a test!"}],
            "temperature": 0.7
        });
        let sub_url = "chat/completions";
        let result = openai.post(sub_url, body);
        assert_eq!(result.unwrap().to_string().contains("This is a test"), true);
    }

    #[test]
    fn test_get() {
        let openai = openai::new_test_openai();
        let resp = openai.get("models");
        assert_eq!(resp.unwrap().to_string().contains("babbage"), true);
    }
}
