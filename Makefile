NAME := mae
VERSION := $(shell git describe --tags --always --dirty)
BUILD_DAY := $(shell date -u +"%Y-%m-%d")
MANPAGE := docs/man/$(NAME).1
PREFIX ?= "/usr/local"
RELEASE_BIN := "./target/release/$(NAME)"

.PHONY: man
man: $(MANPAGE)

$(MANPAGE): $(MANPAGE).md
	sed "s/VERSION_PLACEHOLDER/${VERSION}/g" $< | \
	 	sed "s/DATE_PLACEHOLDER/${BUILD_DAY}/g" | \
	 	pandoc --standalone -f markdown -t man -o $@


.PHONY: build
build:
	cargo build --all-targets

.PHONY: release
release:
	cargo build --release

.PHONY: test
test: test-clean
	CARGO_TERM_COLOR=always cargo test --verbose --workspace

.PHONY: install
install: release man
	install -d $(PREFIX)/bin
	install -m 0555 $(RELEASE_BIN) $(PREFIX)/bin
	install -d $(PREFIX)/share/man/man1/
	install -m 0644 $(MANPAGE) $(PREFIX)/share/man/man1/

.PHONY: local-install
local-install:
	$(MAKE) install PREFIX=usr/local

.PHONY: clean
clean: test-clean
	rm -rf ./target $(MANPAGE)

.PHONY: test-clean
test-clean:
	rm -rf ./test/tmp_output/*

.DEFAULT_GOAL := build

###
# Release build tasks
###
BUILD_OS ?= "macos"
BUILD_ARCH ?= "arm64"
RELEASE_BIN := "./target/release/$(NAME)"
ARTIFACT_BIN := $(NAME)-$(VERSION).$(BUILD_OS).$(BUILD_ARCH)

RELEASE_ARTIFACTS_DIR := .release_artifacts
CHECKSUM_FILE := $(RELEASE_ARTIFACTS_DIR)/$(ARTIFACT_BIN).checksum.txt

$(RELEASE_ARTIFACTS_DIR):
	install -d $@

.PHONY: build-standalone
build-standalone: release $(RELEASE_ARTIFACTS_DIR)
	mv $(RELEASE_BIN) $(RELEASE_ARTIFACTS_DIR)/$(ARTIFACT_BIN)
	shasum -a 256 $(RELEASE_ARTIFACTS_DIR)/$(ARTIFACT_BIN) >> $(CHECKSUM_FILE)


.PHONY: github-release
github-release:
	gh release create $(VERSION) --title 'v$(VERSION)' \
	 	--notes-file docs/releases/$(VERSION).md $(MANPAGE)

.PHONY: upload-release-artifacts
upload-release-artifacts:
	gh release upload $(VERSION) $(RELEASE_ARTIFACTS_DIR)/*

