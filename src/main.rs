use std::path::Path;
use foreign_types::ForeignTypeRef;
use metal::{CAMetalLayer, CoreAnimationLayerRef};
use pathfinder_canvas::{Canvas, CanvasFontContext, CanvasRenderingContext2D, Path2D, TextAlign, Vector2I, FillStyle, ColorU, Vector2F};
use pathfinder_color::ColorF;
use pathfinder_geometry::vector::{vec2f, vec2i};
use pathfinder_geometry::rect::RectF;
use pathfinder_metal::MetalDevice;
use pathfinder_renderer::concurrent::rayon::RayonExecutor;
use pathfinder_renderer::concurrent::scene_proxy::SceneProxy;
use pathfinder_renderer::gpu::options::{DestFramebuffer, RendererMode, RendererOptions};
use pathfinder_renderer::gpu::renderer::Renderer;
use pathfinder_renderer::options::BuildOptions;
use pathfinder_resources::embedded::EmbeddedResourceLoader;
use font_kit::font::Font;
use sdl2::event::Event;
use sdl2::hint;
use sdl2::keyboard::Keycode;
use sdl2_sys::SDL_RenderGetMetalLayer;

mod session;
use session::Session;
use xi_core_lib::ViewId;

fn main() {
    // Set up SDL2.
    assert!(hint::set("SDL_RENDER_DRIVER", "metal"));
    let sdl_context = sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();

    // Open a window.
    let window_size = vec2i(640, 480);
    let window = video.window("Minimal example", window_size.x() as u32, window_size.y() as u32)
                      .opengl()
                      .build()
                      .unwrap();

    // Create a Metal context.
    let canvas = window.into_canvas().present_vsync().build().unwrap();
    let metal_layer = unsafe {
        CoreAnimationLayerRef::from_ptr(SDL_RenderGetMetalLayer(canvas.raw()) as *mut CAMetalLayer)
    };
    let metal_device = metal_layer.device();
    let drawable = metal_layer.next_drawable().unwrap();

    // Create a Pathfinder renderer.
    let device = unsafe {
        MetalDevice::new(metal_device, drawable.clone())
    };
    let mode = RendererMode::default_for_device(&device);
    let options = RendererOptions {
        dest: DestFramebuffer::full_window(window_size),
        background_color: Some(ColorF::white()),
        ..RendererOptions::default()
    };
    let mut renderer = Renderer::new(device, &EmbeddedResourceLoader, mode, options);

    // Make a canvas. We're going to draw a text editor.
    let canvas = Canvas::new(window_size.to_f32());
    let mut canvas = canvas.get_context_2d(CanvasFontContext::from_system_source());

    let front_end = XiPathFrontend::new_with_pathfinder_renderer();

    // arrange xi-editor backend
    let mut backend_session = Session::new(Box::new(front_end));

    // create a new view
    let view_id = create_view(&mut backend_session);

    // draw our editor on the canvas
    draw(&mut canvas, &window_size);

    // Render the canvas to screen.
    let mut scene = SceneProxy::from_scene(canvas.into_canvas().into_scene(),
                                           renderer.mode().level,
                                           RayonExecutor);
    scene.build_and_render(&mut renderer, BuildOptions::default());
    renderer.device().present_drawable(drawable);

    // Wait for a keypress.
    let mut event_pump = sdl_context.event_pump().unwrap();
    loop {
        match event_pump.wait_event() {
            Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => return,
            //todo insert
            Event::KeyDown { keycode: Some(key), .. } => backend_session.insert(&view_id, key),
            _ => {}
        }
    }
}

fn draw(canvas : &mut CanvasRenderingContext2D, window_size: &Vector2I) {
    let gutter_size = draw_line_gutter(canvas, &window_size);
    draw_sample_lines(canvas, &gutter_size);
}

fn draw_sample_lines(canvas: &mut CanvasRenderingContext2D, gutter_size: &Vector2F) {
    set_line_text_style(canvas);
    let padding = 4_f32;
    let line_margin = gutter_size.x() + padding;

    for line in 1..11 {
        let line_bottom = gutter_size.y() * (line as f32); 
        canvas.fill_text("System.Console.WriteLine(\"Hello World!\");", vec2f(line_margin, line_bottom));
    }

    for line in 11..21 {
        let line_bottom = gutter_size.y() * (line as f32); 
        canvas.fill_text("System.Console.WriteLine(\"Hello World!\");", vec2f(line_margin, line_bottom));
    }
}

fn draw_line_gutter(canvas: &mut CanvasRenderingContext2D, canvas_size: &Vector2I) -> Vector2F {
    set_gutter_text_style(canvas);
    
    // we need metrics for the height of the chosen gutter text style - so we insert a 0 (numbers dont descened, and they take up full vertical space, so any is fine)
    let single_digit_metrics = canvas.measure_text("0");
    let line_height = single_digit_metrics.em_height_ascent();

    //get total lines the view can fit in entirety, drop the remainder by forcing to i32, add 1 line to be partially rendered at bottom
    let lines = (canvas_size.y() as f32 / line_height) as i32 + 1;
    
    let left_margin = 0_f32;
    
    // we use the iterator for line number text, so start at 1 and add 1 the inclusive end
    for number in 1..(lines +1) as i32 {        
        let line_bottom = line_height * number as f32;
        canvas.fill_text(&number.to_string(), vec2f(left_margin, line_bottom))
    }
    
    //measure the text of the last line number (lines in 10s, 100s, 1000s range have diff width)
    let max_digit_metrics = canvas.measure_text(&lines.to_string());
    return vec2f(max_digit_metrics.width(), line_height);
}

fn set_line_text_style(canvas: &mut CanvasRenderingContext2D)
{
    let path = Path::new("/Users/nickspagnola/Library/Fonts/Roboto Mono for Powerline.ttf");
    
    let font = match Font::from_path(path, 0) {
        Err(e) => panic!("{}", e),
        Ok(f) => f,
    };

    canvas.set_font(font);
    canvas.set_font_size(14.0);
    canvas.set_fill_style(FillStyle::Color(ColorU::black()));
}

fn set_gutter_text_style(canvas: &mut CanvasRenderingContext2D)
{
    let path = Path::new("/Users/nickspagnola/Library/Fonts/Roboto Mono Light for Powerline.ttf");
    let font = match Font::from_path(path, 0) {
        Err(e) => panic!("{}", e),
        Ok(f) => f,
    };

    canvas.set_font(font);
    canvas.set_font_size(14.0);
    canvas.set_fill_style(FillStyle::Color(ColorU::new(170, 170, 170, 255)));
}

fn print_text_metrics(canvas: &mut CanvasRenderingContext2D, text: &str) {
    let metrics = canvas.measure_text(text);
    println!("** Text: {}", text);

    println!(" actual_bounding_box_ascent: {}", metrics.actual_bounding_box_ascent());
    println!(" actual_bounding_box_descent: {}", metrics.actual_bounding_box_descent());
    println!(" actual_bounding_box_left: {}", metrics.actual_bounding_box_left());
    println!(" actual_bounding_box_right: {}", metrics.actual_bounding_box_right());
    println!(" alphabetic_baseline: {}", metrics.alphabetic_baseline());
    println!(" em_height_ascent: {}", metrics.em_height_ascent());
    println!(" em_height_descent: {}", metrics.em_height_descent());
    println!(" font_bounding_box_ascent: {}", metrics.font_bounding_box_ascent());
    println!(" font_bounding_box_descent: {}", metrics.font_bounding_box_descent());
    println!(" hanging_baseline: {}", metrics.hanging_baseline());
    println!(" ideographic_baseline: {}", metrics.ideographic_baseline());
    println!(" text_x_offset: {}", metrics.text_x_offset());
    println!(" text_y_offset: {}", metrics.text_y_offset());
    println!(" width: {}", metrics.width());
}

fn insert(session: &mut Session, view_id: &ViewId) {

}

//todo optional path
fn create_view(session: &mut Session) -> ViewId {
    return match session.add_new_view(None) {
        Ok(vid) => vid,
        Err(e) => {
            panic!("{}", e);
        }
    };
}