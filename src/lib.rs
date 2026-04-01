mod application;
mod clipboard;
mod conversion;
mod error;
pub(crate) mod output_subscription;
mod settings;
mod state;
mod task;

// --- Our types ---
pub use application::application;
pub use error::Error;
pub use output_subscription::output_events;
pub use settings::{
    Anchor, KeyboardInteractivity, Layer, LayerShellSettings, OutputEvent, OutputId, OutputInfo,
    SurfaceId,
};
pub use task::{
    clipboard_write, destroy_layer_surface, new_layer_surface, set_anchor, set_exclusive_zone,
    set_keyboard_interactivity, set_layer, set_margin, set_size, Task,
};

// --- Re-exports from iced ---
pub use iced_core::{
    alignment, color, font, gradient, padding, Alignment, Border, Color, Font, Length, Padding,
    Pixels, Point, Rectangle, Shadow, Size, Theme,
};
pub use iced_core::{event::Event, keyboard, mouse, touch, window};
pub use iced_futures::Subscription;
pub use iced_renderer::Renderer;
pub use iced_widget::*;
pub mod widget {
    pub use iced_widget::*;
}

/// A generic iced UI element.
pub type Element<'a, Message> = iced_core::Element<'a, Message, Theme, Renderer>;
