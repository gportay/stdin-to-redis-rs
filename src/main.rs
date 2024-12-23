// SPDX-License-Identifier: LGPL-2.1-only
// SPDX-FileCopyrightText: Copyright 2024 Gaël PORTAY
use redis::Commands;
use std::io;

fn main() -> io::Result<()> {
    let mut conn = redis::Client::open("redis://:@localhost")
        .expect("Invalid connection URL")
        .get_connection()
        .expect("failed to get_connection()");
    let mut buffer = String::new();
    while io::stdin().read_line(&mut buffer)? > 0 {
        let line = std::mem::take(&mut buffer);
        let _: () = conn.set("line", line).expect("failed to set('line')");
        let line: String = conn.get("line").expect("failed to get('line')");
        print!("{line}");
    }
    Ok(())
}
