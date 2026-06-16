use niri_notch_rs::install::{
    InstallPlan, MANAGED_END, MANAGED_START, apply_install_block, apply_uninstall_block,
    build_managed_block,
};

#[test]
fn managed_block_contains_spawn_and_stable_markers() {
    let block = build_managed_block("~/.local/bin/niri-notch-rs");

    assert!(block.contains(MANAGED_START));
    assert!(block.contains("spawn-sh-at-startup \"~/.local/bin/niri-notch-rs\""));
    assert!(block.contains(MANAGED_END));
}

#[test]
fn install_appends_managed_block_once() {
    let original = "spawn-sh-at-startup \"waybar\"\n";
    let plan = InstallPlan::new("~/.local/bin/niri-notch-rs");

    let installed = apply_install_block(original, &plan);
    let reinstalled = apply_install_block(&installed, &plan);

    assert!(installed.contains(MANAGED_START));
    assert_eq!(installed, reinstalled);
}

#[test]
fn install_replaces_old_managed_block() {
    let old = format!(
        "spawn-sh-at-startup \"waybar\"\n{MANAGED_START}\nspawn-sh-at-startup \"old\"\n{MANAGED_END}\n"
    );
    let plan = InstallPlan::new("~/.local/bin/niri-notch-rs");

    let installed = apply_install_block(&old, &plan);

    assert!(!installed.contains("spawn-sh-at-startup \"old\""));
    assert!(installed.contains("spawn-sh-at-startup \"~/.local/bin/niri-notch-rs\""));
}

#[test]
fn uninstall_removes_only_managed_block() {
    let plan = InstallPlan::new("~/.local/bin/niri-notch-rs");
    let installed = apply_install_block("spawn-sh-at-startup \"waybar\"\n", &plan);

    let uninstalled = apply_uninstall_block(&installed);

    assert_eq!(uninstalled, "spawn-sh-at-startup \"waybar\"\n");
}
