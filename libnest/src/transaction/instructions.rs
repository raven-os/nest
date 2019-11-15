use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::process::Command;

use crate::chroot::Chroot;

use super::errors::{InstructionsExecutionError, InstructionsExecutionErrorKind::*};

/// Type representing the result of an execution
/// It contains fields for the exit status, stdout, and stderr
pub type ExecutionOutput = std::process::Output;

/// Structure to control execution of the instructions.sh scripts from NPFs
#[derive(Debug, Clone)]
pub struct InstructionsExecutor {
    script_source: String,
}

static INSTRUCTIONS_PRELUDE: &str = "
before_install() {
    :
}

after_install() {
    :
}

before_remove() {
    :
}

after_remove() {
    :
}
";

impl InstructionsExecutor {
    fn find_suitable_shell(root: &Path) -> Option<std::path::PathBuf> {
        let shells = [Path::new("/bin/sh"), Path::new("/bin/bash")];

        shells.iter().find_map(|shell| {
            let chrooted_shell = shell.with_root(root);

            if !chrooted_shell.exists() {
                return None;
            }

            let mut cmd = Command::new("chroot");
            cmd.arg(root);
            cmd.arg(shell);
            cmd.arg("-c");
            cmd.arg(":");

            cmd.output().ok().and_then(|output| {
                if output.status.success() {
                    Some(shell.to_path_buf())
                } else {
                    None
                }
            })
        })
    }

    /// Creates an [`InstructionsExecutor`] from a script file
    pub fn from_script_file(
        file: &mut File,
    ) -> Result<InstructionsExecutor, InstructionsExecutionError> {
        let mut script_source = String::new();

        file.read_to_string(&mut script_source)
            .map_err(|_| CannotReadInstructions)?;

        Ok(Self { script_source })
    }

    fn execute_function(
        &self,
        func_name: &str,
        root: &Path,
    ) -> Result<ExecutionOutput, InstructionsExecutionError> {
        let shell = Self::find_suitable_shell(root).ok_or(CannotFindShell)?;
        let mut cmd = Command::new("chroot");

        cmd.arg(root);
        cmd.arg(shell);
        cmd.arg("-c");
        cmd.arg(format!(
            "{}\n{}\n{}",
            INSTRUCTIONS_PRELUDE, self.script_source, func_name
        ));

        let output = cmd.output().map_err(|_| CannotExecuteShell)?;

        if !output.status.success() {
            Err(FailureExitStatus(output).into())
        } else {
            Ok(output)
        }
    }

    /// Executes the pre-installation script
    pub fn execute_before_install(
        &self,
        root: &Path,
    ) -> Result<ExecutionOutput, InstructionsExecutionError> {
        self.execute_function("before_install", root)
    }

    /// Executes the post-installation script
    pub fn execute_after_install(
        &self,
        root: &Path,
    ) -> Result<ExecutionOutput, InstructionsExecutionError> {
        self.execute_function("after_install", root)
    }

    /// Executes the pre-uninstallation script
    pub fn execute_before_remove(
        &self,
        root: &Path,
    ) -> Result<ExecutionOutput, InstructionsExecutionError> {
        self.execute_function("before_remove", root)
    }

    /// Executes the post-uninstallation script
    pub fn execute_after_remove(
        &self,
        root: &Path,
    ) -> Result<ExecutionOutput, InstructionsExecutionError> {
        self.execute_function("after_remove", root)
    }
}
