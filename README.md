# gdnative_tweener is..
- An alternative to Godot 3.5's built-in tween/lerp objects: [Tween](https://docs.godotengine.org/en/3.5/classes/class_tween.html) | [SceneTreeTween](https://docs.godotengine.org/en/3.5/classes/class_scenetreetween.html)
- Inspired on the design of the Unity package [DoTween](https://dotween.demigiant.com/)

# Warning!
- This package is not throughly tested, it likely contains game-breaking bugs. That being said, I use it everyday on all my projects, and I am committed to fixing any reported bugs as fast as I can, at least until july 2025.
- This package is largely undocumented, this readme is all that there is, and it will likely remain so since the author is the only known user. PRs adding documentation will be accepted though.
- This package requires Rust nightly, and it uses unstable features such as autotraits and negative impls.

# Summary
- [Usage](#usage)
- [Setup](#setup)
- [Help](#help)


# Usage
When using Godot's built-in `SceneTreeTween`, tweening the color of a node requires doing the following:
```rs
fn tween_color(owner: Ref<Node2D>, color: Color, duration: f64) -> Result<SceneTreeTween> {
    let owner_tref = unsafe {
        owner.assume_safe_if_sane()
             .ok_or_else(|| Error("Owner is not sane, cannot create tween."))?
    };

    let tween =
        owner_tref.create_tween()
                  .ok_or_else(|| Error("Could not create tween."))?;

    let tween_tref = unsafe {
        tween.assume_safe()
    };

    tween_tref.tween_property(owner, "modulate", color, duration);

    return tween;
}
```

That is quite a bit of boilerplate for something you may be doing frequently depending on how much animations your game has.
The purpose of this library is to achieve the same functionatility but in a more ergonomic way.
Here's how you can tween a node's color using `gdnative_tweener`:

```rs
use gdnative_tweener::prelude::*;

fn tween_ref_wrapper_color(owner: Ref<Node2D>, color: Color, duration: f64) -> Result<TweenID<TweenProperty_Color>> {
    owner.do_color(color, duration)
	 .register()
}

// `do_color` is implemented on any of &T|Ref<T>|TRef<T>, where T: SubClass<CanvasItem>, this is because CanvasItem is the class that defines the property `modulate`, which `do_color` acts on.
fn tween_any_subclass_of_canvas_item(ref_sprite: Ref<Sprite>, tref_tilemap: TRef<TileMap>, control: &Control) -> Result<()> {
    let color = Color::from_rgb(0.8, 0.0, 0.8); // purple :3
    let duration = 0.69;

    let tween_id_of_ref = ref_sprite.do_color(color, duration).register()?;

    let tween_id_of_tref = tref_tilemap.do_color(color, duration).register()?;

    let tween_id = control.do_color(color, duration).register()?;

    Ok(())
}
```

Note that, unlike [DoTween](https://dotween.demigiant.com/), we need to call register() at the end.
`register()` attempts to access the [TweensController](src/singleton.rs) autoload, inserting the tween on it's internal list.
It also returns a Result<[TweenID](src/tweens/id.rs)>, which you can use to access the Tween later, to check if the tween is alive, it's state, etc.
The operation will fail if the autoload is inacessible (which can happen when it doesn't exists, or if it is already borrowed somewhere else).

`gdnative_tweener` provides several methods for configuring the behavior of your tweens, although undocumented, the names are fairly intuitive and similar to `DoTween`.
You should make all the configurations before registering the Tween, modifying the tween after registering can be performance-intensive, as it requires acquiring a lock on the [TweensController](src/singleton.rs) autoload.
I do not recommend attempting to register/access tweens outside the main thread.

Here are some examples:
```rs
let owner: Ref<Node2D> = ..;
let mut tween = owner.do_move_x(2000.0, 5.0);

// To mimic TweenProperty::as_relative()
let origin = 0.0; // since relative tweens don't have a starting point, the origin parameter is necessary for the tween to compare how much it needs to move between each frame
tween = tween.as_relative(origin);

// To mimic TweenProperty::from_current(), no need to do anything, that's the default behavior,
// instead, if you want to set a specific starting point:
tween = tween.starting_at(500.0);

// To mimic TweenProperty::set_delay(f64);
tween = tween.with_delay(10.0);

// SpeedBased is also implemented:
let speed = 100.0; // Speed is always in units per second, 100. speed means position:x increases by 100 each second.
tween = tween.as_speed_based(speed);
```

And you can chain all these methods together:
```rs
let tween_id =
	node.do_global_position(Vector2::new(1920., 1080.), 5.0)
		.with_delay(10.0)
		.starting_at(Vector2::new(0., 0.))
		.looped(10)
		.when_finished(node, "print", vec!["Tween finished!".to_variant()])
		.register()?;
```

Property tweeners are merely wrappers around the trait [DoProperty](https://github.com/Houtamelo/gdnative_tweener/blob/main/src/property/property.rs).
`do_property` automatically binds the node to the tween, just like [Node::create_tween](https://docs.godotengine.org/en/3.5/classes/class_node.html#class-node-method-create-tween), this means that the tween will automatically "die" when the node is deleted.

DoProperty uses `call_deferred("set_indexed")`, so property paths are also valid:
```rs
// Tween only the Z position
let tween = owner.do_property("position:z", 20., 10.);
```

DoProperty accepts generic values, as long as they implement _Lerp + FromVariant + ToVariant:
```rs
let my_custom_property = "..";
let my_custom_end_val = CustomStruct {..};
let duration = 5.0;

// the only difference is that the method is called do_property_var()
let tween = owner.do_property_var(my_custom_property, my_custom_end_val, duration);
```

You can also tween methods:
```rs
// Imagine you have a method like this...
#[method]
fn _set_fill(&self, #[base] owner: &Range, value: f64) {
	owner.set_value(value);
}

// You can tween it like this
let start = 0.;
let end = 1.;
let duration = 8.;
let tween = owner.do_method("_set_fill", start, end, duration);
```

If you just need to call a method once, with a delay, you can:
```rs
#[method]
fn _start_game(&mut self, player_name: String, difficulty: Difficulty) {
	...
}

let player_name = "Houtamelo";
let difficulty = Difficulty::GameDeveloper;
let delay = 24.;
let tween = owner.do_callback("_start_game", vec![player_name.to_variant(), difficulty.to_variant()], delay);
```

## Never forget to register your tweens! They will just be dropped otherwise:
```rs
let id = tween.register()?;
```

We also have sequences, that work very much like DoTween's:
```rs
let mut sequence = Sequence::new();
sequence.append(owner.do_move_x(640.0, 5.0));
sequence.join(owner.do_fade(1.0, 4.0));
sequence.append(owner.do_global_move(Vector2::new(124., 256.));
sequence.insert(8.0, owner.do_scale(Vector2::new(10., 10.));

let id = sequence.register()?;
```

Need to kill a tween?
```rs
let id: TweenID<TweenCallback> = ..;
id.kill();

// Same for sequences
let sequence_id: SequenceID = ..;
sequence_id.kill();
```

Need to complete a tween immediately?
```rs
tween_id.complete();
sequence_id.complete();
```

# Know that...
- `gdnative_tweener` does not interact in any way with Godot's built-in tweens, it's a standalone NativeScript that processes the registered tweens on it's `_process(f64)` and `_physics_process(f64)` virtual methods.
- Every single interaction the tweens perform is done in a "deferred" manner... ("set_property_deferred", "call_deferred")

# Setup
The setup process assumes you're already familiar with how [GDNative](https://godot-rust.github.io/gdnative-book/) works.

### Step 1
Add `gdnative_tweener` to your project's dependencies:
run `cargo add gdnative_tweener`
or
add following line to your Cargo.toml:
`gdnative_tweener = "0.3"`

You're most likely going to want to use the latest version, as it will contain the latest bug-fixes.

### Step 2
Register the `TweenController` class in your library's init (in the gdnative template: `rust/src/lib.rs`):
```rs
fn init(handle: InitHandle) {
	handle.add_class::<gdnative_tweener::prelude::TweensController>();
}
```

### Step 3
In your Godot project, create a `NativeScript` for the `TweensController` class:
![image](https://github.com/Houtamelo/gdnative_tweener/assets/88971943/da0f6b81-c76f-4b75-b56d-4059a2d8246a)

### Step 4
Add the `TweensController` class to your project's autoload, with the node named as "tweens_controller":
![image](https://github.com/Houtamelo/gdnative_tweener/assets/88971943/83bd22ef-fe9a-4380-a738-a958f0492a5b)

Warning: The node must be named "tweens_controller", otherwise you'll receive an error whenever attempting to register a tween.

# Help
Open an issue or send me a message on discord: `houtamelo`
