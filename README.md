# rust-minesweeper
A minesweeper written in Rust and SDL.

## Controls
###R:
reset

###Left-click:
- on hidden square: unhide square.
- on unhidden square: if there are enough flags, unhide neighbouring squares.

These actions are recursive.

###Right-click:
toggle flag.
