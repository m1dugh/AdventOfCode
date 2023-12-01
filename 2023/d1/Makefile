CC=rustc

BIN = part1 part2

%: %.rs
	$(CC) $(LDFLAGS) -o $@ $^

clean:
	$(RM) $(BIN)

.PHONY: clean all
