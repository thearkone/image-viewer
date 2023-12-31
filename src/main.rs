mod cli;

use cli::Cli;

use clap::Parser;
use fltk::app::App;
use fltk::enums::FrameType;
use fltk::enums::{Color, Event, Key};
use fltk::frame::Frame;
use fltk::image::{BmpImage, GifImage, IcoImage, JpegImage, PngImage, SvgImage};
use fltk::prelude::{GroupExt, ImageExt, WidgetBase, WidgetExt, WindowExt};
use fltk::window::Window;
use std::path::Path;
use std::process;

fn main() {
	let cli_args = Cli::parse();
	let path = Path::new(&cli_args.file_path);

	if let Ok(res) = path.try_exists() {
		if !res {
			eprintln!("File is not existed");
			process::exit(1);
		}
	} else {
		eprintln!("File is not existed");
		process::exit(1);
	}

	let file_extension = path.extension().unwrap().to_str().unwrap();
	let file_name = path.file_name().unwrap().to_os_string().into_string().unwrap();

	let (screen_width, screen_height) = fltk::app::screen_size();

	let mut image: Box<dyn ImageExt> = match file_extension {
		"jpg" | "jpeg" => Box::new(JpegImage::load(path).unwrap()),
		"png" => Box::new(PngImage::load(path).unwrap()),
		"gif" => Box::new(GifImage::load(path).unwrap()),
		"bmp" => Box::new(BmpImage::load(path).unwrap()),
		"ico" => Box::new(IcoImage::load(path).unwrap()),
		"svg" => Box::new(SvgImage::load(path).unwrap()),
		_ => {
			eprintln!("File is not supported");
			process::exit(1);
		},
	};

	let window_width = image.width().min(screen_width as i32);
	let window_height = image.height().min(screen_height as i32);
	let window_title: &'static str = Box::leak(file_name.into_boxed_str());

	let app = App::default();
	let mut window = Window::new(0, 0, window_width, window_height, window_title).center_screen();

	let mut frame = Frame::default().with_size(window_width, window_height);
	frame.set_frame(FrameType::NoBox);

	frame.draw(move |f| {
		image.scale(f.w(), f.h(), true, false);

		let image_width = image.width();
		let image_height = image.height();

		let frame_width = f.width();
		let frame_height = f.height();

		let mut x = 0;
		let mut y = 0;

		if image_width < frame_width {
			x = (frame_width - image_width) / 2;
		}

		if image_height < frame_height {
			y = (frame_height - image_height) / 2;
		}

		image.draw(x, y, frame_width, frame_height);
	});

	window.handle(|win, event| {
		match event {
			Event::KeyUp => {
				let key = fltk::app::event_key();

				match key {
					Key::Escape => {
						win.hide();
						true
					},
					_ => {
						let char = key.to_char().unwrap();

						match char {
							'q' => {
								win.hide();
								true
							},
							_ => false,
						}
					},
				}
			},
			_ => false,
		}
	});

	window.set_color(Color::from_rgb(255, 255, 255));
	window.make_resizable(true);
	window.end();
	window.show();

	app.run().unwrap();
}
