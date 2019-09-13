
.PHONY: lib

java_exec: java_run
	java -jar -Djava.library.path=mylib/target/debug/ imagecodec/target/image.codec-1.0.jar $(DIR) $(MAGIC)

java_run: lib
	cd imagecodec && mvn clean install package

lib:
	cd mylib && cargo build
