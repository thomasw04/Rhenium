use window::Window;

mod window;

fn main() {
    let window = Window::new("RustyBear-Engine", 512, 512);

    window.run( || {});
}
