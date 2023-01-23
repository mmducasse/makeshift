
use xf::{
    num::{ivec2::{IVec2, i2}, 
    irect::{ir, rect, IRect}}, 
    gl::{bitmap::Bitmap}
};

use crate::{graphics::{textures::TextureId}, game::draw_data::DrawData};


const CHAR_SPACING: IVec2 = i2(6, 8);
const CHAR_BOUNDS: IRect = rect(0, 1, 6, 6);

pub fn draw_text(s: &str, org: IVec2, center: bool, d: &mut DrawData) {
    let texture = d.textures().get(TextureId::Text);

    let mut pos = org;
    
    if center {
        let w = (s.len() as i32) * CHAR_SPACING.x;
        pos.x -= w / 2;

        //draw_rect(ir(pos - i2(1, 1), i2(w, CHAR_SPACING.y) + i2(1, 1)), Color::WHITE)
    }

    for c in s.chars() {
        let src_pos = lookup(c);
        let src = ir(src_pos + CHAR_BOUNDS.pos, CHAR_BOUNDS.size);
        d.buffer().draw_texture(&texture, src, pos);

        pos.x += CHAR_SPACING.x;
    }
}

fn lookup(c: char) -> IVec2 {
    fn rel(c1: char, c2: char) -> usize {
        c1 as usize - c2 as usize
    }

    let src_pos = match c {
        _ if c.is_uppercase() => {
            let idx = rel(c, 'A');
            let x = idx % 13;
            let y = idx / 13;
            i2(x as i32, y as i32)
        },
        _ if c.is_lowercase() => {
            let idx = rel(c, 'a');
            let x = idx % 13;
            let y = idx / 13;
            i2(x as i32, y as i32)
        },
        _ if c.is_numeric() => i2(rel(c, '0') as i32, 4),
        '.' => i2(0, 5),
        ',' => i2(1, 5),
        '!' => i2(2, 5),
        '?' => i2(3, 5),
        ':' => i2(4, 5),
        ';' => i2(5, 5),
        '$' => i2(6, 5),
        '(' => i2(7, 5),
        ')' => i2(8, 5),
        ' ' => i2(9, 5),
        '^' => i2(4, 6),
        '>' => i2(5, 6),
        '@' => i2(6, 6),
        '<' => i2(7, 6),
        _ => i2(3, 5),
    };

    src_pos * CHAR_SPACING
}