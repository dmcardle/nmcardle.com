%.html: %.md
	pandoc --standalone -o $@ $<

%.pdf: %.md
	pandoc -o $@ $<

.PHONY: all
all: resume.html resume.pdf
