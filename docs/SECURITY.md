# Security Policy

##  Supported Versions

Currently supported versions with security updates:

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |

---

##  Reporting a Vulnerability

We take the security of UrbanFlux seriously. If you discover a security vulnerability, please follow these steps:

### Private Disclosure Process

**DO NOT** create a public GitHub issue for security vulnerabilities.

Instead:

1. **Email Security Team**
   - Send details to: `security@urbanflux.dev` (replace with actual email)
   - Subject: `[SECURITY] Brief description`

2. **Include in Your Report**
   - Description of the vulnerability
   - Steps to reproduce
   - Potential impact
   - Suggested fix (if any)
   - Your contact information

3. **Expected Response Time**
   - Initial response: Within 48 hours
   - Status update: Within 7 days
   - Fix timeline: Depends on severity

### What to Expect

1. **Acknowledgment**: We'll confirm receipt of your report
2. **Investigation**: We'll investigate and validate the issue
3. **Fix Development**: We'll develop and test a fix
4. **Disclosure**: Coordinated disclosure with credit to reporter
5. **Release**: Security patch released with advisory

---

##  Security Best Practices

### For Users

1. **Keep Dependencies Updated**
   ```bash
   cargo update
   cargo audit
   ```

2. **Use Environment Variables**
   - Never commit secrets to Git
   - Use `.env` for local development
   - Use secret managers in production

3. **Secure Database Connections**
   - Use strong passwords
   - Enable SSL/TLS for PostgreSQL
   - Restrict network access

4. **Review Permissions**
   - Use least-privilege database roles
   - Separate read and write access
   - Audit user permissions

### For Developers

1. **Dependency Security**
   ```bash
   # Install cargo-audit
   cargo install cargo-audit
   
   # Run audit
   cargo audit
   ```

2. **Input Validation**
   - Validate all CSV data
   - Sanitize user inputs
   - Use parameterized queries (SQLx does this)

3. **Error Handling**
   - Don't expose internal details in errors
   - Sanitize error messages
   - Log securely (no secrets)

4. **Secrets Management**
   ```rust
   // ✅ Good
   let password = env::var("PGPASSWORD")?;
   
   // ❌ Bad
   let password = "hardcoded_password";
   ```

---

## 🔐 Security Features

### Current Implementation

- **SQLx Prepared Statements**: Protection against SQL injection
- **Type Safety**: Rust's type system prevents many common bugs
- **Memory Safety**: No buffer overflows or use-after-free
- **Environment-Based Config**: Secrets via environment variables
- **Log Sanitization**: Passwords and secrets not logged

### Planned Enhancements

- [ ] TLS/SSL for database connections
- [ ] Input rate limiting
- [ ] Audit logging
- [ ] Secret rotation support
- [ ] SIEM integration

---

##  Vulnerability Severity

We use [CVSS v3.1](https://www.first.org/cvss/) to assess severity:

| Severity | CVSS Score | Response Time |
|----------|------------|---------------|
| Critical | 9.0-10.0   | < 24 hours    |
| High     | 7.0-8.9    | < 7 days      |
| Medium   | 4.0-6.9    | < 30 days     |
| Low      | 0.1-3.9    | < 90 days     |

---

##  Security Checklist

Before each release, we verify:

- [ ] All dependencies up to date
- [ ] `cargo audit` passes with no warnings
- [ ] No secrets in code or config files
- [ ] All inputs validated
- [ ] Error messages don't leak sensitive data
- [ ] Logging doesn't contain secrets
- [ ] Security documentation updated
- [ ] Threat model reviewed

---

##  Security Audit History

| Date       | Type          | Findings | Status   |
|------------|---------------|----------|----------|
| 2025-11-11 | Initial Setup | None     | Complete |

---

##  Security Tools

### Recommended Tools

1. **cargo-audit**: Dependency vulnerability scanning
   ```bash
   cargo install cargo-audit
   cargo audit
   ```

2. **cargo-deny**: Dependency policy enforcement
   ```bash
   cargo install cargo-deny
   cargo deny check
   ```

3. **cargo-outdated**: Find outdated dependencies
   ```bash
   cargo install cargo-outdated
   cargo outdated
   ```

4. **cargo-geiger**: Unsafe code detection
   ```bash
   cargo install cargo-geiger
   cargo geiger
   ```

### CI Integration

Our CI pipeline includes:
- Dependency auditing (cargo-audit)
- Linting (clippy with security warnings)
- Static analysis
- Docker image scanning

---

##  Contact

For security concerns:
- **Email**: security@urbanflux.dev (replace with actual)
- **PGP Key**: [Link to PGP key] (if applicable)

For general questions:
- **GitHub Issues**: For non-security bugs
- **GitHub Discussions**: For general questions

---

##  Security Acknowledgments

We appreciate security researchers who responsibly disclose vulnerabilities. Contributors will be acknowledged in:
- Security advisories
- Release notes
- SECURITY.md (this file)

---

##  Resources

- [Rust Security Guidelines](https://anssi-fr.github.io/rust-guide/)
- [OWASP Top 10](https://owasp.org/www-project-top-ten/)
- [CWE Top 25](https://cwe.mitre.org/top25/)
- [SQLx Security](https://github.com/launchbadge/sqlx/blob/main/SECURITY.md)

---

##  Updates

This security policy is reviewed and updated:
- After each security incident
- Quarterly as part of routine maintenance
- When security practices change

Last updated: 2025-11-11

---

**Thank you for helping keep UrbanFlux and its users safe!**
