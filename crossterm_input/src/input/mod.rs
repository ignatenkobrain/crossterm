//! A module that contains all the actions related to reading input from the terminal.
//! Like reading a line, reading a character and reading asynchronously.

mod input;

#[cfg(unix)]
mod unix_input;
#[cfg(windows)]
mod windows_input;

#[cfg(unix)]
pub use self::unix_input::AsyncReader;
#[cfg(unix)]
pub use self::unix_input::SyncReader;
#[cfg(unix)]
use self::unix_input::UnixInput;

#[cfg(windows)]
pub use self::windows_input::AsyncReader;
#[cfg(windows)]
pub use self::windows_input::SyncReader;
#[cfg(windows)]
use self::windows_input::WindowsInput;

pub use self::input::{input, TerminalInput};
use crossterm_utils::Result;
use std::io;
use std::sync::{
    mpsc::{Receiver, Sender},
    Arc,
};

/// This trait defines the actions that can be performed with the terminal input.
/// This trait can be implemented so that a concrete implementation of the ITerminalInput can fulfill
/// the wishes to work on a specific platform.
///
/// ## For example:
///
/// This trait is implemented for Windows and UNIX systems.
/// Unix is using the 'TTY' and windows is using 'libc' C functions to read the input.
trait ITerminalInput {
    /// Read one character from the user input
    fn read_char(&self) -> io::Result<char>;
    /// Read the input asynchronously from the user.
    fn read_async(&self) -> AsyncReader;
    ///  Read the input asynchronously until a certain character is hit.
    fn read_until_async(&self, delimiter: u8) -> AsyncReader;
    /// Read the input synchronously from the user.
    fn read_sync(&self) -> SyncReader;
    fn enable_mouse_mode(&self) -> Result<()>;
    fn disable_mouse_mode(&self) -> Result<()>;
}

/// Enum to specify which input event has occurred.
#[derive(Debug, PartialOrd, PartialEq, Hash, Clone)]
pub enum InputEvent {
    /// A single key or a combination is pressed.
    Keyboard(KeyEvent),
    /// A mouse event occurred.
    Mouse(MouseEvent),
    /// A unsupported event has occurred.
    Unsupported(Vec<u8>),
    /// An unknown event has occurred.
    Unknown,
}

/// Enum to specify which mouse event has occurred.
#[derive(Debug, PartialOrd, PartialEq, Hash, Clone, Copy)]
pub enum MouseEvent {
    /// A mouse press has occurred, this contains the pressed button and the position of the press.
    Press(MouseButton, u16, u16),
    /// A mouse button was released.
    Release(u16, u16),
    /// A mouse button was hold.
    Hold(u16, u16),
    /// An unknown mouse event has occurred.
    Unknown,
}

/// Enum to define mouse buttons.
#[derive(Debug, PartialOrd, PartialEq, Hash, Clone, Copy)]
pub enum MouseButton {
    /// Left mouse button
    Left,
    /// Right mouse button
    Right,
    /// Middle mouse button
    Middle,
    /// Scroll up
    WheelUp,
    /// Scroll down
    WheelDown,
}

/// Enum with different key or key combinations.
#[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Hash)]
pub enum KeyEvent {
    Backspace,
    Left,
    Right,
    Up,
    Down,
    Home,
    End,
    PageUp,
    PageDown,
    BackTab,
    Delete,
    Insert,
    F(u8),
    Char(char),
    Alt(char),
    Ctrl(char),
    Null,
    Esc,
    CtrlUp,
    CtrlDown,
    CtrlRight,
    CtrlLeft,
    ShiftUp,
    ShiftDown,
    ShiftRight,
    ShiftLeft,
}
