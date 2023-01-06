use std::io::Write;

use crossterm::{
    cursor::{self, MoveTo},
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    terminal::{Clear, ClearType},
    ExecutableCommand, QueueableCommand,
};

#[derive(Debug)]
struct RawMode;

impl RawMode {
    fn enable() -> crossterm::Result<Self> {
        crossterm::terminal::enable_raw_mode().map(|_| Self)
    }
}

impl Drop for RawMode {
    fn drop(&mut self) {
        if let Err(e) = crossterm::terminal::disable_raw_mode() {
            eprintln!("failed to disable raw mode: {e:?}");
        }
    }
}

#[derive(Debug)]
pub struct Tui<I> {
    _raw_mode: RawMode,
    position: (u16, u16),
    quit_on: I,
}

impl<I> Tui<I> {
    pub fn new(quit_on: I) -> crossterm::Result<Self> {
        let raw_mode = RawMode::enable()?;
        std::io::stdout().lock().execute(crossterm::cursor::Hide)?;
        Ok(Tui {
            _raw_mode: raw_mode,
            position: cursor::position()?,
            quit_on,
        })
    }

    pub fn next_key(&self) -> crossterm::Result<Option<char>>
    where
        I: IntoIterator<Item = char> + Copy,
    {
        let result = loop {
            let cmd = event::read()?;
            if let Event::Key(key) = cmd {
                use KeyCode::*;
                match key {
                    KeyEvent {
                        code: Char(c),
                        modifiers: KeyModifiers::NONE,
                        ..
                    } if self.quit_on.into_iter().any(|q| q == c) => break Ok(None),
                    KeyEvent {
                        code: Char('c' | 'd'),
                        modifiers: KeyModifiers::CONTROL,
                        ..
                    } => break Ok(None),
                    KeyEvent {
                        code: Char(c),
                        modifiers: KeyModifiers::NONE,
                        ..
                    } => break Ok(Some(c)),
                    _ => {}
                }
            }
        };
        let mut out = std::io::stdout().lock();
        out.queue(MoveTo(self.position.0, self.position.1))?;
        out.queue(Clear(ClearType::FromCursorDown))?;
        out.flush()?;
        result
    }
}

impl<I> Drop for Tui<I> {
    fn drop(&mut self) {
        if let Err(e) = crossterm::terminal::disable_raw_mode() {
            eprintln!("failed to disable raw mode: {e:?}");
        }
        if let Err(e) = std::io::stdout().lock().execute(crossterm::cursor::Show) {
            eprintln!("failed to disable raw mode: {e:?}");
        }
    }
}
