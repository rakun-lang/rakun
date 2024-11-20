ifndef RAKUN_NO_COMPILE
RAKUN_INSTALLED := $(shell command -v rakun)
endif

ERLANG_FILES = $(wildcard {src,build}/*.erl)
RAKUN_FILES = $(wildcard {src,build}/**/*.rakun)

.PHONY: ebin check

ifdef RAKUN_INSTALLED
ebin: check $(RAKUN_FILES) $(ERLANG_FILES)
	rakun compile-package --target erlang
else
ebin: check $(ERLANG_FILES)
	@mkdir -p ./ebin
	@cp build/*.app ebin/
	@erlc -server -o ebin $(ERLANG_FILES) || (rm -rf ebin && false)
endif

check:
ifndef ERL_LIBS
	$(error "ERL_LIBS environment variable not set")
endif
