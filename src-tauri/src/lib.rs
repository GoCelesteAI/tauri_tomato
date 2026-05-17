use std::sync::Mutex;
use std::time::Instant;
use tauri::{
  menu::{MenuBuilder, MenuItemBuilder},
  tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
  AppHandle, Manager, State,
};
use tauri_plugin_notification::NotificationExt;

const SESSION_SECS: u64 = 25 * 60;

#[derive(Default)]
struct TimerState {
  running: bool,
  started_at: Option<Instant>,
  remaining_secs: u64,
}

impl TimerState {
  fn current_remaining(&self) -> u64 {
    if !self.running {
      return self.remaining_secs;
    }
    let elapsed = self.started_at.map(|t| t.elapsed().as_secs()).unwrap_or(0);
    self.remaining_secs.saturating_sub(elapsed)
  }
}

#[tauri::command]
fn start(state: State<'_, Mutex<TimerState>>) {
  let mut s = state.lock().unwrap();
  if !s.running {
    if s.remaining_secs == 0 {
      s.remaining_secs = SESSION_SECS;
    }
    s.running = true;
    s.started_at = Some(Instant::now());
  }
}

#[tauri::command]
fn pause(state: State<'_, Mutex<TimerState>>) {
  let mut s = state.lock().unwrap();
  if s.running {
    s.remaining_secs = s.current_remaining();
    s.running = false;
    s.started_at = None;
  }
}

#[tauri::command]
fn reset(state: State<'_, Mutex<TimerState>>) {
  let mut s = state.lock().unwrap();
  s.running = false;
  s.started_at = None;
  s.remaining_secs = SESSION_SECS;
}

#[tauri::command]
fn tick(app: AppHandle, state: State<'_, Mutex<TimerState>>) -> (u64, bool) {
  let mut s = state.lock().unwrap();
  let remaining = s.current_remaining();
  let finished = s.running && remaining == 0;
  if finished {
    s.running = false;
    s.started_at = None;
    s.remaining_secs = 0;
    let _ = app
      .notification()
      .builder()
      .title("Pomodoro complete")
      .body("Time for a break.")
      .show();
  }
  (remaining, s.running)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .plugin(tauri_plugin_notification::init())
    .plugin(tauri_plugin_store::Builder::default().build())
    .manage(Mutex::new(TimerState {
      remaining_secs: SESSION_SECS,
      ..Default::default()
    }))
    .invoke_handler(tauri::generate_handler![start, pause, reset, tick])
    .setup(|app| {
      let show = MenuItemBuilder::with_id("show", "Show Tomato").build(app)?;
      let quit = MenuItemBuilder::with_id("quit", "Quit").build(app)?;
      let menu = MenuBuilder::new(app).items(&[&show, &quit]).build()?;

      TrayIconBuilder::with_id("main")
        .menu(&menu)
        .show_menu_on_left_click(false)
        .icon(app.default_window_icon().unwrap().clone())
        .on_menu_event(|app, event| match event.id().as_ref() {
          "show" => toggle_window(app),
          "quit" => app.exit(0),
          _ => {}
        })
        .on_tray_icon_event(|tray, event| {
          if let TrayIconEvent::Click {
            button: MouseButton::Left,
            button_state: MouseButtonState::Up,
            ..
          } = event
          {
            toggle_window(tray.app_handle());
          }
        })
        .build(app)?;
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

fn toggle_window(app: &AppHandle) {
  if let Some(win) = app.get_webview_window("main") {
    if win.is_visible().unwrap_or(false) {
      let _ = win.hide();
    } else {
      let _ = win.show();
      let _ = win.set_focus();
    }
  }
}
