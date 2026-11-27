# SigmaOS-blender

SigmaOS-blender provides a Rust library and CLI for reading Blender `.blend` file metadata. It parses the magic header and version information without loading the entire file.

## Features
- Detects `.blend` magic bytes.
- Extracts Blender version (major, minor, subminor).
- Provides a CLI to display metadata.

## Usage
```bash
sigmaos-blender /path/to/file.blend
```

## See Also

Related SigmaOS **tools** repositories:

- [SigmaOS-cline](../SigmaOS-cline/README.md) — AI-powered code editor integration for SigmaOS
- [SigmaOS-codexcli](../SigmaOS-codexcli/README.md) — OpenAI Codex CLI integration for SigmaOS
- [SigmaOS-crewai](../SigmaOS-crewai/README.md) — CrewAI multi-agent orchestration for SigmaOS
- [SigmaOS-cryptomator](../SigmaOS-cryptomator/README.md) — Cryptomator vault management for SigmaOS
- [SigmaOS-deepseek-coder](../SigmaOS-deepseek-coder/README.md) — DeepSeek Coder AI integration for SigmaOS
