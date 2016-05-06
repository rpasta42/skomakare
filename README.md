
2D Graphics environment with a custom scheme-like scripting language called [lambda-oxide](https://github.com/KostyaKow/LambdaOxide). The main aim of the project is to teach lisp to kids and introduce people to progamming. Skomakare is written in rust and uses glium.

[Example Tic Tac Toe game](https://github.com/KostyaKow/skomakare/blob/master/tictactoe.lo)

[Drawing circle from triangles](https://github.com/KostyaKow/skomakare/blob/master/circle.lo)


```scheme
$ cargo run
**>(l "circle.lo")
**>(l "tictactoe.lo")
**>(define my-triangle (triangle red))
**>(move my-triangle 0.3 0.3)
**>(define my-square (square "data/opengl.png"))
**>(define rotate-square
      (lambda (i)
         (do (sleep 0.05)
             (rotate my-square (/ pi 10))
             (if (> i 0)
                  (rotate-square (- i 1))
                  "done"))))
**>(rotate-square 1000)
```

What works, what doesn't:
- [ ] Making basic shapes with colors
   - [x] triangle
   - [x] square
   - [ ] circle
      - [ ] primitive shape constructed in rust
      - [x] can construct circle with triangles from loscript (very slow)
- [ ] colors/textures
   - [x] red, green & blue
   - [ ] custom color from rgb
   - [x] texture from png file (works for triangles and squares)
   - [ ] texture from jpeg
   - [ ] add changecolor for existing project
   - [ ] setting clearcolor from loscript
- [x] changing shapes
   - [x] moving shapes
   - [x] rotating shapes
   - [x] scaling shapes
   - [x] setting shape position
   - [ ] setting rotation
   - [ ] setting scale
   - [ ] add get_pos, get_rot, get_scale to loscript
- [ ] scene manager
   - [ ] connecting rust scenemanager to loscript
   - [ ] quadtrees
- [x] mouse and keyboard input
   - Very basic, needs a re-write
- [ ] misc
   - [ ] lisp shaders
   - [ ] check out stuff from [logo](https://en.wikipedia.org/wiki/Logo_%28programming_language%29)

old todo:

- [ ] TODO
   - [ ] write README.md
   - [ ] [document Lisp API](https://github.com/KostyaKow/skomakare/blob/master/src/main.rs#L53)
   - [ ] Add pos, rot, scale

