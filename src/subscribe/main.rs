// SPDX-License-Identifier: LGPL-2.1-only
// SPDX-FileCopyrightText: Copyright 2024 Gaël PORTAY
use std::env;

fn main() -> redis::RedisResult<()> {
    let args= env::args().collect::<Vec<String>>();
    env::args().nth(1).expect("Usage: subscribe CHANNEL...");
    let mut conn = redis::Client::open("redis://:@localhost")
        .expect("Invalid connection URL")
        .get_connection()
        .expect("failed to get_connection()");
    let mut pubsub = conn.as_pubsub();
    for i in 1..args.len() {
        pubsub.subscribe(args[i].clone())?;
    }

    loop {
        let message = pubsub.get_message()?;
        let payload : String = message.get_payload()?;
        println!("{}: {}", message.get_channel_name(), payload);
    }
}
