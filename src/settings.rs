use smithay_client_toolkit::shell::wlr_layer;
use std::fmt;
use std::ops::BitOr;
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_SURFACE_ID: AtomicU64 = AtomicU64::new(1);

/// Unique identifier for a layer shell surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SurfaceId(u64);

impl SurfaceId {
    /// The main surface, created via `.layer_shell()` on the application builder.
    pub const MAIN: Self = Self(0);

    /// Generate a new unique surface ID.
    pub fn unique() -> Self {
        Self(NEXT_SURFACE_ID.fetch_add(1, Ordering::Relaxed))
    }

    pub const fn new(id: u64) -> Self {
        Self(id)
    }
}

impl fmt::Display for SurfaceId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SurfaceId({})", self.0)
    }
}

/// Edge anchoring for a layer surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Anchor(u32);

impl Anchor {
    pub const NONE: Self = Self(0);
    pub const TOP: Self = Self(1);
    pub const BOTTOM: Self = Self(2);
    pub const LEFT: Self = Self(4);
    pub const RIGHT: Self = Self(8);

    pub fn all() -> Self {
        Self(Self::TOP.0 | Self::BOTTOM.0 | Self::LEFT.0 | Self::RIGHT.0)
    }

    pub fn contains(self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }

    pub(crate) fn to_sctk(self) -> wlr_layer::Anchor {
        let mut a = wlr_layer::Anchor::empty();
        if self.contains(Self::TOP) {
            a |= wlr_layer::Anchor::TOP;
        }
        if self.contains(Self::BOTTOM) {
            a |= wlr_layer::Anchor::BOTTOM;
        }
        if self.contains(Self::LEFT) {
            a |= wlr_layer::Anchor::LEFT;
        }
        if self.contains(Self::RIGHT) {
            a |= wlr_layer::Anchor::RIGHT;
        }
        a
    }
}

impl BitOr for Anchor {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl Default for Anchor {
    fn default() -> Self {
        Self::NONE
    }
}

/// Layer on which a surface is placed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Layer {
    Background,
    Bottom,
    #[default]
    Top,
    Overlay,
}

impl Layer {
    pub(crate) fn to_sctk(self) -> wlr_layer::Layer {
        match self {
            Self::Background => wlr_layer::Layer::Background,
            Self::Bottom => wlr_layer::Layer::Bottom,
            Self::Top => wlr_layer::Layer::Top,
            Self::Overlay => wlr_layer::Layer::Overlay,
        }
    }
}

/// Controls whether a surface receives keyboard input.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum KeyboardInteractivity {
    #[default]
    None,
    Exclusive,
    OnDemand,
}

impl KeyboardInteractivity {
    pub(crate) fn to_sctk(self) -> wlr_layer::KeyboardInteractivity {
        match self {
            Self::None => wlr_layer::KeyboardInteractivity::None,
            Self::Exclusive => wlr_layer::KeyboardInteractivity::Exclusive,
            Self::OnDemand => wlr_layer::KeyboardInteractivity::OnDemand,
        }
    }
}

/// Unique identifier for a Wayland output (monitor).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OutputId(pub(crate) u32);

impl fmt::Display for OutputId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "OutputId({})", self.0)
    }
}

/// Information about a connected output (monitor).
#[derive(Debug, Clone)]
pub struct OutputInfo {
    pub id: OutputId,
    pub name: String,
    pub scale_factor: i32,
    pub logical_size: Option<(i32, i32)>,
    pub make: String,
    pub model: String,
}

/// Events related to output (monitor) changes.
#[derive(Debug, Clone)]
pub enum OutputEvent {
    Added(OutputInfo),
    Removed(OutputId),
    InfoChanged(OutputInfo),
}

/// Configuration for a layer shell surface.
#[derive(Debug, Clone)]
pub struct LayerShellSettings {
    pub anchor: Anchor,
    pub layer: Layer,
    pub exclusive_zone: i32,
    pub keyboard_interactivity: KeyboardInteractivity,
    /// Width and height. `None` for a dimension means the compositor decides
    /// (typically full extent for anchored edges).
    pub size: Option<(u32, u32)>,
    pub margin: (i32, i32, i32, i32),
    pub namespace: String,
    /// Target a specific output. `None` lets the compositor choose.
    pub output: Option<OutputId>,
}

impl Default for LayerShellSettings {
    fn default() -> Self {
        Self {
            anchor: Anchor::NONE,
            layer: Layer::Top,
            exclusive_zone: 0,
            keyboard_interactivity: KeyboardInteractivity::None,
            size: None,
            margin: (0, 0, 0, 0),
            namespace: String::from("iced_layer"),
            output: None,
        }
    }
}
