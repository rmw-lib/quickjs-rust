#!/usr/bin/env bash

DIR=$(dirname $(realpath "$0"))
cd $DIR

cd quickjs
cat quickjs.h|rg "JS_New"
cat quickjs.h|rg "JS_To"
