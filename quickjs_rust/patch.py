#!/usr/bin/env python3

from os.path import abspath, dirname, join


def add(txt, before, insert):
    p = txt.find(before)
    if p < 0:
        print(f"🔥 {before} not exist")
        return

    end = p + len(before)
    p = txt.find(insert, end)
    if p >= 0:
        return

    p = txt.find("\n", end)
    if p < 0:
        p = len(txt)

    txt = txt[:p] + "\n" + insert + txt[p:]
    return txt


def pairwise(iterable):
    a = iter(iterable)
    return zip(a, a)


def patch(file, *args):
    fp = join(dirname(dirname(abspath(__file__))), join("quickjs", file))
    with open(fp) as infile:
        txt = infile.read()

        for before, insert in pairwise(args):
            txt = add(txt, before, insert)
            if not txt:
                return
        if txt:
            with open(fp, "w") as out:
                out.write(txt)


def main():
    for args in (
        (
            "quickjs.c",
            '#include "quickjs.h"',
            '#include "quickjs_rust/quickjs_rust.h"',
            "JS_AddIntrinsicBaseObjects(ctx);",
            "JS_AddRust(ctx,rt);",
            "void JS_FreeRuntime(JSRuntime *rt)\n{",
            "rust_rt_exit(rt);",
            "assert(ctx->header.ref_count == 0);",
            "rust_ctx_exit(ctx,ctx->rt);",
            "if (list_empty(&rt->job_list)) {\n",
            "if(rust_run(rt))\n{return 1;}\nelse",
        ),
        (
            "quickjs-libc.c",
            '#include "quickjs-libc.h"',
            '#include "rust/rust.h"',
            "if (!os_poll_func || os_poll_func(ctx))",
            "if (rust_ended(ctx))",
        ),
        (
            "Makefile",
            "QJS_LIB_OBJS=",
            "QJS_LIB_OBJS+=$(OBJDIR)/../rust/target/release/librust.a",
        ),
    ):
        patch(*args)


if __name__ == "__main__":
    main()
