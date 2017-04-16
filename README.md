# Typing training

Command line typing training software.


## Usage

Clone the repository and compile it with the rust toolchain.

```rust
git clone https://github.com/VaysseB/typing_training.git .
cd typing_training
cargo build
cargo run
```

Current built under:

* Linux 4.7.10-1-MANJARO x86_64 GNU/Linux
* rustup 1.2.0
* cargo-0.17.0-nightly (f9e5481 2017-03-03)
* rustc 1.16.0 (30cf806ef 2017-03-10)
* _as info:_ gcc/g++ 6.3.1 20170306
* _as info:_ clang version 3.9.1

## Features

| Features | Progress |
|----------|----------|
| Keyboard typing                       | &#9745; Done (since 0.1.0) |
| Words database                        | &#9744; TBD |
| Language layout database              | &#9744; TBD |
| Keyboard keys layout database         | &#9744; TBD |
| Multiple visual feedback              | &#9744; TBD |
| Multiple typing logic                 | &#9744; TBD |
| Training statistics                   | &#9744; TBD |
| Language layout autodetection         | &#9746; Not planned |
| Keyboard keys layout autodetection    | &#9746; Not planned |


## Key project objectives

1. Multiplaftorm software (targets: GNU/Linux, and later Windows). This is achieved by using Rust.
1. Multi-language layouts handling, for known and user-definied layouts. The aim is to provide a list the most used language layouts (e.g. _qwerty_), but also provide more specific ones (e.g. _dvorak_).
1. Multi-keyboard key layout handling. This is linked to the keyboard as a hardware equipement. They are some differences in keyboard's keys position.
1. Adaptation of typing training according to an exercise. The list of words to type in will be selected around some constraints: language layout, keyboard key layout, training objective and maybe even exercise history.
1. Customization of visual feedback, like highlights, colors.
1. Customization of exercise based of user-prefered input logic. See relevant section below.
1. Statistics on training, with identification of typing difficulties.

_Personals_

1. Learning rust.
2. Making a true software in rust, and not only a proof-of-concept or learning material.

## Inputs logics

Apps and website for typing tutorial used different kind of inputs logic in reaction to errors.
Simply put, on valid keystroke, all of them move to the next character, but they differ on invalid keystroke.

1. The game blocks on the character mistyped. It waits on position until the user type the right key,
1. The game continues forward but marked the character as failed, and forbids correction.
1. The game continues forward but marked the character as failed, and allows correction using backspace.

At the end of a failed word (which contains at least on failed character), the game:

1. Restarts the word from the begining,
1. Continues, but forbids to go back on passed and failed words once moved to the next one.
1. Continues, and allows undoing previous passed and failed words.



## History

I've always typed in Azerty since childhood without really learning it.
I became a software programmer but I was fascinating about how azerty is inefficient, forcing thumbs, hands, wristle and the full arm to move to type a single key.

After doing some research, I started to learn dvorak in early 2015. I switched completely to it in septembre 2015, and ditch azerty for sure.

At first, training was only with Notepad-like editor, simply to simple learn key positions. Then, I switched to typing website ([try this one](https://learn.dvorak.nl/?lang=en&lesson=1)) to really learn it.

Now, as a french guy, I also need to type in my native language, but I cannot find any good _Bépo_ learning typing website or app. I did try some of some of the most famous ones, but they all lack _Bépo_.

So, why not build a new app to learn (Rust and _Bépo_)?

## License

See LICENSE file.
