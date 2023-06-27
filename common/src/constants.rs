#![allow(unused)]
use std::env;
use std::path::{Path, PathBuf};
use std::time::Duration;

use embedded_graphics::prelude::Size;
use lazy_static::lazy_static;

pub const ALLIUM_VERSION: &str = env!("CARGO_PKG_VERSION");

lazy_static! {
    pub static ref ALLIUM_SD_ROOT: PathBuf = PathBuf::from(
        &env::var("ALLIUM_SD_ROOT").unwrap_or_else(|_| "/mnt/SDCARD/".to_string())
    );
    pub static ref ALLIUM_BASE_DIR: PathBuf = PathBuf::from(
        &env::var("ALLIUM_BASE_DIR").unwrap_or_else(|_| "/mnt/SDCARD/.allium".to_string())
    );
    pub static ref ALLIUM_GAMES_DIR: PathBuf = PathBuf::from(
        &env::var("ALLIUM_GAMES_DIR").unwrap_or_else(|_| "/mnt/SDCARD/Roms".to_string())
    );

    // Folders
    pub static ref ALLIUM_SCRIPTS_DIR: PathBuf = ALLIUM_BASE_DIR.join("scripts");
    pub static ref ALLIUM_TOOLS_DIR: PathBuf = ALLIUM_BASE_DIR.join("tools");
    pub static ref ALLIUM_FONTS_DIR: PathBuf = ALLIUM_BASE_DIR.join("fonts");
    pub static ref ALLIUM_LOCALES_DIR: PathBuf = ALLIUM_BASE_DIR.join("locales");

    // Config
    pub static ref ALLIUM_CONFIG_CONSOLES: PathBuf = ALLIUM_BASE_DIR.join("config/consoles.toml");

    // State
    pub static ref ALLIUMD_STATE: PathBuf = ALLIUM_BASE_DIR.join("state/alliumd.json");
    pub static ref ALLIUM_DATABASE: PathBuf = ALLIUM_SD_ROOT.join("Saves/CurrentProfile/allium.db");
    pub static ref ALLIUM_LAUNCHER_STATE: PathBuf =
        ALLIUM_BASE_DIR.join("state/allium-launcher.json");
    pub static ref ALLIUM_MENU_STATE: PathBuf =
        ALLIUM_BASE_DIR.join("state/allium-menu.json");
    pub static ref ALLIUM_GAME_INFO: PathBuf = ALLIUM_BASE_DIR.join("state/current_game");
    pub static ref ALLIUM_STYLESHEET: PathBuf = ALLIUM_BASE_DIR.join("state/stylesheet.json");
    pub static ref ALLIUM_DISPLAY_SETTINGS: PathBuf = ALLIUM_BASE_DIR.join("state/display.json");
    pub static ref ALLIUM_LOCALE_SETTINGS: PathBuf = ALLIUM_BASE_DIR.join("state/locale.json");
    pub static ref ALLIUM_WIFI_SETTINGS: PathBuf = ALLIUM_BASE_DIR.join("state/wifi.json");

    // Binaries & Scripts
    pub static ref ALLIUM_LAUNCHER: PathBuf = env::var("ALLIUM_LAUNCHER")
        .map(PathBuf::from)
        .unwrap_or_else(|_| ALLIUM_BASE_DIR.join("allium-launcher"));
    pub static ref ALLIUM_MENU: PathBuf = env::var("ALLIUM_MENU")
        .map(PathBuf::from)
        .unwrap_or_else(|_| ALLIUM_BASE_DIR.join("allium-menu"));
    pub static ref ALLIUM_RETROARCH: PathBuf = env::var("ALLIUM_RETROARCH")
        .map(PathBuf::from)
        .unwrap_or_else(|_| ALLIUM_BASE_DIR.join("cores/retroarch/launch.sh"));
}

pub const AUTO_SLEEP_TIMEOUT: Duration = Duration::from_secs(5 * 60);
pub const BATTERY_SHUTDOWN_THRESHOLD: i32 = 5;
pub const BATTERY_UPDATE_INTERVAL: Duration = Duration::from_secs(10);
pub const BUTTON_DIAMETER: u32 = 31;
pub const CLOCK_UPDATE_INTERVAL: Duration = Duration::from_secs(1);
pub const IMAGE_SIZE: Size = Size::new(250, 376);
pub const LISTING_JUMP_SIZE: i32 = 5;
pub const LISTING_SIZE: i32 = 10;
pub const MAXIMUM_FRAME_TIME: Duration = Duration::from_millis(100);
pub const RECENT_GAMES_LIMIT: i64 = 100;
pub const RETROARCH_UDP_SOCKET: &str = "127.0.0.1:55355";
pub const SELECTION_MARGIN: u32 = 8;
