def _pandoc_impl(ctx):
    out_file = ctx.actions.declare_file("{}.{}".format(ctx.label.name, ctx.attr.out_format))

    args = ctx.actions.args()
    args.add("--standalone")
    args.add("--embed-resources") # Use `data` URIs when emitting HTML.
    args.add("--verbose")
    args.add("--pdf-engine=lualatex")
    args.add("--lua-filter")
    args.add_all(ctx.files._pandoc_ext)

    if ctx.files.template:
        args.add("--template")
        args.add_all(ctx.files.template)

    args.add("-o")
    args.add(out_file)
    args.add("-t")
    args.add(ctx.attr.out_format)
    args.add_all(ctx.files.srcs)

    ctx.actions.run(
        inputs = ctx.files.srcs + ctx.files.template,
        outputs = [out_file],
        arguments = [args],
        executable = ctx.attr.pandoc_bin.files_to_run,
        # Pandoc must not be sandboxed so it can use the system `pdflatex`.
        execution_requirements = {
            "no-sandbox": "",
        },
        env = {
            # Without PATH defined, pandoc will look for `pdflatex` in the cwd.
            "PATH": ":".join(["/usr/bin", "/Library/TeX/texbin", "/opt/homebrew/bin"])
        },
    )

    return [DefaultInfo(files = depset([out_file]))]

_pandoc_rule = rule(
    implementation = _pandoc_impl,
    attrs = {
        "pandoc_bin": attr.label(allow_single_file = True),
        "_pandoc_ext": attr.label(default = "@pandoc_ext//:_extensions/diagram/diagram.lua", allow_single_file = True),
        "srcs": attr.label_list(allow_files = True),
        "template": attr.label(allow_single_file = True),
        "out_format": attr.string(),
    },
)

def pandoc(**kwargs):
    _pandoc_rule(
        pandoc_bin = select({
            "@bazel_tools//src/conditions:darwin_arm64": "@pandoc_arm64-macOS//:pandoc",
            "@bazel_tools//src/conditions:linux_x86_64": "@pandoc_linux-amd64//:pandoc",
        }),
        **kwargs,
    )
