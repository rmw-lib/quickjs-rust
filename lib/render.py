#!/usr/bin/env python
from mako.template import Template
from os.path import join, exists


def render(file, **kwds):
    with open(join(f"{file}.mako")) as template:
        txt = Template(template.read()).render(**kwds)
    if exists(file):
        with open(join(file)) as exist:
            if exist.read() == txt:  # 可以避免 make 重新构建
                return
    with open(join(file), "w") as out:
        out.write(txt)
