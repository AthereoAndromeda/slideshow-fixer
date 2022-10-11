# Slideshow-Fixer
A simple CLI program that fixes the sorting of JPEG presentations

## Motive
created cuz our TV is crap and sorts by `Date Created` rather than `Name` for some reason.
This is the same sorting algorithm in my 2003 TV, where 4:3 and AV was all the rage.

In the end, this is just an automated and overglorified copy-paste program. Except
it copy pastes in a certain way to make the slides sort correctly.

Written in Rust cuz idk, might create WASM variant for Web version of thing

## WebAssembly
To compile for WASM, compatible with Vite
```
wasm-pack build --target web
```