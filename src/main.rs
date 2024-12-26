// SPDX-License-Identifier: LGPL-2.1-only
// SPDX-FileCopyrightText: Copyright 2024 Gaël PORTAY
use std::env;
use std::io;

fn main() -> io::Result<()> {
    let key = env::args().nth(1).expect("Usage: stdin-to-redis KEY");
    let mut conn = redis::Client::open("redis://:@localhost")
        .expect("Invalid connection URL")
        .get_connection()
        .expect("failed to get_connection()");
    let mut buffer = String::new();
    while io::stdin().read_line(&mut buffer)? > 0 {
        let line = std::mem::take(&mut buffer);
        let value = line.strip_suffix("\n").unwrap();
        let float= match value.parse::<f64>() {
            Ok(f) => {
                f
            },
            Err(_) => {
                eprintln!("Warning: {value}: Not a float, ignoring...");
                continue;
            },
        };
        redis::cmd("TS.ADD")
            .arg(key.clone())
            .arg("*")
            .arg(float)
            .exec(&mut conn)
            .expect("failed to TS.ADD");
        redis::cmd("PUBLISH")
            .arg(key.clone())
            .arg(float)
            .exec(&mut conn)
            .expect("failed to PUBLISH");
        print!("{line}");
    }
    Ok(())
}
