output = $(join $(dir $(o)), $(join bin/, $(notdir $(o))))
run: $(output)
	zig cc $(o).c -ggdb3 -o $(output) && $(output)

$(output):
	mkdir -p $(dir $@)
