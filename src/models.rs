use std::fmt::Display;

pub enum EnvConfigurationError {
    NotConfigured, UnableToOpenUserShellFile(String), 
    UnableToWriteUserShellFile(String), UnableToOpenSystemShellFile(String), 
    UnableToWriteSystemShellFile(String), InvalidValueToPass, AnotherShell
}

#[derive(PartialEq, Clone)]
pub struct EnvConfiguration{
    pub home_dir: EnvConfigurationOpts,
    pub shell: EnvConfigurationOpts,
    pub os: OsType
}

#[derive(PartialEq, Clone)]
pub enum EnvConfigurationOpts {
    Initial, Specific(String), Shell(ShellType)
}

impl Display for EnvConfigurationOpts {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EnvConfigurationOpts::Shell(shell) => write!(f, "{}", shell),
            EnvConfigurationOpts::Specific(specific) => write!(f, "{}", specific),
            EnvConfigurationOpts::Initial => panic!("You cannot try to display that, it's only for default configurations.")
        }
    }
}

pub enum ShellError {
    PasswdNotExist, UnableToOpenPasswd(String), UnableToReadPasswd(String), FileMalformed, UnknownShell, UserNotFound
}

impl Display for ShellError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ShellError::PasswdNotExist => write!(f, "/etc/passwd file not exist"),
            ShellError::UnableToOpenPasswd(error) => write!(f, "{}", error.clone()),
            ShellError::UnableToReadPasswd(error) => write!(f, "{}", error.clone()),
            ShellError::FileMalformed => write!(f, "/etc/passwd file malformed."),
            ShellError::UnknownShell => write!(f, "your shell is unknown"),
            ShellError::UserNotFound => write!(f, "User not found on /etc/passwd file")
        }
    }
}

#[derive(PartialEq, Clone)]
pub enum ShellType {
    Sh, Bash, Zsh, Fish, Ksh, Csh, Tcsh, Ion, Nushell, Hush, Dash, Ash
}

impl Display for ShellType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Sh => write!(f, "Sh"),
            Self::Bash => write!(f, "Bash"),
            Self::Zsh => write!(f, "Zsh"),
            Self::Fish => write!(f, "Fish"),
            Self::Ksh => write!(f, "Ksh"),
            Self::Csh => write!(f, "Csh"),
            Self::Tcsh => write!(f, "Tcsh"),
            Self::Ion => write!(f, "Ion"),
            Self::Nushell => write!(f, "Nushell"),
            Self::Hush => write!(f, "Hush"),
            Self::Dash => write!(f, "Dash"),
            Self::Ash => write!(f, "Ash")
        }
    }
}

#[derive(PartialEq, Clone)]
pub enum CommandType {
    Version, Install, Help
}

#[derive(Clone, PartialEq)]
pub enum OsType {
    Linux, Darwin, Windows
}

#[derive(Clone)]
pub struct UserSession {
    pub os_type: OsType,
    pub os_release: String,
    pub current_user: String
}

#[derive(Clone)]
pub struct UserAction {
    pub os_type: OsType,
    pub os_release: String,
    pub current_user: String,
    pub env_confs: EnvConfiguration,
    pub first_arg: CommandType,
    pub second_arg: String,
    pub third_arg: String,
    pub fourth_arg: String,
}

pub trait Execution {
    fn execute_windows(&self) -> &Self;
    fn execute_linux(&self) -> &Self;
    fn execute_darwin(&self) -> &Self;
    fn install_rust(&self) -> &Self;
    fn install_jdk(&self) -> &Self;
    fn install_gradle(&self) -> &Self;
    fn install_maven(&self) -> &Self;
    fn install_nodejs(&self) -> &Self;
    fn install_bun(&self) -> &Self;
    fn install_yarn(&self) -> &Self;
    fn install_go(&self) -> &Self;
    fn install_python(&self) -> &Self;
}

pub trait Help {
    fn log_help(&self) -> &Self;
    fn log_help_error(&self) -> &Self;
    fn rust_help(&self) -> &Self;
    fn go_help(&self) -> &Self;
    fn nodejs_help(&self) -> &Self;
    fn bun_help(&self) -> &Self;
    fn yarn_help(&self) -> &Self;
    fn jdk_help(&self) -> &Self;
    fn gradle_help(&self) -> &Self;
    fn maven_help(&self) -> &Self;
    fn python_help(&self) -> &Self;
}

pub trait Version {
    fn log_version(&self) -> &Self;
    fn log_version_error(&self) -> &Self;
    fn rust_version(&self) -> &Self;
    fn go_version(&self) -> &Self;
    fn nodejs_version(&self) -> &Self;
    fn bun_version(&self) -> &Self;
    fn yarn_version(&self) -> &Self;
    fn jdk_version(&self) -> &Self;
    fn gradle_version(&self) -> &Self;
    fn maven_version(&self) -> &Self;
    fn python_version(&self) -> &Self;
}


