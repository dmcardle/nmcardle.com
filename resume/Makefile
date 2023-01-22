PANDOC=pandoc-3.0/bin/pandoc

resume.html: resume.md template.html
	${PANDOC} --template template.html --standalone -o $@ $<

resume.pdf: resume.md template.latex
	${PANDOC} --template template.latex -o $@ $<

.PHONY: all
all: resume.html resume.pdf
