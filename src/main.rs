use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::io::{self, BufRead};

mod TCLNomicon;
use crate::TCLNomicon::TCLNomiconState;

mod systems;
mod audio;

use amethyst::{
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend, 
        RenderingBundle,
    },
    core::transform::TransformBundle,
    utils::application_root_dir,
    input::{InputBundle, StringBindings},
};
use amethyst::ui::{RenderUi, UiBundle};
use amethyst::audio::{AudioBundle,DjSystemDesc};

use crate::audio::Music;

fn main() -> amethyst::Result<()> {
    // setup a Logger
    amethyst::start_logger(Default::default());

    // load display config
    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("config").join("display.ron");

    // load config data (input bundle)
    let binding_path = app_root.join("config").join("bindings.ron");
    let input_bundle = InputBundle::<StringBindings>::new()
        .with_bindings_from_file(binding_path)?;
    // create an application
    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with_bundle(AudioBundle::default())?
        .with_system_desc(
            DjSystemDesc::new(|music: &mut Music| music.music.next()),
            "dj_system",
            &[],
        )
        .with(systems::PaddleSystem, "paddle_system", &["input_system"])
        .with(systems::MoveBallsSystem, "ball_system",&[])
        .with(systems::BounceSystem, "collision_system", &["paddle_system", "ball_system"])
        .with(systems::WinnerSystem, "winer_system", &["ball_system"])
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
            // RenderToWindow plugin provides acaffolding for opning window and draw on it
            .with_plugin(
                RenderToWindow::from_config_path(display_config_path)?
                .with_clear([0.0, 0.0, 0.0, 1.0]),
            )
            //RenderFlat2D plugin is used to render entities with a SpriteRender component
            .with_plugin(RenderFlat2D::default())    
            .with_plugin(RenderUi::default()),
        )?;
        
    let assets_dir = app_root.join("assets");
    let mut game = Application::new(assets_dir, TCLNomiconState::default(), game_data)?;
    game.run();
    
    Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
struct Enchantment {
    name: String,
    spell: String,
}

fn main_old() {
    let mut book = Vec::new();

    match std::fs::read_to_string("book.json") {
        Err(_) => println!("No book.json found!"),
        Ok(s) => book = serde_json::from_str(&s).unwrap(),
    };
    

    let name = "Pepe".to_string();
    let spell = "avelina".to_string();
    let ench = Enchantment {name, spell};
    book.push(ench);

    book.push(Enchantment{name:"julio".to_string(), spell: "caesar".to_string()});

    println!("Spells in the book {}:", book.len());
    for s in 0..book.len() {
        println!("{:?}", book[s]);
    }

    let serialized = serde_json::to_string(&book).unwrap();
    println!("The serialization looks like {}", serialized);

    let path = Path::new("book.json");
    let display = path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    // Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`
    match file.write_all(serialized.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("successfully wrote to {}", display),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
