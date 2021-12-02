SHELL		:= /bin/sh
INSTALL		?= install

PREFIX		?= /usr/local
BINDIR		?= ${PREFIX}/bin

TARGET		:= bartib

install: release
	$(INSTALL)    -d ${DESTDIR}/${BINDIR}
	$(INSTALL) -s -t ${DESTDIR}/${BINDIR} target/release/${TARGET}
release:
	cargo build --release --locked
clean:
	cargo clean


.PHONY: release install clean
