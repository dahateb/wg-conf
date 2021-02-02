use std::{fmt, str};
use tokio::process::Command;
pub struct RegistrationHooks {
    pub pre_register: String,
    pub post_register: String,
}

impl RegistrationHooks {
    pub fn new() -> RegistrationHooks {
        RegistrationHooks {
            pre_register: String::new(),
            post_register: String::new(),
        }
    }

    pub async fn exec_pre_register(&self) -> Result<String, Box<dyn std::error::Error>> {
        //self.pre_register
        let output = run(&self.pre_register).await?;
        Ok(output)
    }

    pub async fn exec_post_register(&self) -> Result<String, Box<dyn std::error::Error>> {
        let output = run(&self.post_register).await?;
        Ok(output)
    }    
}


async fn run(command: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut fmt_output = String::new();
    if command.len() > 0 {
        let output = if cfg!(target_os = "windows") {
            Command::new("cmd").args(&["/C", command]).output().await?
        } else {
            Command::new("sh").arg("-c").arg(command).output().await?
        };
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        if output.status.success() {
            fmt_output.push_str(&stdout);
        } else {
            fmt_output.push_str(&stderr);
            fmt_output.push_str(&stdout);
            return Err(Box::new(HookError {
                output: fmt_output,
                exit_code: output.status.to_string(),
            }));
        }
    }
    Ok(fmt_output)
}

#[derive(Debug, Clone)]
struct HookError {
    output: String,
    exit_code: String,
}

impl fmt::Display for HookError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_fmt(format_args!("{} {}", self.exit_code, self.output))
    }
}

impl std::error::Error for HookError {}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
