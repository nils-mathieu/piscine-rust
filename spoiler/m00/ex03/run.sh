#!/bin/sh

rustc fizzbuzz.rs &&
  ./fizzbuzz > fizzbuzz.c &&
  clang fizzbuzz.c -o a.out &&
  ./a.out
