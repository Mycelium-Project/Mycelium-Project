java_run: lib
	javac Shiitake.java && java -Djava.library.path=shiitake_lib/target/debug/ Shiitake

.PHONY: lib

javah:
	javah Shiitake

lib:
	cd shiitake_lib && cargo build