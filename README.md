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
