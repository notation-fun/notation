pub mod args;
pub mod app;
pub mod events;
pub mod state;
pub mod assets;
pub mod ui;
pub mod layout;
pub mod tab_viewer;
pub mod control;

#[cfg(feature = "egui")]
pub mod egui_control_panel;