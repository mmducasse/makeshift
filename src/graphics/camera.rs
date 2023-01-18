use xf::num::{irect::IRect, ivec2::IVec2};


/// Returns the top-left position of the view as it attempts to stay centereed
/// at `desired_center`.
pub fn follow(desired_center: IVec2, view_size: IVec2, scene_bounds: IRect) -> IVec2 {
    let desired_view = IRect::centered_at(desired_center, view_size);

    desired_view.keep_inside(scene_bounds).pos
}