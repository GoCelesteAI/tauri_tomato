# Tomato — Tauri Patterns Ep 10 (Capstone)

Demo app for **Episode 10: Build a Menu Bar Timer in Tauri 2 — System Tray Plugin Capstone** of the [Tauri Patterns for Production](https://www.youtube.com/playlist?list=PLOeWRYj1QznVJfg6w0_l8M5WUXP7Nf32x) series by Codegiz.

A Pomodoro timer that lives in the macOS menu bar (Windows/Linux system tray). Click the tray icon, a 320×360 panel slides in with a 25:00 countdown and Start/Pause/Reset buttons. Click it again, the panel hides — the timer keeps ticking in the background. When the session ends, an OS notification fires and a daily-completions counter is persisted to disk.

This is the **capstone** of the series — every prior episode shows up somewhere: window flags + capabilities (Ep 1), plugin-notification (Ep 3), `WebviewWindow` show/hide (Ep 4), plugin-store (Ep 5), `Mutex<AppState>` (Ep 7), and the CI matrix from Ep 9 still ships this app to three platforms without changes.

- **Watch on YouTube:** https://www.youtube.com/watch?v=AffSwDFz-g0
- **Read on Codegiz:** https://codegiz.com/blog/tauri-patterns-episode-10-build-a-menu-bar-timer-in-tauri-2
- **Series index:** https://github.com/GoCelesteAI/tauri-patterns

## What this app shows

```
tomato/
├── src/
│   ├── App.tsx              ← setInterval(invoke("tick"), 500); plugin-store counter
│   └── main.tsx
└── src-tauri/
    ├── Cargo.toml           ← tauri = { features = ["tray-icon"] }, plugin-notification, plugin-store
    ├── tauri.conf.json      ← visible:false, decorations:false, transparent:true,
    │                          alwaysOnTop:true, skipTaskbar:true  ← the five menu-bar flags
    ├── capabilities/
    │   └── default.json     ← core:window:allow-show, allow-hide, allow-set-focus,
    │                          notification:default, store:default
    └── src/
        └── lib.rs           ← TimerState (Mutex), 4 commands, TrayIconBuilder in setup()
```

## Run it

```sh
pnpm install
pnpm tauri dev          # development
pnpm tauri build        # production bundle (.app + .dmg on macOS)
```

On launch you'll see no dock icon and no main window — just a tray icon in the menu bar. Click it to toggle the popup.

## Episode topics

- The five window flags that turn a Tauri window into a menu-bar popup.
- `TrayIconBuilder` in the `setup` hook — registering the tray icon, attaching a right-click menu, handling left-click events.
- `.show_menu_on_left_click(false)` — left-click toggles your popup, right-click opens the menu. Standard menu-bar UX.
- `Mutex<TimerState>` — three coupled fields (`running`, `started_at`, `remaining_secs`) guarded by one Mutex, the same pattern from Ep 7.
- Polling via `setInterval` + `invoke("tick")` — simpler than events for a clock that updates twice a second.
- `app.notification().builder().title(…).body(…).show()` — fire-and-forget OS notification when the session ends.
- `tauri-plugin-store` for the daily counter — auto-save, JSON file on disk, survives quits and relaunches.

## About this channel

The Codegiz channel is run by **Claude AI**. Tutorials are AI-produced; reviewed and published by Codegiz. Source for every series at github.com/GoCelesteAI.

## License

MIT
