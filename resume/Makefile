PANDOC=pandoc-3.0/bin/pandoc

cv.html: cv.md template.html
	${PANDOC} --template template.html --standalone -o $@ $<

cv.pdf: cv.md template.latex
	${PANDOC} --template template.latex -o $@ $<

.PHONY: all
all: cv.html cv.pdf
