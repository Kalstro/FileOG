use crate::error::AppError;
use crate::models::settings::LlmConfig;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassificationResult {
    pub file_path: String,
    pub suggested_category: String,
    pub suggested_name: Option<String>,
    pub confidence: f32,
    pub reasoning: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ChatMessage {
    role: String,
    content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ChatRequest {
    model: String,
    messages: Vec<ChatMessage>,
    temperature: f32,
    max_tokens: u32,
}

#[derive(Debug, Clone, Deserialize)]
struct ChatResponse {
    choices: Vec<ChatChoice>,
}

#[derive(Debug, Clone, Deserialize)]
struct ChatChoice {
    message: ChatMessage,
}

#[derive(Debug, Clone, Deserialize)]
struct OllamaResponse {
    message: ChatMessage,
}

pub struct LlmService {
    config: LlmConfig,
    client: reqwest::Client,
}

impl LlmService {
    pub fn new(config: LlmConfig) -> Self {
        Self {
            config,
            client: reqwest::Client::new(),
        }
    }

    pub async fn classify_file(
        &self,
        file_name: &str,
        file_extension: &str,
        file_size: u64,
        categories: &[String],
        custom_prompt: Option<&str>,
    ) -> Result<ClassificationResult, AppError> {
        let system_prompt = custom_prompt.unwrap_or(DEFAULT_CLASSIFICATION_PROMPT);

        let categories_str = categories.join(", ");
        let user_prompt = format!(
            "请分析以下文件并建议合适的分类：\n\
            文件名: {}\n\
            扩展名: {}\n\
            文件大小: {} bytes\n\
            可用分类: [{}]\n\n\
            请返回JSON格式：\n\
            {{\n\
              \"category\": \"分类名称\",\n\
              \"new_name\": \"建议的新名称（可选，如果文件名需要优化）\",\n\
              \"confidence\": 0.95,\n\
              \"reasoning\": \"分类原因\"\n\
            }}",
            file_name, file_extension, file_size, categories_str
        );

        let response = self.send_chat_request(system_prompt, &user_prompt).await?;

        // Parse JSON response
        let parsed = self.parse_classification_response(&response, file_name)?;

        Ok(parsed)
    }

    pub async fn classify_files_batch(
        &self,
        files: Vec<(String, String, u64)>, // (name, extension, size)
        categories: &[String],
        custom_prompt: Option<&str>,
    ) -> Result<Vec<ClassificationResult>, AppError> {
        let mut results = Vec::new();

        for (name, ext, size) in files {
            match self.classify_file(&name, &ext, size, categories, custom_prompt).await {
                Ok(result) => results.push(result),
                Err(e) => {
                    // Log error but continue with other files
                    eprintln!("Failed to classify {}: {:?}", name, e);
                    results.push(ClassificationResult {
                        file_path: name.clone(),
                        suggested_category: "others".to_string(),
                        suggested_name: None,
                        confidence: 0.0,
                        reasoning: format!("Classification failed: {:?}", e),
                    });
                }
            }
        }

        Ok(results)
    }

    async fn send_chat_request(
        &self,
        system_prompt: &str,
        user_prompt: &str,
    ) -> Result<String, AppError> {
        match self.config.provider.as_str() {
            "openai" | "openai-compatible" => {
                self.send_openai_request(system_prompt, user_prompt).await
            }
            "claude" => {
                self.send_claude_request(system_prompt, user_prompt).await
            }
            "ollama" => {
                self.send_ollama_request(system_prompt, user_prompt).await
            }
            _ => {
                // Default to OpenAI-compatible API
                self.send_openai_request(system_prompt, user_prompt).await
            }
        }
    }

    async fn send_openai_request(
        &self,
        system_prompt: &str,
        user_prompt: &str,
    ) -> Result<String, AppError> {
        let endpoint = if self.config.api_endpoint.is_empty() {
            "https://api.openai.com/v1/chat/completions"
        } else {
            &self.config.api_endpoint
        };

        let request = ChatRequest {
            model: self.config.model.clone(),
            messages: vec![
                ChatMessage {
                    role: "system".to_string(),
                    content: system_prompt.to_string(),
                },
                ChatMessage {
                    role: "user".to_string(),
                    content: user_prompt.to_string(),
                },
            ],
            temperature: 0.3,
            max_tokens: 500,
        };

        let response = self.client
            .post(endpoint)
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .map_err(|e| AppError::LlmError(format!("Request failed: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(AppError::LlmError(format!(
                "API error {}: {}",
                status, body
            )));
        }

        let chat_response: ChatResponse = response
            .json()
            .await
            .map_err(|e| AppError::LlmError(format!("Failed to parse response: {}", e)))?;

        chat_response
            .choices
            .first()
            .map(|c| c.message.content.clone())
            .ok_or_else(|| AppError::LlmError("Empty response".to_string()))
    }

    async fn send_claude_request(
        &self,
        system_prompt: &str,
        user_prompt: &str,
    ) -> Result<String, AppError> {
        let endpoint = if self.config.api_endpoint.is_empty() {
            "https://api.anthropic.com/v1/messages"
        } else {
            &self.config.api_endpoint
        };

        let request = serde_json::json!({
            "model": self.config.model,
            "max_tokens": 500,
            "system": system_prompt,
            "messages": [
                {
                    "role": "user",
                    "content": user_prompt
                }
            ]
        });

        let response = self.client
            .post(endpoint)
            .header("x-api-key", &self.config.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .map_err(|e| AppError::LlmError(format!("Request failed: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(AppError::LlmError(format!(
                "API error {}: {}",
                status, body
            )));
        }

        let json: serde_json::Value = response
            .json()
            .await
            .map_err(|e| AppError::LlmError(format!("Failed to parse response: {}", e)))?;

        json["content"][0]["text"]
            .as_str()
            .map(|s| s.to_string())
            .ok_or_else(|| AppError::LlmError("Empty response".to_string()))
    }

    async fn send_ollama_request(
        &self,
        system_prompt: &str,
        user_prompt: &str,
    ) -> Result<String, AppError> {
        let endpoint = if self.config.api_endpoint.is_empty() {
            "http://localhost:11434/api/chat"
        } else {
            &self.config.api_endpoint
        };

        let request = serde_json::json!({
            "model": self.config.model,
            "messages": [
                {
                    "role": "system",
                    "content": system_prompt
                },
                {
                    "role": "user",
                    "content": user_prompt
                }
            ],
            "stream": false
        });

        let response = self.client
            .post(endpoint)
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .map_err(|e| AppError::LlmError(format!("Request failed: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(AppError::LlmError(format!(
                "API error {}: {}",
                status, body
            )));
        }

        let ollama_response: OllamaResponse = response
            .json()
            .await
            .map_err(|e| AppError::LlmError(format!("Failed to parse response: {}", e)))?;

        Ok(ollama_response.message.content)
    }

    fn parse_classification_response(
        &self,
        response: &str,
        file_path: &str,
    ) -> Result<ClassificationResult, AppError> {
        // Try to extract JSON from the response
        let json_str = if let Some(start) = response.find('{') {
            if let Some(end) = response.rfind('}') {
                &response[start..=end]
            } else {
                response
            }
        } else {
            response
        };

        #[derive(Deserialize)]
        struct ParsedResponse {
            category: String,
            new_name: Option<String>,
            confidence: Option<f32>,
            reasoning: Option<String>,
        }

        let parsed: ParsedResponse = serde_json::from_str(json_str)
            .map_err(|e| AppError::LlmError(format!("Failed to parse LLM response: {}", e)))?;

        Ok(ClassificationResult {
            file_path: file_path.to_string(),
            suggested_category: parsed.category,
            suggested_name: parsed.new_name,
            confidence: parsed.confidence.unwrap_or(0.8),
            reasoning: parsed.reasoning.unwrap_or_default(),
        })
    }
}

const DEFAULT_CLASSIFICATION_PROMPT: &str = r#"你是一个文件分类助手。根据文件名、扩展名和文件大小，将文件分类到合适的类别中。

分类规则：
- documents: 文档文件（.doc, .docx, .pdf, .txt, .md, .xlsx, .pptx等）
- images: 图片文件（.jpg, .jpeg, .png, .gif, .bmp, .svg, .webp等）
- videos: 视频文件（.mp4, .avi, .mkv, .mov, .wmv等）
- music: 音频文件（.mp3, .wav, .flac, .aac, .ogg等）
- code: 代码文件（.js, .ts, .py, .rs, .go, .java, .cpp等）
- archives: 压缩文件（.zip, .rar, .7z, .tar, .gz等）
- others: 其他无法分类的文件

请仅返回JSON格式的响应，不要添加其他文字。"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_classification_response() {
        let config = LlmConfig {
            provider: "openai".to_string(),
            api_key: "test".to_string(),
            api_endpoint: String::new(),
            model: "gpt-4".to_string(),
            supports_vision: false,
        };
        let service = LlmService::new(config);

        let response = r#"{"category": "documents", "confidence": 0.95, "reasoning": "PDF file"}"#;
        let result = service.parse_classification_response(response, "test.pdf").unwrap();

        assert_eq!(result.suggested_category, "documents");
        assert_eq!(result.confidence, 0.95);
    }
}
