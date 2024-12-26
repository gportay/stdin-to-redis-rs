# SPDX-License-Identifier: LGPL-2.1-only
# SPDX-FileCopyrightText: Copyright 2024 Gaël PORTAY

.PHONY: all
all: build

.PHONY: build
build:
	cargo build

.PHONY: publish
publish: run-publish

.PHONY: subscribe
subscribe: run-subscribe

run-publish:
run-subscribe:
run-%:
	cargo run --bin $* -- stdin
