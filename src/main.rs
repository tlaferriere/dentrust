#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::{HardwareAcceleration, Theme};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use tokio;
use tokio::runtime::Runtime;

#[tokio::main]
async fn backend() {}

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Log to stdout (if you run with `RUST_LOG=debug`).
    tracing_subscriber::fmt::init();

    let (chan_back, chan_front) = dentrust::app::app_channels();

    let rt = Runtime::new()?;
    let spawner = rt.handle().clone();

    let native_options = eframe::NativeOptions {
        // Just copied these settings from the web, feel free to change and play around to get live results
        always_on_top: false,
        maximized: false,
        decorated: true,
        fullscreen: false,
        drag_and_drop_support: true,
        icon_data: None,
        initial_window_pos: None,
        initial_window_size: None,
        min_window_size: None,
        max_window_size: None,
        resizable: true,
        transparent: true,
        vsync: true,
        multisampling: 0,
        depth_buffer: 0,
        stencil_buffer: 0,
        hardware_acceleration: HardwareAcceleration::Required,
        renderer: Default::default(),
        follow_system_theme: true,
        default_theme: Theme::Light,
        run_and_return: false,
    };
    eframe::run_native(
        "DenTrust",
        native_options,
        Box::new(|cc| Box::new(dentrust::DentrustApp::new(cc, spawner))),
    );
    Ok(())
}

// when compiling to web using trunk.
#[cfg(target_arch = "wasm32")]
#[tokio::main]
async fn main() {
    // Make sure panics are logged using `console.error`.
    console_error_panic_hook::set_once();

    // Redirect tracing to console.log and friends:
    tracing_wasm::set_as_global_default();

    let web_options = eframe::WebOptions::default();
    eframe::start_web(
        "the_canvas_id", // hardcode it
        web_options,
        Box::new(|cc| Box::new(eframe_template::TemplateApp::new(cc))),
    )
    .expect("failed to start eframe");
}
