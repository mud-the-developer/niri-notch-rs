pub const MANAGED_START: &str = "// BEGIN managed by niri-notch-rs";
pub const MANAGED_END: &str = "// END managed by niri-notch-rs";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InstallPlan {
    pub binary: String,
}

impl InstallPlan {
    #[must_use]
    pub fn new(binary: impl Into<String>) -> Self {
        Self {
            binary: binary.into(),
        }
    }
}

#[must_use]
pub fn build_managed_block(binary: &str) -> String {
    format!("{MANAGED_START}\nspawn-sh-at-startup \"{binary}\"\n{MANAGED_END}\n")
}

#[must_use]
pub fn apply_install_block(config: &str, plan: &InstallPlan) -> String {
    let base = apply_uninstall_block(config);
    let mut output = base.trim_end_matches('\n').to_string();
    if !output.is_empty() {
        output.push_str("\n\n");
    }
    output.push_str(&build_managed_block(&plan.binary));
    output
}

#[must_use]
pub fn apply_uninstall_block(config: &str) -> String {
    let Some(start) = config.find(MANAGED_START) else {
        return config.to_string();
    };
    let Some(end_relative) = config[start..].find(MANAGED_END) else {
        return config.to_string();
    };
    let end = start + end_relative + MANAGED_END.len();
    let mut after = &config[end..];
    if let Some(stripped) = after.strip_prefix('\n') {
        after = stripped;
    }
    let mut output = String::with_capacity(config.len());
    let before = config[..start].trim_end_matches('\n');
    if !before.is_empty() {
        output.push_str(before);
        output.push('\n');
    }
    if !after.is_empty() {
        output.push_str(after);
    }
    output
}
