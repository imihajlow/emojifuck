# Emojifuck

A Brainfuck dialect using emojis instead of ASCII operators.

## Commands

Emojifuck is a superset of Brainfuck. A Brainfuck program without emoji is a valid Emojifuck program. Additionally, two emoji mappings exist.

### Classic Brainfuck

 * `>` Increment the data pointer (to point to the next cell to the right).
 * `<` Decrement the data pointer (to point to the next cell to the left).
 * `+` Increment (increase by one) the byte at the data pointer.
 * `-` Decrement (decrease by one) the byte at the data pointer.
 * `.` Output the byte at the data pointer.
 * `,` Accept one byte of input, storing its value in the byte at the data pointer.
 * `[` If the byte at the data pointer is zero, then instead of moving the instruction pointer forward to the next command, jump it forward to the command after the matching ] command.
 * `]` If the byte at the data pointer is nonzero, then instead of moving the instruction pointer forward to the next command, jump it back to the command after the matching [ command. 

### Hands mapping

 * `👉` means `>`
 * `👈` means `<`
 * `👆` means `+`
 * `👇` means `-`
 * `🤌` means `.`
 * `🤏` means `,`
 * `🤜` means `[`
 * `🤛` means `]`

### Wide emoji mapping

 * Happy faces (e.g. 😀 or 🤣) stand for `>`
 * Sad or angry faces (e.g 😥 or 😭) stand for `<`
 * Hearts (❤️, 🫀) stand for `+`
 * Hats (👒, ⛑) stand for `-`
 * Herbivore animals (🦏, 🐄) stand for `.`
 * Carnivore animals (🐈, 🦅) stand for `,`
 * Females (🤵‍♀️, 🧖‍♀️) stand for `[`
 * Males (🥷, 👨‍🏫) stand for `]`

The possible accepted emojis are not limited to ones shown as examples.

## Installation

To install, run:

```
cargo install --path .
```


## Usage

To run a program:

```
emojifuck program.bf
```


To translate a Brainfuck program to emojis:

```
emojifuck --print-emoji program.bf
```

To translate a Brainfuck program to hand emojis:

```
emojifuck --print-hands program.bf
```


To translate an Emojifuck program to pure Brainfuck:

```
emojifuck --print-classic program.bf
```

## Examples

Some examples can be found in `examples/` directory.

A hello world program:

```
💗💝💟💖💖💟💛💝🧕😅💟💜💘💙👩😙🤍💝😚💕🤎💝😍🖤💜💚😗💓🤒🥶😓🙁🧢👦😍💕😋🫀🙂⛑😁😝💙🤶🤕👲😖🎩🕺🤪😝🦧🙃🧢⛑🎓🦘💖🤎💗💝🧡🤍💜🐘🦄💖💜💔🐒😝🤪🦧😞🎓🐑😥🦜💙💔🧡🦒🪖🎩🎩🧢🧢🎓🦕👑🪖⛑🪖🎩👑🪖🧢🐒😇😛💘🐎🥲🤍🤎🐼
```
