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
        let output = RegistrationHooks::run(self.pre_register.as_str()).await?;
        Ok(output)
    }

    pub async fn exec_post_register(&self) -> Result<String, Box<dyn std::error::Error>> {
        let output = RegistrationHooks::run(self.post_register.as_str()).await?;
        Ok(output)
    }

    async fn run(command: &str) -> Result<String, Box<dyn std::error::Error>> {
        let mut fmt_output = String::new();
        if command.len() > 0 {
            let output = Command::new(command).output().await?;

            if output.status.success() {
                fmt_output.push_str(str::from_utf8(&output.stdout[..]).unwrap());
            } else {
                fmt_output.push_str(str::from_utf8(&output.stderr[..]).unwrap());
                return Err(Box::new(HookError {
                    output: fmt_output,
                    exit_code: output.status.to_string(),
                }));
            }
        }
        Ok(fmt_output)
    }
}

#[derive(Debug, Clone)]
struct HookError {
    output: String,
    exit_code: String,
}

impl fmt::Display for HookError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_fmt(format_args!("Exitcode: {} {}", self.exit_code, self.output))
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
