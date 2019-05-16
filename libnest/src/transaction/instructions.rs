use std::fs::File;
use std::io::Read;

use super::errors::{InstructionsExecutionError, InstructionsExecutionErrorKind};
use std::process::Command;

/// Type representing the result of an execution
/// It contains fields for the exit status, stdout, and stderr
pub type ExecutionOutput = std::process::Output;

/// Structure to control execution of the instructions.sh scripts from NPFs
#[derive(Debug, Clone)]
pub struct InstructionsExecutor {
    script_source: String,
}

impl InstructionsExecutor {
    /// Creates an [`InstructionsExecutor`] from a script file
    pub fn from_script_file(
        file: &mut File,
    ) -> Result<InstructionsExecutor, InstructionsExecutionError> {
        let mut script_source = String::new();

        file.read_to_string(&mut script_source)
            .map_err(|_| InstructionsExecutionErrorKind::CannotReadInstructions)?;

        Ok(Self { script_source })
    }

    fn execute_function(
        &self,
        func_name: &str,
        root: &std::path::Path,
    ) -> Result<ExecutionOutput, InstructionsExecutionError> {
        let mut cmd = Command::new("chroot");

        cmd.arg(root.display().to_string());
        cmd.arg("/bin/sh"); // TODO: find a shell dynamically
        cmd.arg("-c");
        cmd.arg(format!("{}\n{}", self.script_source, func_name));

        let output = cmd
            .output()
            .map_err(|_| InstructionsExecutionErrorKind::CannotExecuteShell)?;

        if !output.status.success() {
            Err(InstructionsExecutionErrorKind::FailureExitStatus(output).into())
        } else {
            Ok(output)
        }
    }

    /// Executes the pre-installation script
    pub fn execute_before_install(
        &self,
        root: &std::path::Path,
    ) -> Result<ExecutionOutput, InstructionsExecutionError> {
        self.execute_function("before_install", root)
    }

    /// Executes the post-installation script
    pub fn execute_after_install(
        &self,
        root: &std::path::Path,
    ) -> Result<ExecutionOutput, InstructionsExecutionError> {
        self.execute_function("after_install", root)
    }

    /// Executes the pre-uninstallation script
    pub fn execute_before_remove(
        &self,
        root: &std::path::Path,
    ) -> Result<ExecutionOutput, InstructionsExecutionError> {
        self.execute_function("before_remove", root)
    }

    /// Executes the post-uninstallation script
    pub fn execute_after_remove(
        &self,
        root: &std::path::Path,
    ) -> Result<ExecutionOutput, InstructionsExecutionError> {
        self.execute_function("after_remove", root)
    }
}
