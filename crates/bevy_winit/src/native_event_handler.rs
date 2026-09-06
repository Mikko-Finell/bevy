//! Optional main-thread integration with the native event loop.

use winit::{
    event::{StartCause, WindowEvent},
    event_loop::ActiveEventLoop,
    window::WindowId,
};

/// An optional native integration installed with `App::insert_non_send`.
///
/// Callbacks run synchronously on the event-loop thread, outside application
/// schedules. They do not suppress Bevy's ordinary window/event processing.
/// This is intended for embedded toolkits and input adapters that must service
/// native events or timers independently of display updates. The handler must
/// not recursively run the app or access its render world.
pub struct NativeEventHandler(pub Box<dyn NativeEventCallbacks>);

/// Native callbacks for a [`NativeEventHandler`].
///
/// `about_to_wait` runs after Bevy's ordinary frame/wait decision. A handler
/// adding a timer should preserve polling and shorten, rather than extend, an
/// existing wait deadline. The next `new_events` callback can service due work.
pub trait NativeEventCallbacks: 'static {
    /// Called before Bevy processes a new batch of native events.
    fn new_events(&mut self, _event_loop: &ActiveEventLoop, _cause: StartCause) {}
    /// Called before Bevy translates a native window event.
    fn window_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _window: WindowId,
        _event: &WindowEvent,
    ) {
    }
    /// Called after Bevy's ordinary wait decision and frame scheduling.
    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {}
    /// Called after native windows have been created on resume.
    fn resumed(&mut self, _event_loop: &ActiveEventLoop) {}
    /// Called before Bevy marks the application for suspension.
    fn suspended(&mut self, _event_loop: &ActiveEventLoop) {}
    /// Called before Bevy destroys windows and clears its world.
    fn exiting(&mut self, _event_loop: &ActiveEventLoop) {}
}
