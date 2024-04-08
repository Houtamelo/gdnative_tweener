# gdnative_tweener
- An alternative to Godot 3.5's built-in tween/lerp objects: [Tween](https://docs.godotengine.org/en/3.5/classes/class_tween.html)|[SceneTreeTween](https://docs.godotengine.org/en/3.5/classes/class_scenetreetween.html)
- Inspired on the design of the Unity package [DoTween](https://dotween.demigiant.com/)

## Warning 1
This package is not throughly tested, it likely contains game-breaking bugs.
That being said, I use it everyday on every one of my projects, and I will be fixing any reported bugs as fast as I can, at least until january 2026.

## Warning 2
This package is largely undocumented, this readme is all that there is, and it will likely remain so since the author is the only known user.
PRs adding documentation will be accepted though.

## Warning 3
This package requires Rust nightly, and it uses unstable features such as autotraits and negative impls.

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

fn tween_ref_wrapper_color(ref_owner: Ref<Node2D>, color: Color, duration: f64) -> Result<TweenID<TweenProperty_Color>> {
    ref_owner.do_color(color, duration)
             .register()
}

fn tween_tref_color(tref_owner: TRef<Node2D>, color: Color, duration: f64) -> Result<TweenID<TweenProperty_Color>> {
    ref_owner.do_color(color, duration)
             .register()
}

fn tween_ref_color(owner: &Node2D, color: Color, duration: f64) -> Result<TweenID<TweenProperty_Color>> {
    ref_owner.do_color(color, duration)
             .register()
}

// `do_color` accepts any of &T|Ref<T>|TRef<T>, as long as T is a SubClass<CanvasItem>, this is because CanvasItem is the class that defines the property `modulate`, which `do_color` acts on.
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
`register()` simply accesses the [TweensController](src/singleton.rs) autoload, and inserts the newly-created tween to it's list.
It also returns a [TweenID](src/tweens/id.rs), which you can use to access the Tween later, to check if the tween is alive, it's state, etc.

`gdnative_tweener` provides several methods for configuring the behavior of your tweens, although undocumented, the names are fairly intuitive, and the code is simple.
You should make all the configurations before registering the Tween, modifying the tween after registering can be performance-intensive, as it requires acquiring a lock on the [TweensController](src/singleton.rs) autoload.
I do not recommend attempting to register/access tweens outside the main thread.

Here are some examples:
```rs
let owner: Ref<Node2D> = ..;
let mut tween = owner.do_move_x(2000.0, 5.0);

// To mimic TweenProperty::as_relative()
tween = tween.lerp_relative();

// To mimic TweenProperty::from_current()
// Starting from the current value is the default behavior for `do_property`, so you don't need to manually call this, unless you created the tween in other ways.
tween = tween.starting_at_current()?; // this one returns a Result because it may fail to acquire a valid value from "owner.get_property_indexed()"

// To mimic TweenProperty::from(Variant)
let starting_x = 66.6;
tween = tween.starting_at(starting_x);

// To mimic TweenProperty::set_delay(f64);
tween = tween.with_delay(10.0);


```

Property tweeners are merely wrappers around the trait [DoProperty](https://github.com/Houtamelo/gdnative_tweener/blob/main/src/property/property.rs).
`do_property` automatically binds the node to the tween, just like [Node::create_tween](https://docs.godotengine.org/en/3.5/classes/class_node.html#class-node-method-create-tween), this means that the tween will automatically "die" when the node is deleted.

rs
```

```

# Know that...
- `gdnative_tweener` does not interact in any way with Godot's built-in tweens, it's a standalone NativeScript that processes the registered tweens on it's `_process(f64)` and `_physics_process(f64)` virtual methods.

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





