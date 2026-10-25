//! Bridges the renderer's [`Shell`] notifications to the calloop event loop.

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use iced_graphics::Shell;
use iced_graphics::shell::Notifier;

/// Coalesced relayout/redraw requests from the renderer.
#[derive(Clone)]
pub(crate) struct ShellRequests {
    relayout: Arc<AtomicBool>,
    redraw: Arc<AtomicBool>,
    ping: calloop::ping::Ping,
}

impl ShellRequests {
    pub(crate) fn new(ping: calloop::ping::Ping) -> Self {
        Self {
            relayout: Arc::new(AtomicBool::new(false)),
            redraw: Arc::new(AtomicBool::new(false)),
            ping,
        }
    }

    pub(crate) fn shell(&self) -> Shell {
        Shell::new(self.clone())
    }

    #[must_use]
    pub(crate) fn take_relayout(&self) -> bool {
        self.relayout.swap(false, Ordering::AcqRel)
    }

    #[must_use]
    pub(crate) fn take_redraw(&self) -> bool {
        self.redraw.swap(false, Ordering::AcqRel)
    }
}

impl Notifier for ShellRequests {
    fn request_redraw(&self) {
        self.redraw.store(true, Ordering::Release);
        self.ping.ping();
    }

    fn invalidate_layout(&self) {
        self.relayout.store(true, Ordering::Release);
        self.ping.ping();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn requests() -> ShellRequests {
        let (ping, _source) = calloop::ping::make_ping().expect("ping source");
        ShellRequests::new(ping)
    }

    #[test]
    fn redraw_and_relayout_are_independent() {
        let requests = requests();

        assert!(!requests.take_relayout());
        assert!(!requests.take_redraw());

        requests.request_redraw();
        assert!(!requests.take_relayout());
        assert!(requests.take_redraw());
        assert!(!requests.take_redraw());

        requests.invalidate_layout();
        assert!(requests.take_relayout());
        assert!(!requests.take_relayout());
        assert!(!requests.take_redraw());
    }

    #[test]
    fn repeated_requests_coalesce() {
        let requests = requests();

        for _ in 0..8 {
            requests.invalidate_layout();
            requests.request_redraw();
        }

        assert!(requests.take_relayout());
        assert!(!requests.take_relayout());
        assert!(requests.take_redraw());
        assert!(!requests.take_redraw());
    }
}
