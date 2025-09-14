use std::path::Path;
use std::process::{Command, Output};
use std::{fmt, io};

/// Custom error type for git operations
#[derive(Debug)]
pub enum GitError {
    IoError(io::Error),
    CommandFailed { operation: String, stderr: String },
    PathNotFound(String),
    NothingToCommit,
}

impl fmt::Display for GitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GitError::IoError(err) => write!(f, "IO error: {}", err),
            GitError::CommandFailed { operation, stderr } => {
                write!(f, "Git {} failed: {}", operation, stderr)
            }
            GitError::PathNotFound(path) => write!(f, "Path not found: {}", path),
            GitError::NothingToCommit => write!(f, "Nothing to commit"),
        }
    }
}

impl From<io::Error> for GitError {
    fn from(err: io::Error) -> Self {
        GitError::IoError(err)
    }
}

/// Configuration for git operations
#[derive(Debug, Clone)]
pub struct GitConfig {
    pub remote: String,
    pub branch: String,
    pub allow_unrelated_histories: bool,
}

impl Default for GitConfig {
    fn default() -> Self {
        Self {
            remote: "origin".to_string(),
            branch: "master".to_string(),
            allow_unrelated_histories: true,
        }
    }
}

/// Git repository operations
#[derive(Debug)]
pub struct GitRepo<'a> {
    path: &'a Path,
    config: GitConfig,
}

impl<'a> GitRepo<'a> {
    /// Create a new GitRepo instance
    pub fn new(path: &'a Path) -> Result<Self, GitError> {
        if !path.exists() {
            return Err(GitError::PathNotFound(path.display().to_string()));
        }

        Ok(Self {
            path,
            config: GitConfig::default(),
        })
    }

    /// Create a new GitRepo with custom configuration
    pub fn _with_config(path: &'a Path, config: GitConfig) -> Result<Self, GitError> {
        if !path.exists() {
            return Err(GitError::PathNotFound(path.display().to_string()));
        }

        Ok(Self { path, config })
    }

    /// Helper function to create git commands
    fn git_command(&self) -> Command {
        let mut cmd = Command::new("git");
        cmd.arg("-C").arg(self.path);
        cmd
    }

    /// Execute a git command and handle the result
    fn execute_git_command(&self, mut cmd: Command, operation: &str) -> Result<Output, GitError> {
        let output = cmd.output()?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            return Err(GitError::CommandFailed {
                operation: operation.to_string(),
                stderr,
            });
        }

        Ok(output)
    }

    /// Fetch changes from remote
    pub fn fetch(&self) -> Result<(), GitError> {
        eprintln!(
            "[INFO] Fetching changes from {}/{}",
            self.config.remote, self.config.branch
        );

        let mut cmd = self.git_command();
        cmd.args(["fetch", "--prune", &self.config.remote, &self.config.branch]);

        self.execute_git_command(cmd, "fetch")?;
        Ok(())
    }

    /// Merge changes from remote branch
    pub fn merge(&self) -> Result<(), GitError> {
        eprintln!(
            "[INFO] Merging changes from {}/{}",
            self.config.remote, self.config.branch
        );

        let mut cmd = self.git_command();
        cmd.args([
            "merge",
            &format!("{}/{}", self.config.remote, self.config.branch),
        ]);

        if self.config.allow_unrelated_histories {
            cmd.arg("--allow-unrelated-histories");
        }

        self.execute_git_command(cmd, "merge")?;
        Ok(())
    }

    /// Add all changes to staging
    pub fn add_all(&self) -> Result<(), GitError> {
        eprintln!("[INFO] Adding files to git");

        let mut cmd = self.git_command();
        cmd.args(["add", "."]);

        self.execute_git_command(cmd, "add")?;
        Ok(())
    }

    /// Commit changes with a message
    pub fn commit(&self, message: &str) -> Result<(), GitError> {
        eprintln!("[INFO] Committing changes");

        let mut cmd = self.git_command();
        cmd.args(["commit", "-m", message]);

        match self.execute_git_command(cmd, "commit") {
            Ok(_) => Ok(()),
            Err(GitError::CommandFailed { stderr, .. }) => {
                // Check if it's a "nothing to commit" scenario
                if stderr.contains("nothing to commit") {
                    eprintln!("[INFO] Nothing to commit");
                    Err(GitError::NothingToCommit)
                } else {
                    Err(GitError::CommandFailed {
                        operation: "commit".to_string(),
                        stderr,
                    })
                }
            }
            Err(e) => Err(e),
        }
    }

    /// Push changes to remote
    pub fn push(&self) -> Result<(), GitError> {
        eprintln!("[INFO] Pushing changes to remote");

        let mut cmd = self.git_command();
        cmd.args(["push", &self.config.remote, &self.config.branch]);

        self.execute_git_command(cmd, "push")?;
        eprintln!("[INFO] Successfully pushed to remote");
        Ok(())
    }

    /// Sync repository: fetch, merge, add, commit, and push
    pub fn sync(&self, commit_message: &str) -> Result<(), GitError> {
        self.fetch()?;
        self.merge()?;
        self.add_all()?;

        match self.commit(commit_message) {
            Ok(_) => {
                self.push()?;
                Ok(())
            }
            Err(GitError::NothingToCommit) => {
                // Nothing to commit is not an error in sync context
                Ok(())
            }
            Err(e) => Err(e),
        }
    }
}

/// Legacy function for backward compatibility
pub fn git_add(cheat_dir: &str, commit_msg: &str) {
    let path = Path::new(cheat_dir);

    match GitRepo::new(path) {
        Ok(repo) => {
            if let Err(e) = repo.sync(commit_msg) {
                eprintln!("[ERROR] Git operation failed: {}", e);
            }
        }
        Err(e) => {
            eprintln!("[ERROR] Failed to initialize git repository: {}", e);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;
    use tempfile::TempDir;

    fn create_test_repo() -> (TempDir, PathBuf) {
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        let repo_path = temp_dir.path().to_path_buf();

        // Initialize a git repository
        let output = Command::new("git")
            .args(["init"])
            .current_dir(&repo_path)
            .output()
            .expect("Failed to initialize git repo");

        assert!(output.status.success(), "Git init failed");

        // Configure git user for testing
        Command::new("git")
            .args(["config", "user.name", "Test User"])
            .current_dir(&repo_path)
            .output()
            .expect("Failed to set git user name");

        Command::new("git")
            .args(["config", "user.email", "test@example.com"])
            .current_dir(&repo_path)
            .output()
            .expect("Failed to set git user email");

        (temp_dir, repo_path)
    }

    #[test]
    fn test_git_repo_creation() {
        let (_temp_dir, repo_path) = create_test_repo();

        let repo = GitRepo::new(&repo_path);
        assert!(repo.is_ok());
    }

    #[test]
    fn test_git_repo_nonexistent_path() {
        let nonexistent_path = Path::new("/nonexistent/path");
        let repo = GitRepo::new(nonexistent_path);

        assert!(repo.is_err());
        match repo.unwrap_err() {
            GitError::PathNotFound(_) => (),
            _ => panic!("Expected PathNotFound error"),
        }
    }

    #[test]
    fn test_git_config_default() {
        let config = GitConfig::default();
        assert_eq!(config.remote, "origin");
        assert_eq!(config.branch, "master");
        assert!(config.allow_unrelated_histories);
    }

    #[test]
    fn test_git_config_custom() {
        let config = GitConfig {
            remote: "upstream".to_string(),
            branch: "main".to_string(),
            allow_unrelated_histories: false,
        };

        let (_temp_dir, repo_path) = create_test_repo();
        let repo = GitRepo::_with_config(&repo_path, config.clone());

        assert!(repo.is_ok());
        let repo = repo.unwrap();
        assert_eq!(repo.config.remote, "upstream");
        assert_eq!(repo.config.branch, "main");
        assert!(!repo.config.allow_unrelated_histories);
    }

    #[test]
    fn test_add_all_with_files() {
        let (_temp_dir, repo_path) = create_test_repo();
        let repo = GitRepo::new(&repo_path).unwrap();

        // Create a test file
        let test_file = repo_path.join("test.txt");
        fs::write(&test_file, "test content").expect("Failed to write test file");

        // Test add_all
        let result = repo.add_all();
        assert!(result.is_ok());
    }

    #[test]
    fn test_commit_with_changes() {
        let (_temp_dir, repo_path) = create_test_repo();
        let repo = GitRepo::new(&repo_path).unwrap();

        // Create and add a test file
        let test_file = repo_path.join("test.txt");
        fs::write(&test_file, "test content").expect("Failed to write test file");
        repo.add_all().expect("Failed to add files");

        // Test commit
        let result = repo.commit("Test commit");
        assert!(result.is_ok());
    }

    #[test]
    fn test_commit_nothing_to_commit() {
        let (_temp_dir, repo_path) = create_test_repo();
        let repo = GitRepo::new(&repo_path).unwrap();

        // Try to commit without any changes
        let result = repo.commit("Empty commit");

        // Should fail - either NothingToCommit or CommandFailed
        assert!(result.is_err());

        // Accept either error type as git behavior may vary
        match result.unwrap_err() {
            GitError::NothingToCommit => (),
            GitError::CommandFailed { .. } => (),
            e => panic!("Unexpected error type: {:?}", e),
        }
    }

    #[test]
    fn test_git_command_creation() {
        let (_temp_dir, repo_path) = create_test_repo();
        let repo = GitRepo::new(&repo_path).unwrap();

        let cmd = repo.git_command();
        let program = cmd.get_program();
        assert_eq!(program, "git");
    }

    #[test]
    fn test_error_display() {
        let io_error = GitError::IoError(io::Error::new(io::ErrorKind::NotFound, "test"));
        assert!(io_error.to_string().contains("IO error"));

        let cmd_error = GitError::CommandFailed {
            operation: "test".to_string(),
            stderr: "error message".to_string(),
        };
        assert!(cmd_error.to_string().contains("Git test failed"));

        let path_error = GitError::PathNotFound("/test/path".to_string());
        assert!(path_error.to_string().contains("Path not found"));

        let nothing_error = GitError::NothingToCommit;
        assert!(nothing_error.to_string().contains("Nothing to commit"));
    }

    #[test]
    fn test_legacy_git_add_function() {
        let (_temp_dir, repo_path) = create_test_repo();

        // This should not panic, even if operations fail
        git_add(&repo_path.to_string_lossy(), "test commit");
    }
}
