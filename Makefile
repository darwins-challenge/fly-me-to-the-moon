PNGS := $(patsubst graphviz/%.dot,graphviz/%.dot.png,$(wildcard graphviz/*.dot))

all: $(PNGS)

graphviz/%.dot.png: graphviz/%.dot
	dot -O -Tpng $<
