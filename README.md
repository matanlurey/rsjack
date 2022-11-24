# rsjack

A Rust implementation of Black Jack.

The goal of this project is to learn a bit more about TUI programming in Rust
(without heavy use of existing libraries). By the end of the implementation
it should be possible to:

1. Start, play, and end a game of Black Jack against an automated Dealer.

... and that's it!

## Getting Started

```txt
$ cargo run -q

Welcome to BlackJack!
DEALER: ??
┌──┐ ┌──┐
│8♥│ │░░│
└──┘ └──┘

PLAYER: 7
┌──┐ ┌──┐
│4♦│ │3♦│
└──┘ └──┘

Q: Quit | D: Draw | S: Stay
d

PLAYER: 17
┌──┐ ┌──┐ ┌──┐
│4♦│ │3♦│ │K♠│
└──┘ └──┘ └──┘

Q: Quit | D: Draw | S: Stay
s

DEALER: 22
┌──┐ ┌──┐ ┌──┐
│8♥│ │4♦│ │Q♣│
└──┘ └──┘ └──┘

Dealer busts ... You win!

DEALER: ??
┌──┐ ┌──┐
│9♣│ │░░│
└──┘ └──┘

PLAYER: 10
┌──┐ ┌──┐
│5♥│ │5♠│
└──┘ └──┘

Q: Quit | D: Draw | S: Stay
q

Bye!
```
