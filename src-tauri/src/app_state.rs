//! État partagé de l'application, exposé aux commandes Tauri via
//! `tauri::State` et initialisé une fois au démarrage (`app.manage(...)`).
//!
//! Note : `OsHotkeyBackend` (donc `GlobalHotKeyManager`) n'est **pas**
//! stocké ici. Sous Windows, ce type embarque un handle natif (`*mut
//! c_void`, l'HWND de la fenêtre-message cachée qui reçoit les
//! `WM_HOTKEY`) qui n'implémente pas `Send`. Tauri exige `T: Send + Sync`
//! pour tout ce qui passe par `app.manage(...)`/`tauri::State` — l'y
//! placer casse la compilation de toutes les commandes. Voir
//! `lib.rs::setup_global_hotkey` pour l'endroit où il vit réellement (sur
//! le thread principal, via `Box::leak`).

use std::sync::Mutex;

use crate::audio::AudioCapture;
use crate::hotkey::HotkeyRegistry;

pub struct AppState {
    pub audio: Mutex<AudioCapture>,
    pub hotkeys: Mutex<HotkeyRegistry>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            audio: Mutex::new(AudioCapture::new()),
            hotkeys: Mutex::new(HotkeyRegistry::new()),
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
