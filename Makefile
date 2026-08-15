# Makefile — thin convenience wrapper so `make help` (default), `make build`,
# `make run`, and `make install` work without the cargo / atb flag set.
#
# Cargo builds the `atb` binary from src/main.rs.

# GNU Make ignores $SHELL from the environment and defaults to /bin/sh.
# Honor the user's login shell when make's SHELL was not set on the command line.
ifneq ($(origin SHELL),command line)
SHELL := $(shell printf '%s' "$${SHELL:-/bin/sh}")
endif
.SHELLFLAGS := -eu -c
.DEFAULT_GOAL := help

CARGO ?= cargo
ATB ?= $(CARGO) run --quiet --bin atb --
ARGS ?= --help

.PHONY: help build run install

help: ## Print Makefile targets
	@printf 'Usage: make [TARGET] [VAR=value]\n\nTargets:\n'
	@awk 'BEGIN {FS = ":.*## "} \
	      /^[a-zA-Z][a-zA-Z0-9_-]*:.*## / {printf "  %-10s %s\n", $$1, $$2}' \
	      $(MAKEFILE_LIST)
	@printf '\nVariables:\n'
	@printf '  ARGS      extra argv for `make run` (default: --help)\n'

build: ## Build the atb binary (debug)
	@$(CARGO) build --bin atb

run: ## Run atb (ARGS='sync --help'; default ARGS is --help)
	@$(ATB) $(ARGS)

install: ## Install atb onto cargo's bin dir
	@$(CARGO) install --path . --force
