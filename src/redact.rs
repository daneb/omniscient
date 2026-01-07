/// Redaction engine for filtering sensitive data from commands
use crate::error::Result;
use regex::Regex;

/// Engine for redacting sensitive information from commands
pub struct RedactionEngine {
    patterns: Vec<Regex>,
    enabled: bool,
}

impl RedactionEngine {
    /// Create a new redaction engine with the given patterns
    pub fn new(pattern_strings: Vec<String>, enabled: bool) -> Result<Self> {
        let mut patterns = Vec::new();

        for pattern in pattern_strings {
            // Create case-insensitive regex patterns
            let regex = Regex::new(&format!("(?i){}", pattern)).map_err(|e| {
                crate::error::OmniscientError::redaction(format!(
                    "Invalid redaction pattern '{}': {}",
                    pattern, e
                ))
            })?;
            patterns.push(regex);
        }

        Ok(Self { patterns, enabled })
    }

    /// Check if a command should be redacted
    pub fn should_redact(&self, command: &str) -> bool {
        if !self.enabled {
            return false;
        }

        self.patterns
            .iter()
            .any(|pattern| pattern.is_match(command))
    }

    /// Redact a command if it matches any patterns
    pub fn redact(&self, command: &str) -> String {
        if self.should_redact(command) {
            "[REDACTED]".to_string()
        } else {
            command.to_string()
        }
    }

    /// Get the number of active patterns
    pub fn pattern_count(&self) -> usize {
        self.patterns.len()
    }

    /// Check if redaction is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}

impl Default for RedactionEngine {
    fn default() -> Self {
        Self::new(
            vec![
                "password".to_string(),
                "token".to_string(),
                "secret".to_string(),
                "api_key".to_string(),
                "apikey".to_string(),
            ],
            true,
        )
        .expect("Default patterns should always be valid")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_redaction_engine_creation() {
        let engine =
            RedactionEngine::new(vec!["password".to_string(), "token".to_string()], true).unwrap();

        assert_eq!(engine.pattern_count(), 2);
        assert!(engine.is_enabled());
    }

    #[test]
    fn test_should_redact_basic() {
        let engine = RedactionEngine::new(vec!["password".to_string()], true).unwrap();

        assert!(engine.should_redact("export PASSWORD=secret"));
        assert!(engine.should_redact("echo password123"));
        assert!(!engine.should_redact("git status"));
    }

    #[test]
    fn test_case_insensitive_matching() {
        let engine = RedactionEngine::new(vec!["password".to_string()], true).unwrap();

        assert!(engine.should_redact("export PASSWORD=secret"));
        assert!(engine.should_redact("export password=secret"));
        assert!(engine.should_redact("export PaSsWoRd=secret"));
    }

    #[test]
    fn test_redact_command() {
        let engine =
            RedactionEngine::new(vec!["password".to_string(), "token".to_string()], true).unwrap();

        assert_eq!(engine.redact("export PASSWORD=secret"), "[REDACTED]");
        assert_eq!(engine.redact("curl -H 'token: abc123'"), "[REDACTED]");
        assert_eq!(engine.redact("git status"), "git status");
    }

    #[test]
    fn test_disabled_redaction() {
        let engine = RedactionEngine::new(
            vec!["password".to_string()],
            false, // Disabled
        )
        .unwrap();

        assert!(!engine.should_redact("export PASSWORD=secret"));
        assert_eq!(
            engine.redact("export PASSWORD=secret"),
            "export PASSWORD=secret"
        );
    }

    #[test]
    fn test_multiple_patterns() {
        let engine = RedactionEngine::new(
            vec![
                "password".to_string(),
                "token".to_string(),
                "api_key".to_string(),
            ],
            true,
        )
        .unwrap();

        assert!(engine.should_redact("password"));
        assert!(engine.should_redact("token"));
        assert!(engine.should_redact("api_key"));
        assert!(!engine.should_redact("git commit"));
    }

    #[test]
    fn test_default_engine() {
        let engine = RedactionEngine::default();

        assert!(engine.is_enabled());
        assert!(engine.pattern_count() >= 5);

        // Test default patterns
        assert!(engine.should_redact("export PASSWORD=secret"));
        assert!(engine.should_redact("TOKEN=abc123"));
        assert!(engine.should_redact("SECRET_KEY=xyz"));
        assert!(engine.should_redact("api_key=123"));
        assert!(engine.should_redact("apikey=456"));
    }

    #[test]
    fn test_pattern_in_middle_of_command() {
        let engine = RedactionEngine::new(vec!["secret".to_string()], true).unwrap();

        assert!(engine.should_redact("export MY_SECRET=value"));
        assert!(engine.should_redact("echo secret"));
        assert!(engine.should_redact("cat /path/to/secret.txt"));
    }

    #[test]
    fn test_empty_patterns() {
        let engine = RedactionEngine::new(vec![], true).unwrap();

        assert_eq!(engine.pattern_count(), 0);
        assert!(!engine.should_redact("any command"));
        assert_eq!(engine.redact("any command"), "any command");
    }

    #[test]
    fn test_special_characters_in_command() {
        let engine = RedactionEngine::new(vec!["password".to_string()], true).unwrap();

        assert!(engine.should_redact("curl -d 'password=123' https://api.example.com"));
        assert!(engine.should_redact("echo $PASSWORD"));
        assert!(engine.should_redact("grep 'password' file.txt"));
    }

    #[test]
    fn test_invalid_regex_pattern() {
        let result = RedactionEngine::new(
            vec!["[invalid".to_string()], // Invalid regex
            true,
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_common_false_positives() {
        let engine = RedactionEngine::new(vec!["pass".to_string()], true).unwrap();

        // These should match because "pass" is in them
        assert!(engine.should_redact("password"));
        assert!(engine.should_redact("passphrase"));

        // This is a tradeoff - we match partial words for safety
        assert!(engine.should_redact("compass")); // Contains "pass"
    }
}
