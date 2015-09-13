# rust-minesweeper
A minesweeper written in Rust and SDL.

I've done this as an exercise for learning Rust. If you like, you may try to read my source code to learn Rust yourself, but since it is not commented, nor is it necessarily idiomatic Rust, I'm not sure I'd recommend it.

## Controls
###R:
reset

###Left-click:
- on hidden square: unhide square.
- on unhidden square: if there are enough flags, unhide neighbouring squares.

These actions are recursive.

###Right-click:
toggle flag.
