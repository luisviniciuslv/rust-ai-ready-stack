PROTOC_VERSION ?= 29.3
TOOLS_DIR ?= .tools
PROTOC_HOME := $(TOOLS_DIR)/protoc

ifeq ($(OS),Windows_NT)
PROTOC_ZIP := protoc-$(PROTOC_VERSION)-win64.zip
PROTOC_URL := https://github.com/protocolbuffers/protobuf/releases/download/v$(PROTOC_VERSION)/$(PROTOC_ZIP)
PROTOC_BIN := $(PROTOC_HOME)/bin/protoc.exe
else
UNAME_S := $(shell uname -s)
ifeq ($(UNAME_S),Darwin)
PROTOC_ZIP := protoc-$(PROTOC_VERSION)-osx-x86_64.zip
else
PROTOC_ZIP := protoc-$(PROTOC_VERSION)-linux-x86_64.zip
endif
PROTOC_URL := https://github.com/protocolbuffers/protobuf/releases/download/v$(PROTOC_VERSION)/$(PROTOC_ZIP)
PROTOC_BIN := $(PROTOC_HOME)/bin/protoc
endif

.PHONY: protoc-setup protoc-clean

protoc-setup:
	@echo "[protoc] Downloading $(PROTOC_URL)"
	@mkdir -p $(PROTOC_HOME)
ifeq ($(OS),Windows_NT)
	@powershell -NoProfile -Command "$$ErrorActionPreference='Stop'; New-Item -ItemType Directory -Force -Path '$(PROTOC_HOME)' | Out-Null; Invoke-WebRequest -Uri '$(PROTOC_URL)' -OutFile '$(PROTOC_HOME)/$(PROTOC_ZIP)'; Expand-Archive -Force '$(PROTOC_HOME)/$(PROTOC_ZIP)' '$(PROTOC_HOME)'; New-Item -ItemType Directory -Force -Path '.cargo' | Out-Null; Set-Content -Encoding UTF8 .cargo/config.toml \"[env]`nPROTOC = { value = \\\"$(PROTOC_BIN)\\\", relative = true }\""
else
	@curl -fsSL "$(PROTOC_URL)" -o "$(PROTOC_HOME)/$(PROTOC_ZIP)"
	@unzip -oq "$(PROTOC_HOME)/$(PROTOC_ZIP)" -d "$(PROTOC_HOME)"
	@chmod +x "$(PROTOC_BIN)"
	@mkdir -p .cargo
	@printf '[env]\nPROTOC = { value = "$(PROTOC_BIN)", relative = true }\n' > .cargo/config.toml
endif
	@echo "[protoc] Configured: $(PROTOC_BIN)"
	@echo "[protoc] Cargo env written to .cargo/config.toml"

protoc-clean:
	@echo "[protoc] Cleaning local protoc artifacts"
	@rm -rf "$(PROTOC_HOME)" .cargo/config.toml
