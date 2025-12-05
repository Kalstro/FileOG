# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.0] - 2025-12-06

### Added
- Category filtering: Click sidebar categories to filter files by type
- Duplicate file detection UI: Display duplicate groups with keep/duplicate labels
- Deleted files are now recoverable: Files moved to app trash, can be restored via undo

### Changed
- Redesigned app icon with blue-teal gradient (no purple)
- Updated file type colors: video to orange, archive to cyan
- Updated delete confirmation message to indicate files can be recovered

### Fixed
- Fixed settings panel showing blank for category rules
- Fixed sidebar categories being hardcoded, now loads from user settings dynamically

## [0.1.0] - 2025-12-06

### Added
- Initial release of FileOG
- Folder scanning and file listing
- File categorization based on extension rules
- LLM-powered intelligent file classification
  - OpenAI API support
  - Claude/Anthropic API support
  - Ollama local model support
- File operations: move, copy, delete with undo support
- Duplicate file detection using SHA-256 hash
- Operation history with SQLite persistence
- Custom category management
- Dark/Light theme support
- Multi-language support (Chinese/English)
- Settings panel for LLM configuration
