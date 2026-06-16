use niri_notch_rs::core::{AppEvent, AppState, Effect, UiMode, reduce};

#[test]
fn hover_enter_opens_without_effects() {
    let state = AppState::default();

    let (next, effects) = reduce(&state, AppEvent::HoverChanged(true));

    assert_eq!(next.mode, UiMode::Open);
    assert!(next.hovered);
    assert!(effects.is_empty());
}

#[test]
fn new_notification_caches_text_opens_and_requests_delayed_mako_dismiss() {
    let state = AppState::default();

    let (next, effects) = reduce(
        &state,
        AppEvent::SnapshotUpdated {
            title: "Firefox".into(),
            media: "󰍡 Build finished".into(),
            chip: "알림 3".into(),
            notification_sig: Some("3:42:Build finished".into()),
            workspaces: vec!["1".into(), "2".into()],
            active_workspace: 0,
            now_ms: 10_000,
        },
    );

    assert_eq!(next.mode, UiMode::Open);
    assert_eq!(
        next.cached_notification_text.as_deref(),
        Some("󰍡 Build finished")
    );
    assert_eq!(next.cached_notification_chip.as_deref(), Some("알림 3"));
    assert_eq!(next.auto_open_until_ms, Some(14_200));
    assert_eq!(effects, vec![Effect::DismissMakoAll { delay_ms: 650 }]);
}

#[test]
fn left_click_without_notification_toggles_media_and_temporarily_opens() {
    let state = AppState::default();

    let (next, effects) = reduce(&state, AppEvent::PrimaryClick { now_ms: 20_000 });

    assert_eq!(next.mode, UiMode::Open);
    assert_eq!(next.media, "󰎈 미디어 토글");
    assert_eq!(next.chip, "Media");
    assert_eq!(next.auto_open_until_ms, Some(20_950));
    assert_eq!(effects, vec![Effect::PlayerctlPlayPause]);
}

#[test]
fn right_click_opens_control_center_as_effect_only() {
    let state = AppState::default();

    let (next, effects) = reduce(&state, AppEvent::SecondaryClick { now_ms: 30_000 });

    assert_eq!(next.mode, UiMode::Open);
    assert_eq!(next.media, "󰀻 Control Center");
    assert_eq!(next.chip, "Menu");
    assert_eq!(effects, vec![Effect::SpawnControlCenter]);
}

#[test]
fn scroll_down_emits_workspace_effect() {
    let state = AppState::default();

    let (_next, effects) = reduce(&state, AppEvent::ScrollDown { now_ms: 40_000 });

    assert_eq!(
        effects,
        vec![Effect::NiriAction("focus-workspace-down".into())]
    );
}
