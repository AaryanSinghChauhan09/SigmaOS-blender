# SigmaOS-blender — Agent Guidelines

## Component Purpose
Blender project/file manager that reads .blend file metadata including magic bytes and version information.

## Principles Applied
- Single Responsibility
- Error Handling
- Memory Safety
- Safe Concurrency

## Do Not
- Use unsafe code
- Hardcode credentials
- Skip error handling

## Testing
Run: cargo test
