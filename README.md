[<img alt="github" src="https://img.shields.io/static/v1?label=github&message=ansinator&color=acb0d0&logo=Github&style=flat-square&logoColor=a9b1d6" height="20">](https://github.com/dax99993/ansinator)
[<img alt="crates" src="https://img.shields.io/crates/v/ansinator?logo=rust&logoColor=a9b1d6&style=flat-square&color=fc8d62" height="20">](https://crates.io/crates/ansinator)
<div align="center">

  <h3>
Ansinator is a collection of CLI programs to convert images to various character representation with support for <a href ="https://en.wikipedia.org/wiki/ANSI_escape_code" > ANSI escape code sequence </a>
  </h3>
</div>

<div align="center">
  <a href="https://github.com/dax99993/ansinator/blob/main/demo/demo.md">Demo</a>
  <br/><br/>
</div>

## Installation
#### Cargo:
You can install the binary crate directly
```sh
cargo install ansinator 
```

#### Manual Installation:
you can clone **ansinator** repo and build it locally
```sh
git clone https://github.com/dax99993/ansinator
cd ansinator 
cargo install --path .
```

## Programs
- Ascii
- Block
- Braile 8-dot
- Uniblock (sextant)


## Todo
- [x] Best fitting ascii character analysis 
- [ ] Simple animation

## License
[MIT](https://mit-license.org/)

## Compatibility
This application was design to work in any unicode terminal but the result might look different
depending on the terminal font and settings.
Testing has been performn only on Alacritty, Kitty and ST,
using the JetBrains Mono font.
#### Ascii
Probably will work for all terminals
-- Alacritty working
-- Kitty working
-- ST working
#### Braile
Probably will work on all terminals supporting Unicode Glyphs
-- Alacritty working but background color might extend
-- Kitty working
-- ST working
#### Block
Probably will work on all terminals supporting Unicode Glyphs
-- Alacritty working
-- Kitty working
-- ST working
#### Uniblock 
Probably will work on all terminals supporting Unicode Glyphs, but it also depends 
on the way the terminal handles sextant characters. 
-- Alacritty working but might look weird depending on the x,y offset and the font 
-- Kitty working
-- Alacritty works but might look weird depending on the x,y offset and the font 
-- rxvt-unicode not working missing sextant characters (might be a misconfiguration of my part of the fonts)

## Bugs
If find any bug or weird behaviour send an email to <b>dax99993@gmail.com</b>

## Log
- 0.2.3  Fixed bug in fixed background color for ascii, braile and uniblock, also fixed background color extension to end of line on some terminals.
- 0.2.2  Fixed convertion in braile and block, and fixed resizing.
- 0.2.1  added both GRADIENT mode with PATTERN mode in ascii, and change help message and flags.
- 0.2.0  replace GRADIENT mode with PATTERN mode in ascii.
- 0.1.1  Uniblock convertion added, and extra options as fixed foreground and background
- 0.1.0  Basic ascii, braile, block

