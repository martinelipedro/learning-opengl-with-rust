extern crate glfw;
use self::glfw::{Context, Key, Action};


extern crate gl;
use std::sync::mpsc::Receiver;


fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));


    let (mut window, events) = glfw.create_window(800, 600, "Hello GLFW", glfw::WindowMode::Windowed)
        .expect("Failed to create glfw Window!");
    
    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);


    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);


    while !window.should_close()
    {
        process_events(&mut window, &events);

        window.swap_buffers();
        glfw.poll_events();
    }

}


fn process_events(window: &mut glfw::Window, events: &Receiver<(f64, glfw::WindowEvent)>) {
    for (_, event) in glfw::flush_messages(events) {
        match event {
            glfw::WindowEvent::FramebufferSize(width, height) => {
                // make sure the viewport matches the new window dimensions; note that width and
                // height will be significantly larger than specified on retina displays.
                unsafe { gl::Viewport(0, 0, width, height) }
            }
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
            _ => {}
        }
    }
}