

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
