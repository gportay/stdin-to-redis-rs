# SPDX-License-Identifier: LGPL-2.1-only
# SPDX-FileCopyrightText: Copyright 2024 Gaël PORTAY

.PHONY: all
all: build

.PHONY: build clean
build clean:
	cargo $@

.PHONY: publish
publish: run-publish

.PHONY: subscribe
subscribe: run-subscribe

run-publish:
run-subscribe:
run-%:
	cargo run --quiet --bin $* -- stdin

.PHONY: mqtt-publish
mqtt-publish:
	scripts/mqtt-publish localhost stdin

.PHONY: mqtt-subscribe
mqtt-subscribe:
	scripts/mqtt-subscribe localhost stdin | $(MAKE) run-publish
