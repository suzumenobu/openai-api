/// Given a prompt and an instruction, the model will return an edited version of the prompt.

use crate::{
    openai::OpenAI,
    requests::{ApiResult, Json, Requests},
};
use serde::{Deserialize, Serialize};

use super::{
    completions::{Completion},
    EDIT_CREATE,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct EditBody {
    pub model: String,
    pub instruction: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
}

pub trait EditApi {
    /// Creates a new edit for the provided input, instruction, and parameters.
    fn edit_create(&self, chat_body: &EditBody) -> ApiResult<Completion>;
}

impl EditApi for OpenAI {
    fn edit_create(&self, chat_body: &EditBody) -> ApiResult<Completion> {
        let request_body = serde_json::to_value(chat_body).unwrap();
        let result = self.post(EDIT_CREATE, request_body);
        let res: Json = result.unwrap();
        let completion: Completion = serde_json::from_value(res.clone()).unwrap();
        Ok(completion)
    }
}

#[cfg(test)]
mod tests {
    use crate::{openai::new_test_openai, apis::edits::{EditBody, EditApi}};

    #[test]
    fn test_edit_create() {
        let openai = new_test_openai();
        let body = EditBody {
            model: "text-davinci-edit-001".to_string(),
            temperature: None,
            top_p: None,
            n: Some(2),
            instruction: "Fix the spelling mistakes".to_string(),
            input: Some("What day of the wek is it?".to_string()),
        };
        let rs = openai.edit_create(&body);
        let choice = rs.unwrap().choices;
        let text = &choice[0].text.as_ref().unwrap();
        assert_eq!(text.contains("week"), true);
    }
}