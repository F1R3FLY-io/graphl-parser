
# Define a variable for the filename
PARSER_MAKEFILE := parser/Makefile

# Target to check if the file exists
check_file:
ifeq ($(wildcard $(PARSER_MAKEFILE)),)
	@echo "File does not exist."
	mkdir -p ./parser/dist
else
	@echo "File exists."
	cd parser && make distclean
endif

gen_parser: check_file
	bnfc --c -m -o ./parser  etc/grammar.bnfc

build_obj:
	cd parser && make

archive:
	cd parser && ar rcs libparser.a Absyn.o Parser.o Printer.o Buffer.o Lexer.o

build: gen_parser build_obj archive
	@echo "Done"

test:
	cargo test -- --no-capture --test-threads=1
