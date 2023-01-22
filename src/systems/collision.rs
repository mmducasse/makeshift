use xf::num::{irect::{IRect, rect, ir}, ivec2::IVec2};

use crate::{game::game_data::GameData, consts::P16, level::tile::TileType};

/// Returns the deflection vector that results from colliding the
/// box collider with the tilemap.
pub fn collide(collider: IRect, others: Vec<IRect>, room_bounds: Option<IRect>) -> IVec2 {
    let mut collider = collider;
    let init_collider = collider;

    // Collide once and deflect either vertically or horizontally.
    let deflection = collide_axis(collider, &others, None);
    collider.pos += deflection;

    // Collide again, deflecting in the other direction.
    collider.pos += if deflection.x != 0 {
        collide_axis(collider, &others, Some(true))
    } else {
        collide_axis(collider, &others, Some(false))
    };

    // If needed, keep within the room's bounds.
    if let Some(room_bounds) = room_bounds {
        keep_inside(&mut collider, room_bounds);
    }

    collider.pos - init_collider.pos
}

/// Collides and deflects horizontally or vertically, whichever is smaller.
/// Set force axis to Some(true) for vertical only, Some(false) for horizontal only.
fn collide_axis(collider: IRect, others: &Vec<IRect>, force_axis: Option<bool>) -> IVec2 {
    let init_pos = collider.pos;
    let mut bounds = collider;
    let mut most_overlapping = None;
    let mut max_inter = IVec2::ZERO;

    for other in others {
        if let Some(inter) = bounds.intersection(*other) { // bounds.intersect(*other) {
            if (inter.w() * inter.h()) > (max_inter.x * max_inter.y) {
                max_inter = inter.size;
                most_overlapping = Some(other);
            }
        }
    }

    let do_h = max_inter.x < max_inter.y;
    if let Some(tile_bounds) = most_overlapping {
        if force_axis == Some(false) || do_h {
            if bounds.x() < tile_bounds.x() {
                bounds.pos.x -= max_inter.x
            } else {
                bounds.pos.x += max_inter.x
            }
        } else if force_axis == Some(false) || !do_h {
            if bounds.y() < tile_bounds.y() {
                bounds.pos.y -= max_inter.y
            } else {
                bounds.pos.y += max_inter.y
            }
        }
    }

    bounds.pos - init_pos
}

fn keep_inside(collider: &mut IRect, room_bounds: IRect) {
    // Horizontal
    if collider.left() < room_bounds.left() {
        collider.pos.x = room_bounds.left();
    }
    else if collider.right() > room_bounds.right() {
        collider.pos.x = room_bounds.right() - collider.w();
    }

    // Vertical
    if collider.top() < room_bounds.top() {
        collider.pos.y = room_bounds.top();
    }
    else if collider.bottom() > room_bounds.bottom() {
        collider.pos.y = room_bounds.bottom() - collider.h();
    }
}

pub fn get_colliders_near(center: IVec2, g: &GameData) -> Vec<IRect> {
    const AREA: IRect = rect(-1, -1, 3, 3);
    let mut vec = vec![];

    let pos_p16 = center / P16;

    // Get bounds of colliding tiles.
    let tilemap = &g.level.tilemap;

    for offset in AREA.iter() {
        let tile_p16_pos = pos_p16 + offset;
        if let Some(tile) = tilemap.get(tile_p16_pos) {
            if tile.type_.is_impassable() {
                let tile_bounds = ir(tile_p16_pos * P16, P16);

                vec.push(tile_bounds);
            }
        }
    }
    vec
}

pub fn is_wall_at(bounds: IRect, offset: IVec2, g: &GameData) -> bool {
    let pos = bounds.center() + offset;
    let tile_type = g.level.tile_type_at(pos);
    tile_type != TileType::Empty
}