# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 1.0-preview   | :white_check_mark: |
| < 1.0   | :x:                |

## Reporting a Vulnerability

We take the security of A-lang seriously. If you believe you have found a security vulnerability, please report it to us as described below.

### Where to Report

**Please do NOT report security vulnerabilities through public GitHub issues.**

Instead, please report them via email to: **security@alang.dev**

### What to Include

Please include the following information:
- Type of issue (e.g. buffer overflow, SQL injection, cross-site scripting, etc.)
- Full paths of source file(s) related to the manifestation of the issue
- The location of the affected source code (tag/branch/commit or direct URL)
- Any special configuration required to reproduce the issue
- Step-by-step instructions to reproduce the issue
- Proof-of-concept or exploit code (if possible)
- Impact of the issue, including how an attacker might exploit it

### What to Expect

- **Acknowledgment**: Within 48 hours of submission
- **Initial Assessment**: Within 1 week
- **Status Updates**: Every week until resolved
- **Resolution**: Coordinated disclosure after patch is released

### Safe Harbor

We support safe harbor for security researchers who:
- Make a good faith effort to avoid privacy violations, data destruction, and service interruption
- Only interact with accounts you own or with explicit permission
- Do not exploit a security issue beyond what is necessary to demonstrate it
- Report vulnerabilities promptly
- Allow reasonable time for issues to be fixed before public disclosure

## Security Best Practices for Users

### FFI (Foreign Function Interface)

FFI is inherently unsafe. When using FFI:

- ✅ Verify library paths exist
- ✅ Only load trusted libraries
- ✅ Validate all function signatures
- ✅ Use try/catch for error handling
- ⚠️ Never pass untrusted data to C functions
- ⚠️ Be careful with pointer operations

### Input Validation

When using `input()`:

- ✅ Validate and sanitize user input
- ✅ Use type conversion (int, float) carefully
- ✅ Handle conversion errors
- ⚠️ Never execute untrusted input as code

### File Operations

- ✅ Validate file paths
- ✅ Use relative paths when possible
- ✅ Check file permissions
- ⚠️ Avoid path traversal vulnerabilities
- ⚠️ Sanitize filenames from user input

### Network Operations

- ✅ Validate URLs
- ✅ Use HTTPS when possible
- ✅ Sanitize data before sending
- ⚠️ Be careful with user-controlled URLs
- ⚠️ Validate server responses

## Known Security Considerations

### FFI Security

FFI allows calling C functions, which bypasses A-lang's safety guarantees:

- **Memory Safety**: No automatic bounds checking
- **Type Safety**: Incorrect types can cause crashes
- **Undefined Behavior**: Possible with wrong signatures

**Mitigation**: Only use FFI with trusted libraries and validated inputs.

### Reactive System

The reactive system tracks dependencies:

- **Memory Usage**: Large dependency graphs can consume memory
- **Infinite Loops**: Circular dependencies are detected but can impact performance

**Mitigation**: Keep reactive graphs simple and avoid deep nesting.

### Time-Travel Debugging

Snapshots store program state:

- **Memory Usage**: Can grow large with many snapshots
- **Sensitive Data**: Snapshots may contain sensitive information

**Mitigation**: Use snapshot limits and clear sensitive data when needed.

## Security Updates

Security updates will be:
- Released as soon as possible after verification
- Announced on GitHub Security Advisories
- Documented in release notes
- Backported to supported versions when feasible

## Questions

For questions about this security policy, contact: security@alang.dev

---

**Last Updated**: December 2024  
**Version**: 1.0
