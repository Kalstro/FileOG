use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub theme: String,
    pub language: String,
    pub llm: LlmSettings,
    pub prompts: PromptSettings,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            theme: "system".to_string(),
            language: "zh-CN".to_string(),
            llm: LlmSettings::default(),
            prompts: PromptSettings::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmSettings {
    pub enabled: bool,
    pub config: LlmConfig,
}

impl Default for LlmSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            config: LlmConfig::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmConfig {
    pub provider: String,
    pub api_key: String,
    pub api_endpoint: String,
    pub model: String,
    pub supports_vision: bool,
}

impl Default for LlmConfig {
    fn default() -> Self {
        Self {
            provider: "openai".to_string(),
            api_key: String::new(),
            api_endpoint: "https://api.openai.com/v1".to_string(),
            model: "gpt-4o-mini".to_string(),
            supports_vision: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptSettings {
    pub filename_prompt: String,
    pub text_content_prompt: String,
    pub image_prompt: String,
}

impl Default for PromptSettings {
    fn default() -> Self {
        Self {
            filename_prompt: DEFAULT_FILENAME_PROMPT.to_string(),
            text_content_prompt: DEFAULT_TEXT_CONTENT_PROMPT.to_string(),
            image_prompt: DEFAULT_IMAGE_PROMPT.to_string(),
        }
    }
}

pub const DEFAULT_FILENAME_PROMPT: &str = r#"分析以下文件名，判断该文件应该属于哪个分类。
文件名: {{filename}}
可用分类: {{categories}}
请只返回最匹配的分类名称，不要包含其他内容。"#;

pub const DEFAULT_TEXT_CONTENT_PROMPT: &str = r#"分析以下文件内容，判断该文件应该属于哪个分类。
文件名: {{filename}}
文件内容（前1000字符）:
{{content}}
可用分类: {{categories}}
请只返回最匹配的分类名称，不要包含其他内容。"#;

pub const DEFAULT_IMAGE_PROMPT: &str = r#"分析这张图片的内容，判断它应该属于哪个分类。
可用分类: {{categories}}
请只返回最匹配的分类名称，不要包含其他内容。"#;
