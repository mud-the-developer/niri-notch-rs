#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UiMode {
    Compact,
    Open,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppState {
    pub mode: UiMode,
    pub hovered: bool,
    pub title: String,
    pub media: String,
    pub chip: String,
    pub notification_sig: Option<String>,
    pub cached_notification_text: Option<String>,
    pub cached_notification_chip: Option<String>,
    pub auto_open_until_ms: Option<u64>,
    pub workspaces: Vec<String>,
    pub active_workspace: usize,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            mode: UiMode::Compact,
            hovered: false,
            title: "Desktop".into(),
            media: "알림 준비됨".into(),
            chip: "Niri".into(),
            notification_sig: None,
            cached_notification_text: None,
            cached_notification_chip: None,
            auto_open_until_ms: None,
            workspaces: vec!["1".into(), "2".into()],
            active_workspace: 0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AppEvent {
    HoverChanged(bool),
    SnapshotUpdated {
        title: String,
        media: String,
        chip: String,
        notification_sig: Option<String>,
        workspaces: Vec<String>,
        active_workspace: usize,
        now_ms: u64,
    },
    PrimaryClick {
        now_ms: u64,
    },
    SecondaryClick {
        now_ms: u64,
    },
    ScrollUp {
        now_ms: u64,
    },
    ScrollDown {
        now_ms: u64,
    },
    Tick {
        now_ms: u64,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Effect {
    DismissMakoAll { delay_ms: u64 },
    PlayerctlPlayPause,
    SpawnControlCenter,
    NiriAction(String),
}

pub const NOTIFICATION_OPEN_MS: u64 = 4_200;
pub const NOTIFICATION_DISMISS_DELAY_MS: u64 = 650;
pub const MEDIA_FEEDBACK_MS: u64 = 950;
pub const MENU_FEEDBACK_MS: u64 = 1_200;
pub const WORKSPACE_FEEDBACK_MS: u64 = 900;

#[must_use]
pub fn reduce(state: &AppState, event: AppEvent) -> (AppState, Vec<Effect>) {
    let mut next = state.clone();
    let mut effects = Vec::new();

    match event {
        AppEvent::HoverChanged(hovered) => {
            next.hovered = hovered;
            sync_mode(&mut next, 0);
        }
        AppEvent::SnapshotUpdated {
            title,
            media,
            chip,
            notification_sig,
            workspaces,
            active_workspace,
            now_ms,
        } => {
            next.title = title;
            next.workspaces = workspaces;
            next.active_workspace = active_workspace;

            match (&state.notification_sig, &notification_sig) {
                (old, Some(new_sig)) if old.as_ref() != Some(new_sig) => {
                    next.media = media.clone();
                    next.chip = chip.clone();
                    next.cached_notification_text = Some(media);
                    next.cached_notification_chip = Some(chip);
                    next.auto_open_until_ms = Some(now_ms + NOTIFICATION_OPEN_MS);
                    next.notification_sig = Some(new_sig.clone());
                    effects.push(Effect::DismissMakoAll {
                        delay_ms: NOTIFICATION_DISMISS_DELAY_MS,
                    });
                }
                (_, None) if is_auto_open(&next, now_ms) => {
                    if let Some(text) = &state.cached_notification_text {
                        next.media.clone_from(text);
                    }
                    if let Some(cached_chip) = &state.cached_notification_chip {
                        next.chip.clone_from(cached_chip);
                    }
                    next.notification_sig.clone_from(&state.notification_sig);
                }
                (_, None) => {
                    next.media = media;
                    next.chip = chip;
                    next.notification_sig = None;
                    next.cached_notification_text = None;
                    next.cached_notification_chip = None;
                }
                _ => {
                    next.media = media;
                    next.chip = chip;
                    next.notification_sig = notification_sig;
                }
            }
            sync_mode(&mut next, now_ms);
        }
        AppEvent::PrimaryClick { now_ms } => {
            if state.notification_sig.is_some() {
                next.notification_sig = None;
                next.cached_notification_text = None;
                next.cached_notification_chip = None;
                next.auto_open_until_ms = None;
                effects.push(Effect::DismissMakoAll { delay_ms: 150 });
            } else {
                next.media = "󰎈 미디어 토글".into();
                next.chip = "Media".into();
                next.auto_open_until_ms = Some(now_ms + MEDIA_FEEDBACK_MS);
                effects.push(Effect::PlayerctlPlayPause);
            }
            sync_mode(&mut next, now_ms);
        }
        AppEvent::SecondaryClick { now_ms } => {
            next.media = "󰀻 Control Center".into();
            next.chip = "Menu".into();
            next.auto_open_until_ms = Some(now_ms + MENU_FEEDBACK_MS);
            effects.push(Effect::SpawnControlCenter);
            sync_mode(&mut next, now_ms);
        }
        AppEvent::ScrollUp { now_ms } => {
            next.auto_open_until_ms = Some(now_ms + WORKSPACE_FEEDBACK_MS);
            effects.push(Effect::NiriAction("focus-workspace-up".into()));
            sync_mode(&mut next, now_ms);
        }
        AppEvent::ScrollDown { now_ms } => {
            next.auto_open_until_ms = Some(now_ms + WORKSPACE_FEEDBACK_MS);
            effects.push(Effect::NiriAction("focus-workspace-down".into()));
            sync_mode(&mut next, now_ms);
        }
        AppEvent::Tick { now_ms } => {
            sync_mode(&mut next, now_ms);
        }
    }

    (next, effects)
}

fn sync_mode(state: &mut AppState, now_ms: u64) {
    state.mode = if state.hovered || is_auto_open(state, now_ms) {
        UiMode::Open
    } else {
        UiMode::Compact
    };
}

fn is_auto_open(state: &AppState, now_ms: u64) -> bool {
    state
        .auto_open_until_ms
        .is_some_and(|until_ms| until_ms > now_ms)
}
