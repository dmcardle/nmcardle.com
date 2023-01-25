load("//rules:pandoc.bzl", "pandoc")

filegroup(
    name = "cv",
    srcs = [
        ":dan_mcardle_cv_html",
        ":dan_mcardle_cv_pdf",
    ],
)

# N.B.: The pandoc rule generates the output file's name from the target name.
# The generated files' names must be kept in sync with `firebase.json`.

pandoc(
    name = "dan_mcardle_cv_html",
    srcs = ["cv.md"],
    out_format = "html",
    template = "template.html",
)

pandoc(
    name = "dan_mcardle_cv_pdf",
    srcs = ["cv.md"],
    out_format = "pdf",
    template = "template.latex",
)
