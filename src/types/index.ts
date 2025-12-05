// LLM Classification Types

export interface ClassifyRequest {
  files: FileToClassify[];
}

export interface FileToClassify {
  path: string;
  name: string;
  extension: string;
  size: number;
}

export interface ClassificationResult {
  file_path: string;
  suggested_category: string;
  suggested_name: string | null;
  confidence: number;
  reasoning: string;
}

// Settings Types
export interface AppSettings {
  llm: LlmSettings;
  prompts: PromptSettings;
  ui: UiSettings;
}

export interface LlmSettings {
  enabled: boolean;
  config: LlmConfig;
}

export interface LlmConfig {
  provider: string;
  api_key: string;
  api_endpoint: string;
  model: string;
  supports_vision: boolean;
}

export interface PromptSettings {
  filename_prompt: string;
  content_prompt: string;
}

export interface UiSettings {
  theme: "light" | "dark" | "system";
}

export interface Category {
  id: string;
  name: string;
  icon: string;
  color: string;
  extensions: string[];
  target_path: string;
}
