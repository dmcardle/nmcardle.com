load("//rules:pandoc.bzl", "pandoc")

filegroup(
    name = "cv",
    srcs = [
        ":dan_mcardle_cv_html_foo",
        ":dan_mcardle_cv_pdf_foo",
    ],
    visibility = ["//:__pkg__"],
)

# N.B.: The pandoc rule generates the output filename from the target's name.
# The generated files' names must be kept in sync with `firebase.json`.

pandoc(
    name = "dan_mcardle_cv_html_foo",
    srcs = ["cv.md"],
    out_format = "html",
    template = "template.html",
)

pandoc(
    name = "dan_mcardle_cv_pdf_foo",
    srcs = ["cv.md"],
    out_format = "pdf",
    template = "template.latex",
)
