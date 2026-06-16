#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CliCommand {
    Doctor,
    Install { user: bool, dry_run: bool },
    Uninstall { user: bool, dry_run: bool },
    Preview,
    Help,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CliError {
    UnknownCommand(String),
    UnsupportedFlag { command: String, flag: String },
    MissingUserScope(String),
}

pub fn parse_cli<I, S>(args: I) -> Result<Option<CliCommand>, CliError>
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    let mut args = args.into_iter();
    let _program = args.next();
    let Some(command) = args.next() else {
        return Ok(None);
    };
    let command = command.as_ref();
    match command {
        "doctor" => Ok(Some(CliCommand::Doctor)),
        "preview" => Ok(Some(CliCommand::Preview)),
        "help" | "--help" | "-h" => Ok(Some(CliCommand::Help)),
        "install" => parse_scoped_command("install", args),
        "uninstall" => parse_scoped_command("uninstall", args),
        other => Err(CliError::UnknownCommand(other.into())),
    }
}

fn parse_scoped_command<I, S>(command: &str, args: I) -> Result<Option<CliCommand>, CliError>
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    let mut user = false;
    let mut dry_run = false;
    for arg in args {
        match arg.as_ref() {
            "--user" => user = true,
            "--dry-run" => dry_run = true,
            flag => {
                return Err(CliError::UnsupportedFlag {
                    command: command.into(),
                    flag: flag.into(),
                });
            }
        }
    }
    if !user {
        return Err(CliError::MissingUserScope(command.into()));
    }
    let parsed = match command {
        "install" => CliCommand::Install { user, dry_run },
        "uninstall" => CliCommand::Uninstall { user, dry_run },
        _ => unreachable!("parse_scoped_command only handles known scoped commands"),
    };
    Ok(Some(parsed))
}

#[must_use]
pub const fn doctor_probe_names() -> [&'static str; 7] {
    [
        "niri",
        "gtk-layer-shell-0",
        "waybar",
        "makoctl",
        "playerctl",
        "niri config",
        "niri layers",
    ]
}

#[must_use]
pub const fn help_text() -> &'static str {
    "niri-notch-rs\n\nCommands:\n  doctor\n  install --user [--dry-run]\n  uninstall --user [--dry-run]\n  preview\n"
}
