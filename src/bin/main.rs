use ndarray::Array2;
use num_traits::cast::FromPrimitive;
use palette::{LinSrgba, Srgba};
use pixels::{Pixels, SurfaceTexture};
use rand::random;
use serde::{Deserialize, Serialize};
use structopt::StructOpt;
use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

#[derive(StructOpt)]
struct Cli {
    /// Set a custom int
    #[structopt(short = "i", long = "integer")]
    num: i32,

    /// Set a custom boolean
    #[structopt(short = "b", long = "boolean")]
    flag: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct Parameters {
    resolution: (u32, u32),
}

struct Image {
    pub data: Array2<LinSrgba>,
}
impl Image {
    /// Construct a new image.
    pub fn new(width: usize, height: usize) -> Self {
        let data = Array2::default((width, height));
        Self { data }
    }

    /// Get the resolution of the image.
    pub fn resolution(&self) -> (usize, usize) {
        let shape = self.data.shape();
        (shape[0], shape[1])
    }

    /// Process the image.
    pub fn process(&mut self) {
        let x = random::<usize>() % self.resolution().0;
        let y = random::<usize>() % self.resolution().1;
        self.data[[x, y]] = LinSrgba::new(1.0, 1.0, 1.0, 1.0);
    }
}

fn main() {
    let args = Cli::from_args();
    println!("num: {}, flag: {}", args.num, args.flag);

    let json_str = r#"
        {
            "resolution": [192, 108]
        }
    "#;

    let p: Parameters = serde_json::from_str(json_str).unwrap();
    let width = p.resolution.0 as usize;
    let height = p.resolution.1 as usize;

    let event_loop = EventLoop::new();
    let window = {
        let size = LogicalSize::new(width as f64, height as f64);
        WindowBuilder::new()
            .with_title("Photo")
            .with_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let window_size = window.inner_size();
    let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
    let mut pixels = Pixels::new(
        window_size.width as u32,
        window_size.height as u32,
        surface_texture,
    )
    .unwrap();

    let mut img = Image::new(width, height);
    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::RedrawRequested(_) => {
                update_display_buffer(&mut pixels, &img);
                if pixels.render().is_err() {
                    *control_flow = ControlFlow::Exit;
                    return;
                }
            }
            Event::WindowEvent {
                event: WindowEvent::Resized(new_size),
                ..
            } => pixels
                .resize_surface(new_size.width, new_size.height)
                .unwrap(),
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit;
                return;
            }
            _ => (),
        }

        img.process();

        window.request_redraw();
    });
}

fn update_display_buffer(pixels: &mut Pixels, img: &Image) {
    let frame: &mut [u8] = pixels.frame_mut();

    let resolution = img.resolution();
    let width = resolution.0;
    let height = resolution.1;

    for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
        let x = i % height;
        let y = i / width;

        let col = img.data[[x, y]];
        let slice = rgba_slice(col);
        pixel.copy_from_slice(&slice);
    }
}

fn rgba_slice(col: LinSrgba<f32>) -> [u8; 4] {
    let rgba: Srgba = Srgba::from_linear(col.into());
    let (r, g, b, a) = rgba.into_components();
    [
        u8::from_f32(r * 255.0).unwrap_or(0),
        u8::from_f32(g * 255.0).unwrap_or(0),
        u8::from_f32(b * 255.0).unwrap_or(0),
        u8::from_f32(a * 255.0).unwrap_or(0),
    ]
}
