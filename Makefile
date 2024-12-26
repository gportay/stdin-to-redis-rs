# SPDX-License-Identifier: LGPL-2.1-only
# SPDX-FileCopyrightText: Copyright 2024 Gaël PORTAY

.PHONY: all
all: build

.PHONY: build
build:
	cargo build

.PHONY: run
run:
	cargo run -- stdin
