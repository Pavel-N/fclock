PREFIX ?= /usr
CARGO ?= cargo

all:
	@echo Run \"make install\" to install fclock

install:
	@echo "Building..."
	@$(CARGO) build --release
	@echo "Installing..."
	@sudo install -Dm755 ./target/release/fclock $(DESTDIR)$(PREFIX)/bin/fclock
	@echo "Done, enjoy! :)"

uninstall:
	@echo "Uninstalling..."
	@sudo rm -f $(DESTDIR)$(PREFIX)/bin/fclock
	@echo "Successfully uninstalled :("

clean:
	@echo "Cleaning... :P"
	@$(CARGO) clean
	@echo "Done!"
