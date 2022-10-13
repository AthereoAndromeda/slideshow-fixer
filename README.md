# Slideshow-Fixer
A simple CLI program and WASM Module that fixes the sorting of JPEG presentations.  
Also compatible with ZIP files.

## Motive
created cuz our TV is crap and sorts by `Date Created` rather than `Name` for some reason.
This is the same sorting algorithm in my 2003 TV, where 4:3 and A/V via RCA cables were all the rage.

In the end, this is just an automated and overglorified copy-paste program. Except
it copy pastes in a certain way to make the slides sort correctly.

Written in Rust cuz idk, might create WASM variant for Web version of thing

## Usage
1. Your folder or ZIP file of JPEG Presentation. Files will be sorted alphabetically. Typically `[01.jpg, 02.jpg, 03.jpg, ...]`
1. Run `./target/release/slideshow-fixer --help`
1. Program will change `Date Created` while maintaining original sorting, so files are sorted alphabetically in the TV
1. it's self explanatory rlly
1. Save to Flash Drive
1. Profit $$


## WebAssembly
To compile for WASM, compatible with Vite

**Debug:**
```
wasm-pack build --dev -t web --features wasm
```

**Production:**
```
wasm-pack build -t web --features wasm
```