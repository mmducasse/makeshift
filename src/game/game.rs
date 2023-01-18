use macroquad::window::next_frame;



pub async fn run() {
    // Main game loop.
    loop {
        next_frame().await;
    }
}