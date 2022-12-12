use core::fmt;

use limine::LimineTerminalRequest;
use spin::Mutex;

use super::PornosWriter;

static TERMINAL_REQUEST: LimineTerminalRequest = LimineTerminalRequest::new(0);
pub static PORNOS_WRITER: Mutex<Writer> = Mutex::new(Writer {terminals: None});

pub struct Writer {
    terminals: Option<&'static limine::LimineTerminalResponse>,
}

unsafe impl Send for Writer {}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        // Get the Terminal response and cache it.
        let response = match self.terminals {
            None => {
                let response = TERMINAL_REQUEST.get_response().get().ok_or(fmt::Error)?;
                self.terminals = Some(response);
                response
            }
            Some(resp) => resp,
        };

        let write = response.write().ok_or(fmt::Error)?;

        // Output the string onto each terminal.
        for terminal in response.terminals() {
            write(terminal, s);
        }

        Ok(())
    }
}

impl PornosWriter for Writer {}
