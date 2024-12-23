// SPDX-License-Identifier: LGPL-2.1-only
// SPDX-FileCopyrightText: Copyright 2024 Gaël PORTAY
use std::io;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    while io::stdin().read_line(&mut buffer)? > 0 {
        let line = std::mem::take(&mut buffer);
        print!("{line}");
    }
    Ok(())
}
