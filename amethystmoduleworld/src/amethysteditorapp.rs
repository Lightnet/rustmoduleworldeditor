extern crate amethyst;
extern crate amethyst_editor_sync;
extern crate serde;
extern crate tap;

use amethyst::prelude::*;
use amethyst::ecs::prelude::*;
use amethyst_editor_sync::*;
use serde::*;
use tap::*;

fn main() -> Result<(), amethyst::Error> {
    // Create a `SyncEditorBundle` which will register all necessary systems to serialize and send
    // data to the editor.
    let editor_bundle = SyncEditorBundle::default()
        // Register the default types from the engine.
        .tap(SyncEditorBundle::sync_default_types)
        // Register the components and resources specified above.
        .tap(|bundle| sync_components!(bundle, Foo))
        .tap(|bundle| sync_resources!(bundle, BarResource));

    let _game_data = GameDataBuilder::default()
        .with_bundle(editor_bundle)?;
    Ok(())
}

// Make sure you enable serialization for your custom components and resources!
#[derive(Serialize, Deserialize)]
struct Foo {
    bar: usize,
    baz: String,
}

impl Component for Foo {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Serialize, Deserialize)]
struct BarResource {
    bar: usize,
}