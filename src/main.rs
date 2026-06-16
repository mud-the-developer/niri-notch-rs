use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Box as GtkBox, EventBox, Label, Orientation, gdk, glib};
use gtk_layer_shell::{Edge, Layer, LayerShell};
use niri_notch_rs::cli::{CliCommand, doctor_probe_names, help_text, parse_cli};
use niri_notch_rs::core::{AppEvent, AppState, Effect, UiMode, reduce};
use std::cell::{Cell, RefCell};
use std::io::{self, Write};
use std::process::{Command, Stdio};
use std::rc::Rc;
use std::time::{Duration, Instant};

const APP_ID: &str = "dev.mud.NiriNotch";
const COMPACT_W: i32 = 138;
const COMPACT_H: i32 = 42;
const OPEN_W: i32 = 468;
const OPEN_H: i32 = 86;

#[derive(Debug, Clone)]
struct Snapshot {
    title: String,
    media: String,
    chip: String,
    notification_sig: Option<String>,
    workspaces: Vec<Workspace>,
}

#[derive(Debug, Clone)]
struct Workspace {
    name: String,
    active: bool,
}

fn cmd_out(cmd: &str, args: &[&str]) -> String {
    Command::new(cmd)
        .args(args)
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .unwrap_or_default()
        .trim()
        .to_string()
}

fn focused_title() -> String {
    let out = cmd_out("niri", &["msg", "focused-window"]);
    let mut title = "Desktop".to_string();
    for line in out.lines() {
        let line = line.trim();
        if let Some(rest) = line.strip_prefix("Title: ") {
            title = rest.trim_matches('"').to_string();
            break;
        }
    }
    title = title.replace(" | Hermes - Discord", " · Hermes");
    if title.chars().count() > 46 {
        title = trim(&title, 43);
    }
    if title.trim().is_empty() {
        "Desktop".into()
    } else {
        title
    }
}

fn workspace_state() -> Vec<Workspace> {
    let out = cmd_out("niri", &["msg", "workspaces"]);
    let mut in_primary = false;
    let mut items = Vec::new();

    for raw in out.lines() {
        let line = raw.trim_end();
        if line.starts_with("Output ") {
            in_primary = line.contains("\"DP-2\"");
            continue;
        }
        if !in_primary {
            continue;
        }
        let trimmed = line.trim();
        let active = trimmed.starts_with('*');
        let name = trimmed
            .trim_start_matches('*')
            .split_whitespace()
            .next()
            .unwrap_or_default();
        if !name.is_empty() {
            items.push(Workspace {
                name: name.to_string(),
                active,
            });
        }
    }

    if items.is_empty() {
        items.push(Workspace {
            name: "1".into(),
            active: true,
        });
        items.push(Workspace {
            name: "2".into(),
            active: false,
        });
    }
    items.truncate(5);
    items
}

fn notification_info() -> Option<(String, String, usize)> {
    Command::new("makoctl")
        .args(["list", "-j"])
        .output()
        .ok()
        .and_then(|o| {
            if o.status.success() {
                String::from_utf8(o.stdout).ok()
            } else {
                None
            }
        })
        .and_then(|s| serde_json::from_str::<serde_json::Value>(&s).ok())
        .and_then(|v| {
            let notifications = v.as_array()?;
            let count = notifications.len();
            let n = notifications.first()?.clone();
            let id = n
                .get("id")
                .map(std::string::ToString::to_string)
                .unwrap_or_default();
            let summary = n
                .get("summary")
                .and_then(|s| s.as_str())
                .unwrap_or_default()
                .trim()
                .to_string();
            let body = n
                .get("body")
                .and_then(|s| s.as_str())
                .unwrap_or_default()
                .trim()
                .to_string();
            if summary.is_empty() && body.is_empty() {
                None
            } else {
                let label = if body.is_empty() {
                    summary.clone()
                } else {
                    format!("{summary} — {body}")
                };
                Some((format!("{count}:{id}:{summary}:{body}"), label, count))
            }
        })
}

fn media_status() -> (String, String, Option<String>) {
    if let Some((sig, text, count)) = notification_info() {
        let chip = if count > 1 {
            format!("알림 {count}")
        } else {
            "알림".to_string()
        };
        return (format!("󰍡 {}", trim(&text, 42)), chip, Some(sig));
    }

    let media = Command::new("playerctl")
        .args(["metadata", "--format", "{{artist}} — {{title}}"])
        .output()
        .ok()
        .and_then(|o| {
            if o.status.success() {
                String::from_utf8(o.stdout).ok()
            } else {
                None
            }
        })
        .unwrap_or_default()
        .trim()
        .to_string();
    if !media.is_empty() {
        return (format!("󰎈 {}", trim(&media, 34)), "Media".to_string(), None);
    }
    ("알림 준비됨".to_string(), "Niri".to_string(), None)
}

fn trim(s: &str, max_chars: usize) -> String {
    let mut out = String::new();
    for (i, ch) in s.chars().enumerate() {
        if i >= max_chars {
            out.push('…');
            break;
        }
        out.push(ch);
    }
    out
}

fn load_state() -> Snapshot {
    let (media, chip, notification_sig) = media_status();
    Snapshot {
        title: focused_title(),
        media,
        chip,
        notification_sig,
        workspaces: workspace_state(),
    }
}

const fn css() -> &'static str {
    r#"
window, .notch-root { background: transparent; }
.notch-frame {
  background-image: linear-gradient(to bottom, rgba(255,255,255,0.16), rgba(255,255,255,0.045));
  background-color: rgba(10, 10, 12, 0.88);
  border: 1px solid rgba(255,255,255,0.18);
  border-top-color: rgba(255,255,255,0.32);
  border-radius: 999px;
  box-shadow: 0 18px 50px rgba(0,0,0,0.52), inset 0 1px 0 rgba(255,255,255,0.18);
  padding: 5px 9px;
}
.notch-frame.open {
  border-radius: 26px;
  padding: 8px 11px 9px 11px;
  background-image: linear-gradient(to bottom, rgba(255,255,255,0.18), rgba(255,255,255,0.050));
  background-color: rgba(13, 13, 16, 0.94);
  border-color: rgba(255,255,255,0.20);
  border-top-color: rgba(255,255,255,0.36);
  box-shadow: 0 22px 64px rgba(0,0,0,0.58), 0 0 0 1px rgba(0,209,178,0.08), inset 0 1px 0 rgba(255,255,255,0.20);
}
.top-row, .detail-row { background: transparent; }
.ws-pill {
  min-width: 28px;
  min-height: 25px;
  padding: 0 10px;
  margin: 0 2px;
  border-radius: 999px;
  color: rgba(245,245,247,0.60);
  background: rgba(255,255,255,0.060);
  font: 600 12px "D2CodingLigature Nerd Font Propo";
}
.ws-pill.active {
  color: #fff;
  background: linear-gradient(to bottom, #35a7ff, #0071e3);
  box-shadow: 0 8px 20px rgba(41,151,255,0.26), inset 0 1px 0 rgba(255,255,255,0.25);
}
.info {
  color: rgba(245,245,247,0.92);
  font: 600 12px "D2CodingLigature Nerd Font Propo";
  margin-left: 11px;
}
.subtle {
  color: rgba(245,245,247,0.66);
  font: 600 12px "D2CodingLigature Nerd Font Propo";
  margin-left: 2px;
  margin-top: 7px;
}
.chip {
  color: rgba(245,245,247,0.96);
  background: rgba(41,151,255,0.18);
  border-radius: 999px;
  padding: 4px 10px;
  margin-left: 10px;
  font: 700 11px "D2CodingLigature Nerd Font Propo";
}
"#
}

#[derive(Clone)]
struct Ui {
    window: ApplicationWindow,
    frame: GtkBox,
    title: Label,
    media: Label,
    chip: Label,
    workspace_labels: Rc<Vec<Label>>,
    current_w: Rc<Cell<f64>>,
    current_h: Rc<Cell<f64>>,
    velocity_w: Rc<Cell<f64>>,
    velocity_h: Rc<Cell<f64>>,
    target_w: Rc<Cell<f64>>,
    target_h: Rc<Cell<f64>>,
    app_state: Rc<RefCell<AppState>>,
    started_at: Rc<Instant>,
}

impl Ui {
    fn now_ms(&self) -> u64 {
        self.started_at
            .elapsed()
            .as_millis()
            .try_into()
            .unwrap_or(u64::MAX)
    }

    fn reduce_event(&self, event: AppEvent) {
        let (next, effects) = {
            let current = self.app_state.borrow();
            reduce(&current, event)
        };
        *self.app_state.borrow_mut() = next;
        self.render_state();
        Self::run_effects(effects);
    }

    fn render_state(&self) {
        let state = self.app_state.borrow();
        self.set_open(state.mode == UiMode::Open);
        self.title.set_text(&format!("󰣆 {}", state.title));
        self.media.set_text(&state.media);
        self.chip.set_text(&state.chip);
        self.update_workspaces(&state);
    }

    fn run_effects(effects: Vec<Effect>) {
        for effect in effects {
            match effect {
                Effect::DismissMakoAll { delay_ms } => {
                    glib::timeout_add_local_once(Duration::from_millis(delay_ms), || {
                        let _ = Command::new("makoctl")
                            .arg("dismiss")
                            .arg("--all")
                            .arg("--no-history")
                            .output();
                    });
                }
                Effect::PlayerctlPlayPause => {
                    let _ = Command::new("playerctl").arg("play-pause").output();
                }
                Effect::SpawnControlCenter => {
                    let _ = Command::new("/home/mud/.local/bin/niri-control-center").spawn();
                }
                Effect::NiriAction(action) => {
                    let _ = Command::new("niri")
                        .args(["msg", "action", action.as_str()])
                        .output();
                }
            }
        }
    }

    fn set_open(&self, open: bool) {
        let ctx = self.frame.style_context();
        if open {
            ctx.add_class("open");
            self.title.show();
            self.media.show();
            self.chip.show();
            self.target_w.set(f64::from(OPEN_W));
            self.target_h.set(f64::from(OPEN_H));
        } else {
            ctx.remove_class("open");
            self.title.hide();
            self.media.hide();
            self.chip.hide();
            self.target_w.set(f64::from(COMPACT_W));
            self.target_h.set(f64::from(COMPACT_H));
        }
    }

    fn animate_tick(&self) -> glib::ControlFlow {
        self.reduce_event(AppEvent::Tick {
            now_ms: self.now_ms(),
        });
        let (nw, vw) = spring_step(
            self.current_w.get(),
            self.velocity_w.get(),
            self.target_w.get(),
        );
        let (nh, vh) = spring_step(
            self.current_h.get(),
            self.velocity_h.get(),
            self.target_h.get(),
        );
        self.current_w.set(nw);
        self.current_h.set(nh);
        self.velocity_w.set(vw);
        self.velocity_h.set(vh);

        let width = round_layer_size(nw);
        let height = round_layer_size(nh);
        self.window.set_size_request(width, height);
        self.frame.set_size_request(width, height);
        glib::ControlFlow::Continue
    }

    fn refresh_state(&self) -> glib::ControlFlow {
        let s = load_state();
        let active_workspace = s.workspaces.iter().position(|ws| ws.active).unwrap_or(0);
        let workspaces = s
            .workspaces
            .iter()
            .map(|ws| ws.name.clone())
            .collect::<Vec<_>>();
        self.reduce_event(AppEvent::SnapshotUpdated {
            title: s.title,
            media: s.media,
            chip: s.chip,
            notification_sig: s.notification_sig,
            workspaces,
            active_workspace,
            now_ms: self.now_ms(),
        });
        glib::ControlFlow::Continue
    }

    fn update_workspaces(&self, state: &AppState) {
        let visible: Vec<(String, bool)> = if state.mode == UiMode::Open {
            state
                .workspaces
                .iter()
                .enumerate()
                .map(|(idx, name)| (name.clone(), idx == state.active_workspace))
                .collect()
        } else {
            let active_idx = state
                .active_workspace
                .min(state.workspaces.len().saturating_sub(1));
            let mut compact = Vec::new();
            if let Some(active) = state.workspaces.get(active_idx) {
                compact.push((active.clone(), true));
            }
            let companion_idx = if active_idx + 1 < state.workspaces.len() {
                active_idx + 1
            } else {
                active_idx.saturating_sub(1)
            };
            if let Some(companion) = state
                .workspaces
                .get(companion_idx)
                .filter(|_| companion_idx != active_idx)
            {
                compact.push((companion.clone(), false));
            }
            compact
        };

        for (idx, label) in self.workspace_labels.iter().enumerate() {
            if let Some((name, active)) = visible.get(idx) {
                label.set_text(name);
                label.show();
                let ctx = label.style_context();
                if *active {
                    ctx.add_class("active");
                } else {
                    ctx.remove_class("active");
                }
            } else {
                label.hide();
            }
        }
    }
}

#[allow(clippy::cast_possible_truncation)]
fn round_layer_size(value: f64) -> i32 {
    value
        .round()
        .clamp(f64::from(i32::MIN), f64::from(i32::MAX)) as i32
}

fn spring_step(current: f64, velocity: f64, target: f64) -> (f64, f64) {
    // Slightly slower, Apple-like damped response: soft acceleration, no snappy jump.
    let delta = target - current;
    let next_velocity = delta.mul_add(0.052, velocity * 0.74);
    let next = current + next_velocity;
    if delta.abs() < 0.35 && next_velocity.abs() < 0.35 {
        (target, 0.0)
    } else {
        (next, next_velocity)
    }
}

#[allow(clippy::too_many_lines)]
fn build_ui(app: &Application) {
    let provider = gtk::CssProvider::new();
    if provider.load_from_data(css().as_bytes()).is_err() {
        return;
    }
    let Some(screen) = gdk::Screen::default() else {
        return;
    };
    gtk::StyleContext::add_provider_for_screen(
        &screen,
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    let window = ApplicationWindow::new(app);
    window.set_title("Niri Notch RS");
    window.set_app_paintable(true);
    window.set_decorated(false);
    window.set_resizable(false);
    window.set_accept_focus(false);
    window.style_context().add_class("notch-root");

    window.init_layer_shell();
    window.set_namespace("niri-notch-rs");
    window.set_layer(Layer::Overlay);
    window.set_anchor(Edge::Top, true);
    // Niri positions overlay layer-shell surfaces below Waybar's exclusive zone.
    // Compensate for Waybar's 49px bar so this stays on the same visual row.
    window.set_layer_shell_margin(Edge::Top, -49);
    window.set_keyboard_interactivity(false);
    window.set_exclusive_zone(0);

    let event = EventBox::new();
    event.set_visible_window(false);
    event.set_size_request(COMPACT_W, COMPACT_H);
    window.add(&event);

    let frame = GtkBox::new(Orientation::Vertical, 0);
    frame.style_context().add_class("notch-frame");
    frame.set_halign(gtk::Align::Center);
    frame.set_valign(gtk::Align::Start);
    frame.set_size_request(COMPACT_W, COMPACT_H);
    event.add(&frame);

    let top_row = GtkBox::new(Orientation::Horizontal, 0);
    top_row.style_context().add_class("top-row");
    top_row.set_halign(gtk::Align::Center);
    frame.pack_start(&top_row, false, false, 0);

    let mut workspace_labels = Vec::new();
    for idx in 0..5 {
        let l = Label::new(Some(&(idx + 1).to_string()));
        l.style_context().add_class("ws-pill");
        top_row.pack_start(&l, false, false, 0);
        workspace_labels.push(l);
    }

    let title = Label::new(Some("󰣆 Desktop"));
    title.style_context().add_class("info");
    title.set_max_width_chars(31);
    title.set_ellipsize(gtk::pango::EllipsizeMode::End);
    top_row.pack_start(&title, false, false, 0);

    let chip = Label::new(Some("Rust"));
    chip.style_context().add_class("chip");
    top_row.pack_start(&chip, false, false, 0);

    let detail_row = GtkBox::new(Orientation::Horizontal, 0);
    detail_row.style_context().add_class("detail-row");
    detail_row.set_halign(gtk::Align::Center);
    frame.pack_start(&detail_row, false, false, 0);

    let media = Label::new(Some("알림 준비됨"));
    media.style_context().add_class("subtle");
    media.set_max_width_chars(40);
    media.set_ellipsize(gtk::pango::EllipsizeMode::End);
    detail_row.pack_start(&media, false, false, 0);

    let ui = Ui {
        window: window.clone(),
        frame,
        title,
        media,
        chip,
        workspace_labels: Rc::new(workspace_labels),
        current_w: Rc::new(Cell::new(f64::from(COMPACT_W))),
        current_h: Rc::new(Cell::new(f64::from(COMPACT_H))),
        velocity_w: Rc::new(Cell::new(0.0)),
        velocity_h: Rc::new(Cell::new(0.0)),
        target_w: Rc::new(Cell::new(f64::from(COMPACT_W))),
        target_h: Rc::new(Cell::new(f64::from(COMPACT_H))),
        app_state: Rc::new(RefCell::new(AppState::default())),
        started_at: Rc::new(Instant::now()),
    };

    window.set_size_request(COMPACT_W, COMPACT_H);
    event.add_events(gdk::EventMask::BUTTON_PRESS_MASK | gdk::EventMask::SCROLL_MASK);

    let ui_click = ui.clone();
    event.connect_button_press_event(move |_, ev| {
        match ev.button() {
            1 => ui_click.reduce_event(AppEvent::PrimaryClick {
                now_ms: ui_click.now_ms(),
            }),
            3 => ui_click.reduce_event(AppEvent::SecondaryClick {
                now_ms: ui_click.now_ms(),
            }),
            _ => {}
        }
        glib::Propagation::Proceed
    });

    let ui_scroll = ui.clone();
    event.connect_scroll_event(move |_, ev| {
        match ev.direction() {
            gdk::ScrollDirection::Up => ui_scroll.reduce_event(AppEvent::ScrollUp {
                now_ms: ui_scroll.now_ms(),
            }),
            gdk::ScrollDirection::Down => ui_scroll.reduce_event(AppEvent::ScrollDown {
                now_ms: ui_scroll.now_ms(),
            }),
            _ => {}
        }
        glib::Propagation::Proceed
    });

    let ui_enter = ui.clone();
    event.connect_enter_notify_event(move |_, _| {
        ui_enter.reduce_event(AppEvent::HoverChanged(true));
        ui_enter.refresh_state();
        glib::Propagation::Proceed
    });
    let ui_leave = ui.clone();
    event.connect_leave_notify_event(move |_, _| {
        ui_leave.reduce_event(AppEvent::HoverChanged(false));
        ui_leave.refresh_state();
        glib::Propagation::Proceed
    });

    let demo_open = std::env::var("NIRI_NOTCH_OPEN").ok().as_deref() == Some("1");
    if demo_open {
        ui.reduce_event(AppEvent::HoverChanged(true));
    }
    ui.refresh_state();
    let tick_ui = ui.clone();
    glib::timeout_add_local(Duration::from_millis(16), move || tick_ui.animate_tick());
    let refresh_ui = ui;
    glib::timeout_add_local(Duration::from_millis(500), move || {
        refresh_ui.refresh_state()
    });

    window.show_all();
}

fn main() {
    match parse_cli(std::env::args_os().map(|value| value.to_string_lossy().into_owned())) {
        Ok(Some(command)) => {
            run_cli(&command);
        }
        Ok(None) => run_gui(),
        Err(error) => {
            write_stderr(&format!("{error:?}\n\n{}", help_text()));
            std::process::exit(2);
        }
    }
}

fn run_gui() {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_ui);
    app.run();
}

fn write_stdout(message: &str) {
    let _ = io::stdout().write_all(message.as_bytes());
}

fn write_stderr(message: &str) {
    let _ = io::stderr().write_all(message.as_bytes());
}

fn run_cli(command: &CliCommand) {
    match command {
        CliCommand::Doctor => run_doctor(),
        CliCommand::Install { user, dry_run } => {
            write_stdout(&format!("install plan: user={user} dry_run={dry_run}\n"));
            write_stdout("managed spawn: ~/.local/bin/niri-notch-rs\n");
        }
        CliCommand::Uninstall { user, dry_run } => {
            write_stdout(&format!("uninstall plan: user={user} dry_run={dry_run}\n"));
            write_stdout("remove managed spawn: ~/.local/bin/niri-notch-rs\n");
        }
        CliCommand::Preview => {
            write_stdout("preview: run with NIRI_NOTCH_OPEN=1 niri-notch-rs\n");
        }
        CliCommand::Help => {
            write_stdout(help_text());
        }
    }
}

fn run_doctor() {
    write_stdout("niri-notch-rs doctor\n");
    for probe in doctor_probe_names() {
        let ok = match probe {
            "niri" => command_exists("niri"),
            "gtk-layer-shell-0" => pkg_config_exists("gtk-layer-shell-0"),
            "waybar" => command_exists("waybar"),
            "makoctl" => command_exists("makoctl"),
            "playerctl" => command_exists("playerctl"),
            "niri config" => Command::new("niri")
                .args(["validate", "-c"])
                .arg(expand_home("~/.config/niri/config.kdl"))
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .status()
                .is_ok_and(|status| status.success()),
            "niri layers" => Command::new("niri")
                .args(["msg", "layers"])
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .status()
                .is_ok_and(|status| status.success()),
            _ => false,
        };
        write_stdout(&format!(
            "{probe:<18} {}\n",
            if ok { "ok" } else { "missing" }
        ));
    }
}

fn command_exists(command: &str) -> bool {
    Command::new("sh")
        .args(["-c", "command -v \"$1\" >/dev/null 2>&1", "sh", command])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .is_ok_and(|status| status.success())
}

fn pkg_config_exists(package: &str) -> bool {
    Command::new("pkg-config")
        .args(["--exists", package])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .is_ok_and(|status| status.success())
}

fn expand_home(path: &str) -> String {
    path.strip_prefix("~/").map_or_else(
        || path.to_string(),
        |rest| {
            std::env::var("HOME").map_or_else(|_| path.to_string(), |home| format!("{home}/{rest}"))
        },
    )
}
