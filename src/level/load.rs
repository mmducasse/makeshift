use std::{rc::Rc, str::FromStr};

use macroquad::texture::Image;
use xf::{map::{tiled_json::{tileset::{JsonTileset, JsonTile}, tilemap::{JsonTilemap, Object, Layer}}, tileset::Tileset, tilemap::Tilemap}, num::ivec2::i2};

use crate::{graphics::buffer::convert_mq_image_to_xf_texture, entities::{player::player::Player, entity::Entity, entities::Entities, bosses::boss::boss::Boss}};

use super::{level_info::LevelId, level::Level, tile::{Tile, TileType}};



pub fn load_level(level_id: LevelId) -> Result<(Level, Entities), String> {
    let level_info = level_id.info();
    let tilemap_info = level_info.tilemap_info;
    let tileset_info = tilemap_info.tileset_info;

    // Load tileset image.
    let tileset_image = tileset_info.image;
    let image = Image::from_file_with_format(tileset_image, None);
    let texture = convert_mq_image_to_xf_texture(&image);

    // Load tileset JSON and convert to a Tileset.
    let tileset = tileset_info.tileset;
    let tileset_json: JsonTileset = serde_json::from_slice(tileset).or_else(|e| {
        Err(format!("Load Tileset JSON: {}",e.to_string()))
    })?;

    let tileset: Tileset<Tile> = Tileset::from_json(&tileset_json, texture, load_tile).or_else(|e| {
        Err(format!("Convert Tileset JSON: {}",e.to_string()))
    })?;

    // Load tilemap JSON and convert to a Tilemap.
    let tilemap_json: JsonTilemap = serde_json::from_slice(tilemap_info.tilemap).or_else(|e| {
        Err(format!("Load Tilemap JSON: {}",e.to_string()))
    })?;
    let tilemap_layers: Vec<Tilemap<Tile>> = Tilemap::from_json(&tilemap_json, Rc::new(tileset)).or_else(|e| {
        Err(format!("Convert Tilemap JSON: {}",e.to_string()))
    })?;
    let tilemap = tilemap_layers.into_iter().nth(0).unwrap();

    let entities = load_entities(&tilemap_json);

    // Create level.
    Ok((Level::new(tilemap), entities))
}

fn load_tile(json_tile: &JsonTile) -> Result<Tile, String> {
    Ok(if let Some(type_) = &json_tile.type_ {
        Tile {
            type_: TileType::from_str(type_).unwrap(),
            //frames: get_tile_property(json_tile, "Frames").map(|s| u64::from_str(s).unwrap()),
        }
    } else {
        Tile::default()
    })
}

fn get_tile_property<'a>(json_tile: &'a JsonTile, name: &str) -> Option<&'a str> {
    if let Some(properties) = &json_tile.properties {
        for property in properties {
            if property.name == name {
                return Some(&property.value)
            }
        }
    }
    
    None
}

fn load_entities(json: &JsonTilemap) -> Entities {
    let mut entities = Entities::new();

    for layer in &json.layers {
        if let Layer::Objectgroup { objects, .. } = layer {
            for object in objects {
                let entity = load_entity(&object);
                entities.add(entity);
            }
        }
    }

    entities
}

fn load_entity(object: &Object) -> Box<dyn Entity> {
    let pos = i2(object.x, object.y);
    match object.name.as_str() {
        "Player" => Box::new(Player::new(pos)),
        "Boss" => Box::new(Boss::new(pos)),
        _ => panic!("Unexpected object name: {}", object.name),
    }
}