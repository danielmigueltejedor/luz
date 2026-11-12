//! Software updates: Help ▸ Check for Updates, the check at start, and their windows.
//!
//! The flow and the windows are `nodalia-update`'s, shared with the other Nodalia apps; this file
//! is what is this app's own: when an update may close it, and where its keys go.

use serde_json::Value;

use crate::LightcraftApp;

#[cfg(not(target_arch = "wasm32"))]
use nodalia_update::egui::{Action, Updater};

pub(crate) struct Runtime {
    #[cfg(not(target_arch = "wasm32"))]
    updater: Updater,
    /// The desktop app turns the check at start on; tests and the web build leave it off.
    #[cfg(not(target_arch = "wasm32"))]
    startup_check: bool,
    #[cfg(not(target_arch = "wasm32"))]
    check_requested: bool,
    /// The context of the last frame, to wake the UI when a check is queued from a menu callback.
    #[cfg(not(target_arch = "wasm32"))]
    repaint: Option<egui::Context>,
    /// A requested check's result dialog is showing.
    #[cfg(target_arch = "wasm32")]
    pub(crate) manual_window_open: bool,
    #[cfg(target_arch = "wasm32")]
    pub(crate) status_message: Option<String>,
}

impl Default for Runtime {
    fn default() -> Self {
        Self {
            #[cfg(not(target_arch = "wasm32"))]
            updater: Updater::new(crate::UPDATE_APP).with_accent([0xf2, 0xa5, 0x16]),
            #[cfg(not(target_arch = "wasm32"))]
            startup_check: false,
            #[cfg(not(target_arch = "wasm32"))]
            check_requested: false,
            #[cfg(not(target_arch = "wasm32"))]
            repaint: None,
            #[cfg(target_arch = "wasm32")]
            manual_window_open: false,
            #[cfg(target_arch = "wasm32")]
            status_message: None,
        }
    }
}

impl LightcraftApp {
    /// Whether the app looks for a newer release when it starts (off until the desktop app turns it on).
    #[cfg(not(target_arch = "wasm32"))]
    pub fn set_startup_update_check(&mut self, on: bool) {
        self.updates_rt.startup_check = on;
    }
}

/// The update windows are modal while they are open: shortcuts must not reach the library behind them.
pub(crate) fn blocks_input(app: &LightcraftApp) -> bool {
    #[cfg(not(target_arch = "wasm32"))]
    {
        app.updates_rt.updater.is_open()
    }
    #[cfg(target_arch = "wasm32")]
    {
        let _ = app;
        false
    }
}

pub(crate) fn check(app: &mut LightcraftApp) -> Result<Value, String> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        app.updates_rt.check_requested = true;
        if let Some(ctx) = &app.updates_rt.repaint {
            ctx.request_repaint();
        }
        Ok(Value::Null)
    }
    #[cfg(target_arch = "wasm32")]
    {
        app.updates_rt.manual_window_open = true;
        app.updates_rt.status_message = Some("Update check is not supported in the web browser.".into());
        Ok(Value::Null)
    }
}

pub(crate) fn tick(app: &mut LightcraftApp, ctx: &egui::Context) {
    #[cfg(not(target_arch = "wasm32"))]
    {
        let rt = &mut app.updates_rt;
        rt.repaint = Some(ctx.clone());
        if std::mem::take(&mut rt.check_requested) {
            rt.updater.check_now(ctx);
        } else if rt.startup_check {
            rt.updater.check_at_start(ctx);
        }
        // What the update threads sent, so `blocks_input` is current before the shortcuts run.
        let action = rt.updater.poll();
        act(ctx, action);
    }
    #[cfg(target_arch = "wasm32")]
    let _ = (app, ctx);
}

/// Carries out what the update windows asked for.
#[cfg(not(target_arch = "wasm32"))]
fn act(ctx: &egui::Context, action: Action) {
    match action {
        Action::OpenUrl(url) => ctx.open_url(egui::OpenUrl::new_tab(url)),
        Action::Exit => {
            // The new version is starting, or a helper waits for this process to end. Close
            // normally so the catalog and preferences are saved; the install only ran once
            // nothing was unsaved. Exit anyway if the app has not ended soon.
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            std::thread::spawn(|| {
                std::thread::sleep(std::time::Duration::from_secs(5));
                std::process::exit(0);
            });
        }
        Action::None => {}
    }
}

pub(crate) fn show(app: &mut LightcraftApp, ctx: &egui::Context) {
    #[cfg(not(target_arch = "wasm32"))]
    {
        // An installation waits until nothing is unsaved.
        let clean = app.session.unsaved().is_none();
        let language = nodalia_update::Language::from_code(crate::i18n::language().code());
        let action = app.updates_rt.updater.show(ctx, language, clean);
        act(ctx, action);
    }

    #[cfg(target_arch = "wasm32")]
    if app.updates_rt.manual_window_open {
        let mut open = true;
        let shown = egui::Window::new(crate::i18n::tr("LightCraft Updates"))
            .open(&mut open)
            .collapsible(false)
            .resizable(false)
            .pivot(egui::Align2::CENTER_CENTER)
            .default_pos(ctx.content_rect().center())
            .show(ctx, |ui| {
                if let Some(msg) = &app.updates_rt.status_message {
                    ui.label(msg);
                }
                ui.add_space(8.0);
                if ui.button(crate::i18n::tr("OK")).clicked() {
                    app.updates_rt.manual_window_open = false;
                }
            });
        if let Some(w) = &shown {
            crate::widgets::announce_dialog(ctx, &w.response, crate::widgets::Modal::Window);
        }
        if !open {
            app.updates_rt.manual_window_open = false;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lightcraft_engine::Session;
    #[cfg(not(target_arch = "wasm32"))]
    use nodalia_update::egui::Status;

    #[test]
    fn default_runtime_is_idle() {
        let rt = Runtime::default();
        #[cfg(target_arch = "wasm32")]
        assert!(!rt.manual_window_open);
        #[cfg(not(target_arch = "wasm32"))]
        {
            assert!(!rt.startup_check);
            assert!(!rt.updater.is_open());
            assert_eq!(rt.updater.status(), &Status::Idle);
        }
    }

    #[test]
    #[cfg(not(target_arch = "wasm32"))]
    fn the_releases_are_this_apps_own() {
        assert_eq!(crate::UPDATE_APP.repo, "getnodalia/luz");
        assert_eq!(crate::UPDATE_APP.version, env!("CARGO_PKG_VERSION"));
    }

    #[test]
    #[cfg(not(target_arch = "wasm32"))]
    fn startup_prompt_shows_holds_the_keys_and_closes() {
        let mut app = LightcraftApp::new(Session::new(), Default::default());
        let ctx = egui::Context::default();
        app.updates_rt.updater = Updater::new(crate::UPDATE_APP).with_stamp(None);
        app.updates_rt.updater.offer(nodalia_update::Release {
            tag: "v99.0.0".into(),
            version: "99.0.0".into(),
            notes: String::new(),
            page: "https://github.com/getnodalia/luz/releases/tag/v99.0.0".into(),
            asset: None,
        });
        let frame = |app: &mut LightcraftApp, events: Vec<egui::Event>| {
            let mut output = ctx.run_ui(egui::RawInput { events, ..Default::default() }, |ui| {
                show(app, ui.ctx());
            });
            output.textures_delta.clear();
        };
        frame(&mut app, vec![]);
        assert!(blocks_input(&app));
        let escape = egui::Event::Key { key: egui::Key::Escape, physical_key: None, pressed: true, repeat: false, modifiers: egui::Modifiers::NONE };
        frame(&mut app, vec![escape]);
        assert!(!blocks_input(&app));
    }

    #[test]
    #[cfg(not(target_arch = "wasm32"))]
    fn a_silent_check_at_start_holds_no_keys() {
        let mut app = LightcraftApp::new(Session::new(), Default::default());
        app.updates_rt.updater.set_status(Status::Checking { user: false });
        assert!(!blocks_input(&app));
        app.updates_rt.updater.set_status(Status::Checking { user: true });
        assert!(blocks_input(&app));
    }
}
