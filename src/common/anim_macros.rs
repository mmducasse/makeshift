
// #[macro_export]
// macro_rules! row {
//     ($len:expr, $dur:expr, $origin:expr) => {
//         {
//             use xf::gl::anim::{Seq, seq_row};

//             const LEN: usize = $len;
//             const C: Seq<LEN> = seq_row::<LEN>($dur, $origin, false);
//             &C
//         }
//     };
// }

// #[macro_export]
// macro_rules! row_l {
//     ($len:expr, $dur:expr, $origin:expr) => {
//         {
//             use xf::gl::anim::{Seq, seq_row};

//             const LEN: usize = $len;
//             const C: Seq<LEN> = seq_row::<LEN>($dur, $origin, true);
//             &C
//         }
//     };
// }


#[macro_export]
macro_rules! row_2_l {
    ($dir:ident, $len:expr, $dur:expr, $origin:expr) => {
        {
            use xf::data::dir_h::DirH;
            use xf::num::ivec2::i2;
            use xf::gl::anim::{Seq, seq_row};

            match $dir {
                DirH::L => {
                    const LEN: usize = $len;
                    const C: Seq<LEN> = seq_row::<LEN>($dur, i2($origin.x, $origin.y + 0), true);
                    &C
                },
                DirH::R => {
                    const LEN: usize = $len;
                    const C: Seq<LEN> = seq_row::<LEN>($dur, i2($origin.x, $origin.y + 1), true);
                    &C
                },
            }
        }
    };
}

