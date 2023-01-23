PANDOC=pandoc-3.0/bin/pandoc

ALL_TARGETS=cv.html cv.pdf

.PHONY: all
all: ${ALL_TARGETS}

cv.html: cv.md template.html
	${PANDOC} --template template.html --standalone -o $@ $<

cv.pdf: cv.md template.latex
	${PANDOC} --template template.latex -o $@ $<

.PHONY: clean
clean:
	rm ${ALL_TARGETS}
