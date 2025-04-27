load("//rules:pandoc.bzl", "pandoc")

filegroup(
    name = "cv",
    srcs = [
        ":dan_mcardle_cv_html",
        ":dan_mcardle_cv_pdf",
        ":dan_mcardle_resume_html",
    ],
    visibility = ["//:__pkg__"],
)

# N.B.: The pandoc rule generates the output filename from the target's name.
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

pandoc(
    name = "dan_mcardle_resume_html",
    srcs = ["resume.md"],
    out_format = "html",
    template = "template.html",
)
