// FILE: aspect_window_input_listener.rs
// occt: Aspect_WindowInputListener

/// Defines a listener for window input events.
///
/// This is an abstract trait that handles keyboard, mouse, touch, and 3D mouse input.
/// Implementations must provide event processing methods.
pub trait AspectWindowInputListener {
    /// Handle expose event (window content has been invalidated and should be redrawn).
    fn process_expose(&mut self);

    /// Handle window resize event.
    fn process_configure(&mut self, is_resized: bool);

    /// Handle window input event immediately (flush input buffer or ignore).
    fn process_input(&mut self);

    /// Handle focus event.
    fn process_focus(&mut self, is_activated: bool);

    /// Handle window close event.
    fn process_close(&mut self);

    /// Press key.
    /// @param key key pressed
    /// @param time event timestamp
    /// @param pressure pressure value (default 1.0)
    fn key_down(&mut self, key: u32, time: f64, pressure: f64);

    /// Release key.
    /// @param key key released
    /// @param time event timestamp
    fn key_up(&mut self, key: u32, time: f64);

    /// Simulate key up/down events from axis value.
    /// @param negative negative key
    /// @param positive positive key
    /// @param time event timestamp
    /// @param pressure pressure value
    fn key_from_axis(&mut self, negative: u32, positive: u32, time: f64, pressure: f64);

    /// Update mouse scroll event.
    /// @return TRUE if new event has been created or FALSE if existing one has been updated
    fn update_mouse_scroll(&mut self, delta_x: f64, delta_y: f64) -> bool;

    /// Handle mouse button press/release event.
    /// @param point mouse cursor position
    /// @param buttons pressed buttons
    /// @param modifiers key modifiers
    /// @param is_emulated if TRUE then mouse event comes NOT from real mouse
    /// @return TRUE if window content should be redrawn
    fn update_mouse_buttons(
        &mut self,
        point_x: i32,
        point_y: i32,
        buttons: u32,
        modifiers: u32,
        is_emulated: bool,
    ) -> bool;

    /// Handle mouse cursor movement event.
    /// @param point mouse cursor position
    /// @param buttons pressed buttons
    /// @param modifiers key modifiers
    /// @param is_emulated if TRUE then mouse event comes NOT from real mouse
    /// @return TRUE if window content should be redrawn
    fn update_mouse_position(
        &mut self,
        point_x: i32,
        point_y: i32,
        buttons: u32,
        modifiers: u32,
        is_emulated: bool,
    ) -> bool;

    /// Add touch point with the given ID.
    /// @param id touch unique identifier
    /// @param pnt touch coordinates
    /// @param clear_before if TRUE previously registered touches will be removed
    fn add_touch_point(&mut self, id: usize, pnt_x: f64, pnt_y: f64, clear_before: bool);

    /// Remove touch point with the given ID.
    /// @param id touch unique identifier
    /// @param clear_select_pnts if TRUE will initiate clearing of selection points
    /// @return TRUE if point has been removed
    fn remove_touch_point(&mut self, id: usize, clear_select_pnts: bool) -> bool;

    /// Update touch point with the given ID.
    /// If point with specified ID was not registered before, it will be added.
    /// @param id touch unique identifier
    /// @param pnt touch coordinates
    fn update_touch_point(&mut self, id: usize, pnt_x: f64, pnt_y: f64);
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockListener {
        expose_called: bool,
        configure_called: bool,
        input_called: bool,
        focus_called: bool,
        close_called: bool,
    }

    impl AspectWindowInputListener for MockListener {
        fn process_expose(&mut self) {
            self.expose_called = true;
        }

        fn process_configure(&mut self, _is_resized: bool) {
            self.configure_called = true;
        }

        fn process_input(&mut self) {
            self.input_called = true;
        }

        fn process_focus(&mut self, _is_activated: bool) {
            self.focus_called = true;
        }

        fn process_close(&mut self) {
            self.close_called = true;
        }

        fn key_down(&mut self, _key: u32, _time: f64, _pressure: f64) {}
        fn key_up(&mut self, _key: u32, _time: f64) {}
        fn key_from_axis(&mut self, _negative: u32, _positive: u32, _time: f64, _pressure: f64) {}
        fn update_mouse_scroll(&mut self, _delta_x: f64, _delta_y: f64) -> bool {
            true
        }
        fn update_mouse_buttons(
            &mut self,
            _point_x: i32,
            _point_y: i32,
            _buttons: u32,
            _modifiers: u32,
            _is_emulated: bool,
        ) -> bool {
            true
        }
        fn update_mouse_position(
            &mut self,
            _point_x: i32,
            _point_y: i32,
            _buttons: u32,
            _modifiers: u32,
            _is_emulated: bool,
        ) -> bool {
            true
        }
        fn add_touch_point(&mut self, _id: usize, _pnt_x: f64, _pnt_y: f64, _clear_before: bool) {}
        fn remove_touch_point(&mut self, _id: usize, _clear_select_pnts: bool) -> bool {
            true
        }
        fn update_touch_point(&mut self, _id: usize, _pnt_x: f64, _pnt_y: f64) {}
    }

    #[test]
    fn test_listener_methods() {
        let mut listener = MockListener {
            expose_called: false,
            configure_called: false,
            input_called: false,
            focus_called: false,
            close_called: false,
        };

        listener.process_expose();
        assert!(listener.expose_called);

        listener.process_configure(true);
        assert!(listener.configure_called);

        listener.process_input();
        assert!(listener.input_called);

        listener.process_focus(true);
        assert!(listener.focus_called);

        listener.process_close();
        assert!(listener.close_called);
    }

    #[test]
    fn test_mouse_events() {
        let mut listener = MockListener {
            expose_called: false,
            configure_called: false,
            input_called: false,
            focus_called: false,
            close_called: false,
        };

        let result = listener.update_mouse_buttons(100, 200, 1, 0, false);
        assert!(result);

        let result = listener.update_mouse_position(100, 200, 1, 0, false);
        assert!(result);

        let result = listener.update_mouse_scroll(5.0, 10.0);
        assert!(result);
    }

    #[test]
    fn test_touch_events() {
        let mut listener = MockListener {
            expose_called: false,
            configure_called: false,
            input_called: false,
            focus_called: false,
            close_called: false,
        };

        listener.add_touch_point(1, 100.0, 200.0, false);
        listener.update_touch_point(1, 105.0, 205.0);
        let removed = listener.remove_touch_point(1, false);
        assert!(removed);
    }

    #[test]
    fn test_key_events() {
        let mut listener = MockListener {
            expose_called: false,
            configure_called: false,
            input_called: false,
            focus_called: false,
            close_called: false,
        };

        listener.key_down(65, 0.0, 1.0);
        listener.key_from_axis(83, 87, 0.0, 1.0);
        listener.key_up(65, 0.1);
    }
}
