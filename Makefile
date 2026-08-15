# Makefile — thin convenience wrapper so `make help` (default), `make build`,
# `make run`, `make bin`, and `make install` work without the cargo / atb flag set.
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
BINDIR ?= bin

.PHONY: help build run bin install

help: ## Print Makefile targets
	@printf 'Usage: make [TARGET] [VAR=value]\n\nTargets:\n'
	@awk 'BEGIN {FS = ":.*## "} \
	      /^[a-zA-Z][a-zA-Z0-9_-]*:.*## / {printf "  %-10s %s\n", $$1, $$2}' \
	      $(MAKEFILE_LIST)
	@printf '\nVariables:\n'
	@printf '  ARGS      extra argv for `make run` (default: --help)\n'
	@printf '  BINDIR    where `make bin` puts atb (default: bin)\n'

build: ## Build the atb binary (debug)
	@$(CARGO) build --bin atb

run: ## Run atb (ARGS='sync --help'; default ARGS is --help)
	@$(ATB) $(ARGS)

bin: ## Build a release atb into ./bin (override with BINDIR)
	@$(CARGO) build --release --bin atb
	@mkdir -p $(BINDIR)
	@cp target/release/atb $(BINDIR)/atb
	@printf 'built %s/atb\n' '$(BINDIR)'

install: ## Install atb onto cargo's bin dir
	@$(CARGO) install --path . --force
