# iTermColors in Rust

This project can currently parse an `.itermcolors` file and convert it to
a Kitty file. Currently, it only parses `iceberg.itermcolors` because I
haven't had time to do it properly yet.

To run (after installing Cargo toolchain, etc):
```
cargo run
```

Output:
```
Kitty color scheme:
foreground #c6c8d1
background #161821
selection_foreground #222222
selection_background #9aa5db
color0  #161821
color1  #e27878
color2  #b4be82
color3  #e2a478
color4  #84a0c6
color5  #a093c7
color6  #89b8c2
color7  #c6c8d1
color8  #6b7089
color9  #e98989
color10 #c0ca8e
color11 #e9b189
color12 #91acd1
color13 #ada0d3
color14 #95c4ce
color15 #d2d4de
```

You can replace `iceberg.itermcolors` in `main` with some other file if you
want. PRs to turn this into a real library are welcome!

