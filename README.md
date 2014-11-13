# spine

```toml
[dependencies.spine_tiny]
git = "http://github.com/tomaka/spine-rs"
```

```rust
extern crate spine;
```

Parses a Spine document and calculates what needs to be drawn.

## Step 1: loading the document

Call `SpineDocument::new` to parse the content of a document.

This function returns an `Err` if the document is not valid JSON or if something is not
 recognized in it.

```rust
let document = spine::SpineDocument::new(File::open(&Path::new("skeleton.json")).unwrap())
    .unwrap();
```

## Step 2: preparing for drawing

You can retreive the list of animations and skins provided a document:

```rust
let skins = document.get_skins_list();

let animations = document.get_animations_list();
let first_animation_duration = document.get_animation_duration(animations[0]).unwrap();
```

You can also get a list of the names of all the sprites that can possibly be drawn by this
 Spine animation.

```rust
let sprites = document.get_possible_sprites();
```

Note that the names do not necessarly match file names. They are the same names that you have in
 the Spine editor. It is your job to turn these resource names into file names if necessary.

## Step 3: animating

At each frame, call `document.calculate()` in order to get the list of things that need to be
 drawn for the current animation.

This function takes the skin name, the animation name (or `None` for the default pose) and the
 time in the current animation's loop.

```rust
let results = document.calculate("default", Some("walk"), 0.176).unwrap();
```

The results contain the list of sprites that need to be drawn, with their matrix. The matrix
 supposes that each sprite would cover the whole viewport (ie. drawn from `(-1, -1)` to
 `(1, 1)`). The matrix is pre-multiplying ; if you want to apply your own matrix `C` over
 the one returned, you need to call `C * M`.

```rust
for (sprite_name, matrix, color) in results.sprites.into_iter() {
    let texture = textures_list.find(&sprite_name).unwrap();
    draw(texture, matrix, color);
}
```
