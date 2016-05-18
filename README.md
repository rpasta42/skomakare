
2D Graphics environment with a custom scheme-like scripting language called [lambda-oxide](https://github.com/KostyaKow/LambdaOxide). The main aim of the project is to teach lisp to kids and introduce people to progamming. Skomakare is written in rust and uses glium.

[Example Tic Tac Toe game](https://github.com/KostyaKow/skomakare/blob/master/examples/tictactoe.lo)

[Drawing circle from triangles](https://github.com/KostyaKow/skomakare/blob/master/examples/circle.lo)

For rendering text, you need to install comic sans ttf font (sudo apt-get install ttf-mscorefonts-installer).

```scheme
$ ./setup.sh; cargo run
**>(l "circle.lo")
**>(l "tictactoe.lo")
**>(define my-triangle (triangle red))
**>(move my-triangle 0.3 0.3)
**>(define my-square (square "resources/opengl.png"))
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
- [ ] options to create hidden objects
- [ ] defining new shaders from string in loscirpt
- [ ] Making basic shapes with colors
   - [ ] check-exists function (?? forgot what this note is about)
   - [x] triangle
   - [x] square
   - [x] circle
      - [x] can export blender circle to obj after triangulation
      - [ ] primitive shape constructed in rust
      - [x] can construct circle with triangles from loscript (very slow)
- [ ] colors/textures
   - [ ] texture compiled from string and added to shader manager from loscript.
   - [x] red, green & blue
   - [ ] custom color from rgb
   - [x] texture from png file (works for triangles and squares)
   - [ ] texture from jpeg
   - [ ] add changecolor for existing project
   - [ ] setting clearcolor from loscript
- [x] text rasterization
   - [x] very ugly comic sans without picking colors
   - [x] fix opposite orientation
   - [x] create better-looking fragment shader with gradients
   - [ ] account for aspect ratio of text, and be able to pass resolution manually
- [x] changing shapes
   - [x] moving shapes
   - [x] rotating shapes
   - [x] scaling shapes
   - [x] setting shape position
   - [ ] setting rotation
   - [ ] flip function (x and y)
   - [ ] setting scale
   - [ ] add get_pos, get_rot, get_scale to loscript
- [x] add tiny sleep on the render thread loop waiting for events
- [ ] scene manager
   - [ ] connecting rust scenemanager to loscript
   - [ ] quadtrees
- [x] mouse and keyboard input
   - Very basic, needs a re-write
- [x] setup script needs to also be able to update examples-data
- [ ] misc
   - [ ] lisp shaders
   - [ ] check out stuff from [logo](https://en.wikipedia.org/wiki/Logo_%28programming_language%29)
- [ ] custom shapes with points, lines or trianges from loscript
   - [ ] PrimitiveType::Points
- [ ] https://en.wikipedia.org/wiki/Octree http://www.gamedev.net/page/resources/_/technical/game-programming/introduction-to-octrees-r3529

old todo:

- [ ] TODO
   - [ ] write README.md
   - [ ] [document Lisp API](https://github.com/KostyaKow/skomakare/blob/master/src/main.rs#L53)
   - [ ] Add pos, rot, scale
   - [ ] https://doc.rust-lang.org/book/iterators.html


