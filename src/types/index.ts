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

// Settings Types (matches backend settings.rs)
export interface AppSettings {
  theme: string;
  language: string;
  llm: LlmSettings;
  prompts: PromptSettings;
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
  text_content_prompt: string;
  image_prompt: string;
}

// Category types (matches backend category.rs)
export interface Category {
  id: string;
  name: string;
  description?: string;
  target_folder: string;
  rules: CategoryRule[];
  icon?: string;
  color?: string;
}

export interface CategoryRule {
  rule_type: "extension" | "nameContains" | "nameRegex" | "mimeType" | "llmKeyword";
  pattern: string;
  priority: number;
}

// Operation types
export interface PlannedOperation {
  file_id: string;
  file_name: string;
  operation_type: "move" | "copy" | "rename" | "delete";
  source: string;
  destination: string;
  category?: string;
}

export interface Operation {
  id: string;
  operation_type: "move" | "copy" | "rename" | "delete";
  source_path: string;
  destination_path?: string;
  original_name?: string;
  new_name?: string;
  timestamp: number;
  status: "pending" | "in_progress" | "completed" | "undone" | string;
  batch_id?: string;
  backup_path?: string;
}

export interface OperationBatch {
  id: string;
  operations: Operation[];
  created_at: number;
  description: string;
}
