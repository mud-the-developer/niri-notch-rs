use niri_notch_rs::cli::{CliCommand, CliError, doctor_probe_names, parse_cli};

#[test]
fn parse_doctor_command() {
    let command = parse_cli(["niri-notch-rs", "doctor"]);

    assert_eq!(command, Ok(Some(CliCommand::Doctor { json: false })));
}

#[test]
fn parse_doctor_json_command() {
    let command = parse_cli(["niri-notch-rs", "doctor", "--json"]);

    assert_eq!(command, Ok(Some(CliCommand::Doctor { json: true })));
}

#[test]
fn parse_install_user_dry_run_command() {
    let command = parse_cli(["niri-notch-rs", "install", "--user", "--dry-run"]);

    assert_eq!(
        command,
        Ok(Some(CliCommand::Install {
            user: true,
            dry_run: true,
        }))
    );
}

#[test]
fn parse_uninstall_user_dry_run_command() {
    let command = parse_cli(["niri-notch-rs", "uninstall", "--user", "--dry-run"]);

    assert_eq!(
        command,
        Ok(Some(CliCommand::Uninstall {
            user: true,
            dry_run: true,
        }))
    );
}

#[test]
fn parse_no_subcommand_runs_gui() {
    let command = parse_cli(["niri-notch-rs"]);

    assert_eq!(command, Ok(None));
}

#[test]
fn parse_unknown_command_is_error() {
    let command = parse_cli(["niri-notch-rs", "wat"]);

    assert_eq!(command, Err(CliError::UnknownCommand("wat".into())));
}

#[test]
fn doctor_probe_list_is_stable_and_distribution_oriented() {
    let probes = doctor_probe_names();

    assert_eq!(
        probes,
        [
            "niri",
            "gtk-layer-shell-0",
            "waybar",
            "makoctl",
            "playerctl",
            "niri config",
            "niri layers",
        ]
    );
}
