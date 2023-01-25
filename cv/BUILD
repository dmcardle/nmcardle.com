load("//rules:pandoc.bzl", "pandoc")

filegroup(
    name = "cv",
    srcs = [
        ":cv_html",
        ":cv_pdf",
    ],
)

pandoc(
    name = "cv_html",
    srcs = ["cv.md"],
    out_format = "html",
    template = "template.html",
)

pandoc(
    name = "cv_pdf",
    srcs = ["cv.md"],
    out_format = "pdf",
    template = "template.latex",
)
