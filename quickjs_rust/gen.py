#!/usr/bin/env python3
from os.path import abspath,dirname,join
from render import render

PWD = dirname(abspath(__file__))
ROOT = dirname(PWD)

li = []

with open(join(ROOT,'rust/rust.h')) as rust:
  for i in rust:
    if i.startswith('#define '):
      i = i.split(' ',2)
      if len(i) == 3:
        name = i[1]
        if name.endswith('_args_len'):
          li.append(name[:-9])

render(join(PWD,'js_rust_funcs.h'),li=li)

  #print(Template("hello ${data}!").render(data="world"))
