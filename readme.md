# Sedust

## What is this?

Sed written in Rust.  

More than that, this is an opportunity for me to learn
Rust. Eventually I want this implementation of sed to be *fairly*
complete, whatever that means. My goal is not to create a faster or
more feature-rich version of sed, though I may someday explore some
optimizations.

## Is it done yet?

Nope! Heck, I haven't even implemented the **s** command yet or regex
addresses, which are arguably the most used features of sed. So here's
what's ipmlemented so far:

- Numerical addresses
- Commands
  - a
  - c
  - d
  - g and G
  - h and H
  - i
  - p and P
  - q
  - r
  - w
  - x
  - =
  - comments (#)
- Handles multiple input files
- Only one script can be specified (i.e. no ; or script files)

Here's what still needs done (a lot!):

- Command line switches
  - -e
  - -f
  - -n
- Regex addresses
- Improved script parsing to handle multiple scripts
  - For example, sedust cannot yet handle 'n;n;s/./X' since this is three scripts separated by semi-colons
- Commands
  - b
  - D
  - l
  - n and N
  - s
  - t
  - y
  - :

## The Master Plan

This is not a shining example of Rust code. This is my first Rust
project and it's probably going to look ugly along the way (this has
had no code reviews to date). My goal is to slowly but surely address
the todo's listed above, create good test coverage, and then once
everything is working and correct, make the code more idiomatic.