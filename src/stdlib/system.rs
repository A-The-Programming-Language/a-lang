//! System utilities module for A-lang
//! Provides process execution, environment variables, and system information

use crate::interpreter::value::Value;
use std::collections::HashMap;
use std::env;
use std::path::PathBuf;
use std::process::{Command, Output, Stdio};
use std::time::{SystemTime, UNIX_EPOCH};

/// Process execution result
#[derive(Debug, Clone)]
pub struct ProcessResult {
    pub exit_code: i32,
    pub stdout: String,
    pub stderr: String,
    pub success: bool,
}

impl ProcessResult {
    pub fn from_output(output: Output) -> Self {
        Self {
            exit_code: output.status.code().unwrap_or(-1),
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            success: output.status.success(),
        }
    }

    pub fn to_value(&self) -> Value {
        let mut result = HashMap::new();
        result.insert(
            "exitCode".to_string(),
            Value::Integer(self.exit_code as i64),
        );
        result.insert("stdout".to_string(), Value::String(self.stdout.clone()));
        result.insert("stderr".to_string(), Value::String(self.stderr.clone()));
        result.insert("success".to_string(), Value::Boolean(self.success));
        Value::Object(result)
    }
}

/// System utilities
pub struct SystemUtils;

impl SystemUtils {
    /// Execute a shell command
    pub fn exec(command: &str) -> Result<ProcessResult, String> {
        let output = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(["/C", command])
                .output()
                .map_err(|e| format!("Failed to execute command: {}", e))?
        } else {
            Command::new("sh")
                .arg("-c")
                .arg(command)
                .output()
                .map_err(|e| format!("Failed to execute command: {}", e))?
        };

        Ok(ProcessResult::from_output(output))
    }

    /// Execute a command with arguments
    pub fn exec_with_args(program: &str, args: &[String]) -> Result<ProcessResult, String> {
        let output = Command::new(program)
            .args(args)
            .output()
            .map_err(|e| format!("Failed to execute {}: {}", program, e))?;

        Ok(ProcessResult::from_output(output))
    }

    /// Execute a command with custom environment variables
    pub fn exec_with_env(
        command: &str,
        env_vars: HashMap<String, String>,
    ) -> Result<ProcessResult, String> {
        let output = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(["/C", command])
                .envs(&env_vars)
                .output()
                .map_err(|e| format!("Failed to execute command: {}", e))?
        } else {
            Command::new("sh")
                .arg("-c")
                .arg(command)
                .envs(&env_vars)
                .output()
                .map_err(|e| format!("Failed to execute command: {}", e))?
        };

        Ok(ProcessResult::from_output(output))
    }

    /// Spawn a detached process (fire and forget)
    pub fn spawn(command: &str) -> Result<u32, String> {
        let child = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(["/C", command])
                .stdin(Stdio::null())
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn()
                .map_err(|e| format!("Failed to spawn process: {}", e))?
        } else {
            Command::new("sh")
                .arg("-c")
                .arg(command)
                .stdin(Stdio::null())
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn()
                .map_err(|e| format!("Failed to spawn process: {}", e))?
        };

        Ok(child.id())
    }

    /// Get environment variable
    pub fn get_env(key: &str) -> Option<String> {
        env::var(key).ok()
    }

    /// Set environment variable for the current process
    pub fn set_env(key: &str, value: &str) {
        env::set_var(key, value);
    }

    /// Remove environment variable
    pub fn remove_env(key: &str) {
        env::remove_var(key);
    }

    /// Get all environment variables
    pub fn get_all_env() -> HashMap<String, String> {
        env::vars().collect()
    }

    /// Get current working directory
    pub fn get_cwd() -> Result<String, String> {
        env::current_dir()
            .map(|p| p.to_string_lossy().to_string())
            .map_err(|e| format!("Failed to get current directory: {}", e))
    }

    /// Change current working directory
    pub fn set_cwd(path: &str) -> Result<(), String> {
        env::set_current_dir(path)
            .map_err(|e| format!("Failed to change directory to {}: {}", path, e))
    }

    /// Get home directory
    pub fn get_home_dir() -> Option<String> {
        env::var("HOME").or_else(|_| env::var("USERPROFILE")).ok()
    }

    /// Get temporary directory
    pub fn get_temp_dir() -> String {
        env::temp_dir().to_string_lossy().to_string()
    }

    /// Get current executable path
    pub fn get_exe_path() -> Result<String, String> {
        env::current_exe()
            .map(|p| p.to_string_lossy().to_string())
            .map_err(|e| format!("Failed to get executable path: {}", e))
    }

    /// Get command line arguments
    pub fn get_args() -> Vec<String> {
        env::args().collect()
    }

    /// Get operating system name
    pub fn get_os() -> String {
        env::consts::OS.to_string()
    }

    /// Get architecture
    pub fn get_arch() -> String {
        env::consts::ARCH.to_string()
    }

    /// Get OS family
    pub fn get_os_family() -> String {
        env::consts::FAMILY.to_string()
    }

    /// Get number of CPUs
    pub fn get_cpu_count() -> usize {
        num_cpus::get()
    }

    /// Get current process ID
    pub fn get_pid() -> u32 {
        std::process::id()
    }

    /// Get current username
    pub fn get_username() -> Option<String> {
        env::var("USER").or_else(|_| env::var("USERNAME")).ok()
    }

    /// Get hostname
    pub fn get_hostname() -> Option<String> {
        env::var("HOSTNAME")
            .or_else(|_| env::var("COMPUTERNAME"))
            .ok()
    }

    /// Get current timestamp in milliseconds
    pub fn get_timestamp_ms() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64
    }

    /// Get current timestamp in seconds
    pub fn get_timestamp() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs()
    }

    /// Sleep for specified milliseconds
    pub fn sleep(millis: u64) {
        std::thread::sleep(std::time::Duration::from_millis(millis));
    }

    /// Exit the program with status code
    pub fn exit(code: i32) -> ! {
        std::process::exit(code);
    }

    /// Check if a program exists in PATH
    pub fn which(program: &str) -> Option<String> {
        if let Ok(path) = env::var("PATH") {
            let paths = env::split_paths(&path);
            for mut path in paths {
                path.push(program);

                // Check with various extensions on Windows
                if cfg!(target_os = "windows") {
                    for ext in &["", ".exe", ".bat", ".cmd"] {
                        let mut full_path = path.clone();
                        if !ext.is_empty() {
                            full_path.set_extension(&ext[1..]);
                        }
                        if full_path.exists() {
                            return Some(full_path.to_string_lossy().to_string());
                        }
                    }
                } else if path.exists() {
                    return Some(path.to_string_lossy().to_string());
                }
            }
        }
        None
    }

    /// Get system information as a HashMap
    pub fn get_system_info() -> HashMap<String, String> {
        let mut info = HashMap::new();

        info.insert("os".to_string(), Self::get_os());
        info.insert("arch".to_string(), Self::get_arch());
        info.insert("family".to_string(), Self::get_os_family());
        info.insert("cpus".to_string(), Self::get_cpu_count().to_string());
        info.insert("pid".to_string(), Self::get_pid().to_string());

        if let Some(username) = Self::get_username() {
            info.insert("username".to_string(), username);
        }

        if let Some(hostname) = Self::get_hostname() {
            info.insert("hostname".to_string(), hostname);
        }

        if let Some(home) = Self::get_home_dir() {
            info.insert("home".to_string(), home);
        }

        if let Ok(cwd) = Self::get_cwd() {
            info.insert("cwd".to_string(), cwd);
        }

        info.insert("temp".to_string(), Self::get_temp_dir());

        info
    }
}

/// Path utilities
pub struct PathUtils;

impl PathUtils {
    /// Join path components
    pub fn join(components: &[String]) -> String {
        let mut path = PathBuf::new();
        for component in components {
            path.push(component);
        }
        path.to_string_lossy().to_string()
    }

    /// Get file name from path
    pub fn basename(path: &str) -> Option<String> {
        PathBuf::from(path)
            .file_name()
            .map(|s| s.to_string_lossy().to_string())
    }

    /// Get directory name from path
    pub fn dirname(path: &str) -> Option<String> {
        PathBuf::from(path)
            .parent()
            .map(|p| p.to_string_lossy().to_string())
    }

    /// Get file extension
    pub fn extname(path: &str) -> Option<String> {
        PathBuf::from(path)
            .extension()
            .map(|s| format!(".{}", s.to_string_lossy()))
    }

    /// Check if path is absolute
    pub fn is_absolute(path: &str) -> bool {
        PathBuf::from(path).is_absolute()
    }

    /// Check if path is relative
    pub fn is_relative(path: &str) -> bool {
        PathBuf::from(path).is_relative()
    }

    /// Normalize path (resolve . and ..)
    pub fn normalize(path: &str) -> String {
        let path = PathBuf::from(path);
        let mut normalized = PathBuf::new();

        for component in path.components() {
            match component {
                std::path::Component::ParentDir => {
                    normalized.pop();
                }
                std::path::Component::CurDir => {
                    // Skip current directory
                }
                _ => {
                    normalized.push(component);
                }
            }
        }

        normalized.to_string_lossy().to_string()
    }

    /// Get absolute path
    pub fn absolute(path: &str) -> Result<String, String> {
        std::fs::canonicalize(path)
            .map(|p| p.to_string_lossy().to_string())
            .map_err(|e| format!("Failed to get absolute path: {}", e))
    }

    /// Get path separator for current OS
    pub fn separator() -> String {
        std::path::MAIN_SEPARATOR.to_string()
    }
}

/// Timer and performance utilities
pub struct Timer {
    start: SystemTime,
    name: String,
}

impl Timer {
    pub fn new(name: String) -> Self {
        Self {
            start: SystemTime::now(),
            name,
        }
    }

    pub fn elapsed_ms(&self) -> u64 {
        SystemTime::now()
            .duration_since(self.start)
            .unwrap_or_default()
            .as_millis() as u64
    }

    pub fn elapsed_micros(&self) -> u64 {
        SystemTime::now()
            .duration_since(self.start)
            .unwrap_or_default()
            .as_micros() as u64
    }

    pub fn reset(&mut self) {
        self.start = SystemTime::now();
    }

    pub fn report(&self) -> String {
        format!("{}: {} ms", self.name, self.elapsed_ms())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exec() {
        let result = SystemUtils::exec("echo hello").unwrap();
        assert!(result.success);
        assert!(result.stdout.contains("hello"));
        assert_eq!(result.exit_code, 0);
    }

    #[test]
    fn test_env_vars() {
        let key = "ALANG_TEST_VAR";
        let value = "test_value";

        SystemUtils::set_env(key, value);
        assert_eq!(SystemUtils::get_env(key), Some(value.to_string()));

        SystemUtils::remove_env(key);
        assert_eq!(SystemUtils::get_env(key), None);
    }

    #[test]
    fn test_system_info() {
        let info = SystemUtils::get_system_info();
        assert!(info.contains_key("os"));
        assert!(info.contains_key("arch"));
        assert!(info.contains_key("cpus"));
    }

    #[test]
    fn test_path_utils() {
        let joined = PathUtils::join(&[
            "home".to_string(),
            "user".to_string(),
            "file.txt".to_string(),
        ]);
        assert!(joined.contains("home"));
        assert!(joined.contains("file.txt"));

        assert_eq!(
            PathUtils::basename("/path/to/file.txt"),
            Some("file.txt".to_string())
        );
        assert_eq!(PathUtils::extname("file.txt"), Some(".txt".to_string()));

        assert!(PathUtils::is_absolute("/absolute/path"));
        assert!(PathUtils::is_relative("relative/path"));
    }

    #[test]
    fn test_path_normalize() {
        let normalized = PathUtils::normalize("foo/./bar/../baz");
        assert!(normalized.contains("foo"));
        assert!(normalized.contains("baz"));
        assert!(!normalized.contains("bar"));
    }

    #[test]
    fn test_timer() {
        let timer = Timer::new("test".to_string());
        std::thread::sleep(std::time::Duration::from_millis(10));
        let elapsed = timer.elapsed_ms();
        assert!(elapsed >= 10);
    }

    #[test]
    fn test_get_os_info() {
        let os = SystemUtils::get_os();
        assert!(!os.is_empty());

        let arch = SystemUtils::get_arch();
        assert!(!arch.is_empty());

        let cpu_count = SystemUtils::get_cpu_count();
        assert!(cpu_count > 0);
    }

    #[test]
    fn test_timestamp() {
        let ts1 = SystemUtils::get_timestamp();
        std::thread::sleep(std::time::Duration::from_millis(100));
        let ts2 = SystemUtils::get_timestamp();
        assert!(ts2 > ts1);
    }

    #[test]
    fn test_cwd() {
        let cwd = SystemUtils::get_cwd().unwrap();
        assert!(!cwd.is_empty());
    }

    #[test]
    fn test_temp_dir() {
        let temp = SystemUtils::get_temp_dir();
        assert!(!temp.is_empty());
    }

    #[test]
    fn test_process_result() {
        let result = ProcessResult {
            exit_code: 0,
            stdout: "output".to_string(),
            stderr: "".to_string(),
            success: true,
        };

        let value = result.to_value();
        if let Value::Object(map) = value {
            assert!(map.contains_key("exitCode"));
            assert!(map.contains_key("stdout"));
            assert!(map.contains_key("success"));
        } else {
            panic!("Expected Value::Object");
        }
    }
}
