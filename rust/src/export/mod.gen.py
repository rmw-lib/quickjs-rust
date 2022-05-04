#!/usr/bin/env python
from os.path import abspath,dirname,join
from render import render
from glob import glob

PWD = dirname(abspath(__file__))
PWD_LEN = len(PWD)+1

li = []
for name in glob(join(PWD,'*.rs')):
  name = name[PWD_LEN:-3]
  if name in ('mod',):
    continue
  li.append(name)

render(join(PWD,'mod.rs'),li=li)
