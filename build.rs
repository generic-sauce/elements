use std::process::{Command, Stdio};

fn main() {
	// println!("cargo:rerun-if-changed=res/shader/render_vertex.glsl");

	Command::new("glslangValidator")
		.arg("-V")
		.arg("res/shader/render.vert")
		.arg("-o")
		.arg("res/shader/render.vert.spv")
		.spawn()
		.expect("failed to compile glsl shader to spirv");

	Command::new("glslangValidator")
		.arg("-V")
		.arg("res/shader/render.frag")
		.arg("-o")
		.arg("res/shader/render.frag.spv")
		.stdout(Stdio::inherit())
		.spawn()
		.expect("failed to compile glsl shader to spirv");
}
