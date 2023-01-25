def _pandoc_impl(ctx):
  out_file = ctx.actions.declare_file("pandoc-generated." + ctx.attr.out_format)

  args = ctx.actions.args()
  args.add("--verbose")
  args.add("--template")
  args.add_all(ctx.files.template)
  args.add("-o")
  args.add(out_file)
  args.add("-t")
  args.add(ctx.attr.out_format)
  args.add_all(ctx.files.srcs)

  ctx.actions.run(
    inputs = ctx.files.srcs,
    outputs = [out_file],
    arguments = [args],
    executable = ctx.attr._pandoc_bin.files_to_run,
    execution_requirements = {
        "no-sandbox": "",
    },
    env = {
        # Without PATH defined, pandoc will look for `pdflatex` in the cwd.
        "PATH": "/usr/bin",
    },
  )

  return [ DefaultInfo(files = depset([out_file])) ]

pandoc = rule(
    implementation = _pandoc_impl,
    attrs = {
      "_pandoc_bin": attr.label(default="@pandoc//:pandoc", allow_single_file=True),
      "srcs": attr.label_list(allow_files=True),
      "template": attr.label(allow_single_file=True),
      "out_format": attr.string(),
    },
)
