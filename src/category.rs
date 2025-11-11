/// Categorization engine for automatically categorizing commands
use std::collections::HashMap;

/// Engine for categorizing commands based on the command name
pub struct Categorizer {
    rules: HashMap<String, String>,
}

impl Categorizer {
    /// Create a new categorizer with default rules
    pub fn new() -> Self {
        let mut rules = HashMap::new();

        // Git commands
        for cmd in &["git", "gh"] {
            rules.insert(cmd.to_string(), "git".to_string());
        }

        // Docker commands
        for cmd in &["docker", "docker-compose", "podman"] {
            rules.insert(cmd.to_string(), "docker".to_string());
        }

        // Package managers
        for cmd in &["npm", "yarn", "pnpm", "cargo", "pip", "pip3", "gem", "bundle", 
                     "apt", "apt-get", "brew", "yum", "dnf", "pacman"] {
            rules.insert(cmd.to_string(), "package".to_string());
        }

        // File operations
        for cmd in &["ls", "cd", "mkdir", "rm", "rmdir", "cp", "mv", "cat", "less", 
                     "more", "head", "tail", "touch", "find", "grep", "awk", "sed"] {
            rules.insert(cmd.to_string(), "file".to_string());
        }

        // Network commands
        for cmd in &["curl", "wget", "ping", "ssh", "scp", "rsync", "nc", "netcat",
                     "telnet", "ftp", "sftp"] {
            rules.insert(cmd.to_string(), "network".to_string());
        }

        // Build tools
        for cmd in &["make", "cmake", "ninja", "bazel", "gradle", "mvn", "ant"] {
            rules.insert(cmd.to_string(), "build".to_string());
        }

        // Database commands
        for cmd in &["psql", "mysql", "sqlite3", "mongo", "redis-cli", "mongosh"] {
            rules.insert(cmd.to_string(), "database".to_string());
        }

        // Kubernetes/Container orchestration
        for cmd in &["kubectl", "k9s", "helm", "minikube", "kind"] {
            rules.insert(cmd.to_string(), "kubernetes".to_string());
        }

        // Cloud providers
        for cmd in &["aws", "gcloud", "az", "terraform", "terragrunt", "pulumi"] {
            rules.insert(cmd.to_string(), "cloud".to_string());
        }

        // Text editors
        for cmd in &["vim", "nvim", "nano", "emacs", "code", "subl"] {
            rules.insert(cmd.to_string(), "editor".to_string());
        }

        // System administration
        for cmd in &["sudo", "systemctl", "service", "journalctl", "top", "htop", 
                     "ps", "kill", "killall", "df", "du", "free", "uptime"] {
            rules.insert(cmd.to_string(), "system".to_string());
        }

        // Version control (other than git)
        for cmd in &["svn", "hg", "bzr"] {
            rules.insert(cmd.to_string(), "vcs".to_string());
        }

        Self { rules }
    }

    /// Categorize a command based on its first word
    pub fn categorize(&self, command: &str) -> String {
        // Extract the first word (the actual command)
        let first_word = command
            .split_whitespace()
            .next()
            .unwrap_or("");

        // Remove any path prefix (e.g., /usr/bin/git -> git)
        let cmd_name = first_word
            .rsplit('/')
            .next()
            .unwrap_or(first_word);

        // Look up in rules, return "other" if not found
        self.rules
            .get(cmd_name)
            .cloned()
            .unwrap_or_else(|| "other".to_string())
    }

    /// Get all available categories
    pub fn categories(&self) -> Vec<String> {
        let mut cats: Vec<String> = self.rules.values().cloned().collect();
        cats.sort();
        cats.dedup();
        cats
    }

    /// Get the number of categorization rules
    pub fn rule_count(&self) -> usize {
        self.rules.len()
    }
}

impl Default for Categorizer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_categorizer_creation() {
        let categorizer = Categorizer::new();
        assert!(categorizer.rule_count() > 0);
    }

    #[test]
    fn test_git_categorization() {
        let categorizer = Categorizer::new();
        
        assert_eq!(categorizer.categorize("git status"), "git");
        assert_eq!(categorizer.categorize("git commit -m 'test'"), "git");
        assert_eq!(categorizer.categorize("gh pr list"), "git");
    }

    #[test]
    fn test_docker_categorization() {
        let categorizer = Categorizer::new();
        
        assert_eq!(categorizer.categorize("docker ps"), "docker");
        assert_eq!(categorizer.categorize("docker-compose up"), "docker");
        assert_eq!(categorizer.categorize("podman run alpine"), "docker");
    }

    #[test]
    fn test_package_manager_categorization() {
        let categorizer = Categorizer::new();
        
        assert_eq!(categorizer.categorize("npm install"), "package");
        assert_eq!(categorizer.categorize("cargo build"), "package");
        assert_eq!(categorizer.categorize("pip install requests"), "package");
        assert_eq!(categorizer.categorize("brew install git"), "package");
    }

    #[test]
    fn test_file_operations_categorization() {
        let categorizer = Categorizer::new();
        
        assert_eq!(categorizer.categorize("ls -la"), "file");
        assert_eq!(categorizer.categorize("cd /tmp"), "file");
        assert_eq!(categorizer.categorize("mkdir test"), "file");
        assert_eq!(categorizer.categorize("grep pattern file.txt"), "file");
    }

    #[test]
    fn test_network_categorization() {
        let categorizer = Categorizer::new();
        
        assert_eq!(categorizer.categorize("curl https://example.com"), "network");
        assert_eq!(categorizer.categorize("ssh user@host"), "network");
        assert_eq!(categorizer.categorize("ping google.com"), "network");
    }

    #[test]
    fn test_build_tools_categorization() {
        let categorizer = Categorizer::new();
        
        assert_eq!(categorizer.categorize("make all"), "build");
        assert_eq!(categorizer.categorize("cmake .."), "build");
        assert_eq!(categorizer.categorize("gradle build"), "build");
    }

    #[test]
    fn test_database_categorization() {
        let categorizer = Categorizer::new();
        
        assert_eq!(categorizer.categorize("psql -U user"), "database");
        assert_eq!(categorizer.categorize("mysql -u root"), "database");
        assert_eq!(categorizer.categorize("sqlite3 test.db"), "database");
    }

    #[test]
    fn test_kubernetes_categorization() {
        let categorizer = Categorizer::new();
        
        assert_eq!(categorizer.categorize("kubectl get pods"), "kubernetes");
        assert_eq!(categorizer.categorize("helm install myapp"), "kubernetes");
        assert_eq!(categorizer.categorize("k9s"), "kubernetes");
    }

    #[test]
    fn test_cloud_categorization() {
        let categorizer = Categorizer::new();
        
        assert_eq!(categorizer.categorize("aws s3 ls"), "cloud");
        assert_eq!(categorizer.categorize("gcloud compute instances list"), "cloud");
        assert_eq!(categorizer.categorize("terraform apply"), "cloud");
    }

    #[test]
    fn test_system_categorization() {
        let categorizer = Categorizer::new();
        
        assert_eq!(categorizer.categorize("sudo apt update"), "system");
        assert_eq!(categorizer.categorize("systemctl status nginx"), "system");
        assert_eq!(categorizer.categorize("ps aux"), "system");
        assert_eq!(categorizer.categorize("top"), "system");
    }

    #[test]
    fn test_editor_categorization() {
        let categorizer = Categorizer::new();
        
        assert_eq!(categorizer.categorize("vim file.txt"), "editor");
        assert_eq!(categorizer.categorize("code ."), "editor");
        assert_eq!(categorizer.categorize("nano config.yml"), "editor");
    }

    #[test]
    fn test_unknown_command() {
        let categorizer = Categorizer::new();
        
        assert_eq!(categorizer.categorize("unknown_command"), "other");
        assert_eq!(categorizer.categorize("my-custom-script"), "other");
    }

    #[test]
    fn test_empty_command() {
        let categorizer = Categorizer::new();
        
        assert_eq!(categorizer.categorize(""), "other");
        assert_eq!(categorizer.categorize("   "), "other");
    }

    #[test]
    fn test_command_with_path() {
        let categorizer = Categorizer::new();
        
        assert_eq!(categorizer.categorize("/usr/bin/git status"), "git");
        assert_eq!(categorizer.categorize("/usr/local/bin/docker ps"), "docker");
    }

    #[test]
    fn test_categories_list() {
        let categorizer = Categorizer::new();
        let categories = categorizer.categories();
        
        assert!(categories.contains(&"git".to_string()));
        assert!(categories.contains(&"docker".to_string()));
        assert!(categories.contains(&"package".to_string()));
        assert!(categories.len() > 5);
    }

    #[test]
    fn test_default_categorizer() {
        let categorizer = Categorizer::default();
        assert!(categorizer.rule_count() > 0);
        assert_eq!(categorizer.categorize("git status"), "git");
    }

    #[test]
    fn test_case_sensitivity() {
        let categorizer = Categorizer::new();
        
        // Commands are case-sensitive in Unix
        assert_eq!(categorizer.categorize("git status"), "git");
        assert_eq!(categorizer.categorize("GIT status"), "other"); // Different command
    }

    #[test]
    fn test_complex_commands() {
        let categorizer = Categorizer::new();
        
        assert_eq!(
            categorizer.categorize("git commit -m 'message' --amend"), 
            "git"
        );
        assert_eq!(
            categorizer.categorize("docker run -d -p 8080:80 nginx"), 
            "docker"
        );
        assert_eq!(
            categorizer.categorize("kubectl get pods -n production --watch"), 
            "kubernetes"
        );
    }
}