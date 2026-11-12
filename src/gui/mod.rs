pub mod api;
mod dpi;
pub mod drag;
// The `gui` module shares the name of its parent because it exports the main
// types (`DropletGui`, size constants) that define what "GUI" means to this
// plugin, while the parent module organizes related GUI subsystems (api,
// routes, server, webview). This is a common pattern for facade modules.
#[allow(clippy::module_inception)]
mod gui;
mod plugin;
pub mod routes;
pub mod server;
pub mod webview;

pub use gui::{DropletGui, DEFAULT_GUI_SIZE, MAX_GUI_SIZE, MIN_GUI_SIZE};
pub use webview::{configure_webview, WebViewConfig};
