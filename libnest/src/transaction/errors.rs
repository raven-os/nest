//! Errors that can be returned by the transaction module

use failure::{Context, Fail};

use super::ExecutionOutput;

/// Error type for errors related to package installation
#[derive(Debug)]
pub struct InstallError {
    inner: Context<InstallErrorKind>,
}

/// Error kind describing a kind of error related to package installation
#[derive(Debug, Fail)]
pub enum InstallErrorKind {
    /// The package could not be installed because it would overwrite an existing file
    #[fail(display = "file already exists")]
    FileAlreadyExists(std::path::PathBuf),

    /// The package could not be installed because it is already installed
    #[fail(display = "package already installed")]
    PackageAlreadyInstalled,

    /// The package could not be installed because the downloaded NPF was invalid
    #[fail(display = "invalid package file")]
    InvalidPackageFile,

    /// The package could not be installed because the contained data.tar.gz was invalid
    #[fail(display = "invalid package data")]
    InvalidPackageData,

    /// The package could not be installed because its data could not be extracted
    #[fail(display = "unable to extract")]
    ExtractError(#[cause] std::io::Error),

    /// The package could not be installed because its associated log files could not be created
    #[fail(display = "unable to create the log")]
    LogCreationError(#[cause] std::io::Error),

    /// The package could not be installed its pre-install instructions returned an error
    #[fail(display = "pre-install instructions reported an error: {}", _0)]
    PreInstallInstructionsFailure(#[cause] InstructionsExecutionError),

    /// The package could not be installed its post-install instructions returned an error
    #[fail(display = "post-install instructions reported an error: {}", _0)]
    PostInstallInstructionsFailure(#[cause] InstructionsExecutionError),
}

use_as_error!(InstallError, InstallErrorKind);

/// Error type for errors related to package removal
#[derive(Debug)]
pub struct RemoveError {
    inner: Context<RemoveErrorKind>,
}

/// Error kind describing a kind of error related to package removal
#[derive(Copy, Clone, Eq, PartialEq, Debug, Fail)]
pub enum RemoveErrorKind {
    /// The package could not be removed because its log file could not be loaded
    #[fail(display = "log file not found")]
    LogFileLoadError,

    /// The package could not be completely removed because one of its files could not be removed
    #[fail(display = "cannot remove package file")]
    FileRemoveError,

    /// The package could not be completely removed because its log file could not be removed
    #[fail(display = "cannot remove log file")]
    LogFileRemoveError,
}

use_as_error!(RemoveError, RemoveErrorKind);

/// Error type for errors related to the execution of the instructions.sh script
#[derive(Debug)]
pub struct InstructionsExecutionError {
    inner: Context<InstructionsExecutionErrorKind>,
}

/// Error kind describing a kind of error related to the execution of the instructions.sh script
#[derive(Debug, Fail)]
pub enum InstructionsExecutionErrorKind {
    /// The given instructions.sh file could not be read
    #[fail(display = "cannot read the given instructions.sh file")]
    CannotReadInstructions,

    /// No suitable shell program was found to execute the instructions.sh
    #[fail(display = "cannot find a suitable shell to execute instructions.sh")]
    CannotFindShell,

    /// The chosen shell program could not be executed
    #[fail(display = "cannot execute instructions.sh using the chosen shell")]
    CannotExecuteShell,

    /// The invoked script exited with a failure status
    #[fail(display = "script exited with a failure status")]
    FailureExitStatus(ExecutionOutput),
}

use_as_error!(InstructionsExecutionError, InstructionsExecutionErrorKind);
