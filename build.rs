use std::process::{Command, exit};
use std::io::ErrorKind;

fn compile_shader(shader_name: &str) {
	let glsl_path = format!("res/shader/{}{}", shader_name, ".glsl");
	let spv_path = format!("res/shader/{}{}", shader_name, ".spv");

	println!("cargo:rerun-if-changed={}", glsl_path);
	// this line is usefull but unfortunately forces linkage of the app
	// println!("{}{}", "cargo:rerun-if-changed=", spv_path);

	let status = Command::new("glslangValidator")
		.arg("-V")
		.arg(glsl_path)
		.arg("-o").arg(spv_path)
		.status();

	let status = match status.map_err(|e| e.kind()) {
		Err(ErrorKind::NotFound) => {
			println!("cargo:warning=glslangValidator not installed. Skipping shader compilation.");
			exit(0)
		},
		Err(e) => Err(e).unwrap(),
		Ok(exit_status) => exit_status,
	};

	match status.code() {
		Some(code) => if code != 0 { exit(code); },
		None => if !status.success() { exit(1); },
	}
}

fn main() {
	compile_shader("triangles.vert");
	compile_shader("triangles.frag");
	compile_shader("tilemap.vert");
	compile_shader("tilemap.frag");
	compile_shader("fluidmap.vert");
	compile_shader("fluidmap.frag");
}
