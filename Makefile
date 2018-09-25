

DEFAULT = make

CARGO = cargo

all: $(DEFAULT)

clean:
	$(Q)$(CARGO) clean

make:
	$(Q)$(CARGO) build

build-release:
	$(Q)$(CARGO) build --release

run:
	$(Q)$(CARGO) run

test:
	$(Q)$(CARGO) test

test-bench:
	$(Q)RUST_BACKTRACE=1 time $(CARGO) test

watch-tests:
	$(Q)watch -n 5 $(CARGO) test

test-spec:
	$(Q)RUST_BACKTRACE=1 time $(CARGO) run tests/language_spec.ldst
