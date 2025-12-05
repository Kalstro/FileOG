import { invoke } from "@tauri-apps/api/core";
import type { ClassificationResult, FileToClassify } from "@/types";

/**
 * Classify multiple files using LLM
 */
export async function classifyFiles(
  files: FileToClassify[]
): Promise<ClassificationResult[]> {
  return invoke<ClassificationResult[]>("classify_files", {
    request: { files },
  });
}

/**
 * Classify a single file using LLM
 */
export async function classifySingleFile(
  file: FileToClassify
): Promise<ClassificationResult> {
  return invoke<ClassificationResult>("classify_single_file", { file });
}

/**
 * Test LLM connection with current settings
 */
export async function testLlmConnection(): Promise<string> {
  return invoke<string>("test_llm_connection");
}

/**
 * Helper to extract file extension from filename
 */
export function getFileExtension(filename: string): string {
  const lastDot = filename.lastIndexOf(".");
  return lastDot > 0 ? filename.substring(lastDot + 1).toLowerCase() : "";
}
