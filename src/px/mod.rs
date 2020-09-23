use crate::prelude::*;

const SCREEN_WIDTH: u32 = 400;
const SCREEN_HEIGHT: u32 = 300;

pub fn create_window(
	event_loop: &pxp::EventLoop<()>,
) -> pxp::Window {
	// Create a hidden window so we can estimate a good default window size
	let window = winit::window::WindowBuilder::new()
		.with_visible(false)
		.with_title("Elements 2")
		.build(&event_loop)
		.unwrap();
	let hidpi_factor = window.scale_factor();

	// Get dimensions
	let width = SCREEN_WIDTH as f64;
	let height = SCREEN_HEIGHT as f64;
	let (monitor_width, monitor_height) = {
		let size = window.current_monitor().size();
		(
			size.width as f64 / hidpi_factor,
			size.height as f64 / hidpi_factor,
		)
	};
	let scale = (monitor_height / height * 2.0 / 3.0).round();

	// Resize, center, and display the window
	let min_size: winit::dpi::LogicalSize<f64> =
		pxp::PhysicalSize::new(width, height).to_logical(hidpi_factor);
	let default_size = pxp::LogicalSize::new(width * scale, height * scale);
	let center = pxp::LogicalPosition::new(
		(monitor_width - width * scale) / 2.0,
		(monitor_height - height * scale) / 2.0,
	);
	window.set_inner_size(default_size);
	window.set_min_inner_size(Some(min_size));
	window.set_outer_position(center);
	window.set_visible(true);

	window
}