// SPDX-FileCopyrightText: 2026 Ahum Maitra <theahummaitra@gmail.com>
// SPDX-License-Identifier: GPL-3.0-or-later

//    Copyright (C) 2026 Ahum Maitra

//      This program is free software: you can redistribute it and/or modify
//      it under the terms of the GNU General Public License as published by
//      the Free Software Foundation, either version 3 of the License, or
//      (at your option) any later version.

//      This program is distributed in the hope that it will be useful,
//      but WITHOUT ANY WARRANTY; without even the implied warranty of
//      MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//      GNU General Public License for more details.

//      You should have received a copy of the GNU General Public License
//      along with this program.  If not, see <https://www.gnu.org/licenses/>.

use crossterm::event::{self, Event, KeyCode};
use std::{process::Command, time::Duration};

fn main() -> std::io::Result<()> {
    let mut child = Command::new("termflix")
        .args(["--clean", "--cycle", "15"])
        .spawn()?;

    loop {
        if child.try_wait()?.is_some() {
            break;
        }

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Esc
                    | KeyCode::Left
                    | KeyCode::Right
                    | KeyCode::Char('q')
                    | KeyCode::Char('r')
                    | KeyCode::Char('b')
                    | KeyCode::Char('c')
                    | KeyCode::Char('h') => {
                        // Let termflix handle these
                    }

                    _ => {
                        let _ = child.kill();
                        let _ = child.wait();
                        break;
                    }
                }
            }
        }
    }

    Ok(())
}
