# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2025-05-17

### Added
- **Models Query Support**: Added functionality to query available AI models
  - Added structs for AI models list representation
  - Implemented support for querying models from AI service
  - Added function documentation and removed deprecated comments

- **Structured Output**: Implemented structured output functionality
  - Added structured output support for AI responses
  - Updated request types and added JSON schema support

- **Basic Chat functionality**: Implemented basic chat functionality
  - Added support for user messages and AI responses
  - Implemented message formatting and display

- **Initial Implementation**: Basic project setup and core functionality
  - Environment file `.env` support for API key management
  - Project initialization and basic structure with `ai` and `ai-cli` crates
  - Added GitHub Actions workflow for CI/CD
