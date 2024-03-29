use crate::prelude::*;
use std::ops::{Add, Sub, Mul};

pub const MAIN_BUTTON_FONT_SIZE: f32 = 0.03;
pub const NORMAL_BUTTON_FONT_SIZE: f32 = 0.04;
pub const GO_BUTTON_FONT_SIZE: f32 = 0.05;
pub const LOBBY_BUTTON_FONT_SIZE: f32 = 0.03;
const EDIT_FIELD_TEXT_SIZE: f32 = 0.03;
const EDIT_FIELD_BORDER_WIDTH: f32 = 0.004;
const EDIT_FIELD_CURSOR_WIDTH: f32 = 0.002;
const EDIT_FIELD_CURSOR_BLINK_INTERVAL: u32 = 60;
pub const EXPLANATION_FONT_SIZE: f32 = 0.03;
pub const SUBTITLE_FONT_SIZE: f32 = 0.05;
pub const DEFAULT_BUTTON_COLOR: Color = Color::rgb(0.08, 0.26, 0.42);
pub const BUTTON_IMAGE_SIZE: CanvasVec = CanvasVec::new(0.04, 0.04);
pub const LIST_VIEW_HEADER_SIZE: f32 = 0.04;
pub const LIST_VIEW_CONTENT_SIZE: f32 = 0.03;
pub const LIST_VIEW_BUTTON_SIZE: f32 = 0.03;

pub struct MenuElement<B: Backend> {
	pub name: String,
	pub kind: MenuKind<B>,
	pub position: CanvasVec,
	pub size: CanvasVec,
	pub color: Color,
}

pub struct Button<B: Backend> {
	pub text: String,
	pub on_click: OnEvent<B>,
	pub font_size: f32,
	pub image: Option<TextureId>,
}

pub struct EditField {
	pub template_text: String,
	pub font_size: f32,
}

pub enum TextAlign {
	Left,
	Center,
}

pub struct Label {
	pub text: String,
	pub align: TextAlign,
	pub font_size: f32,
}

pub struct Image {
	pub texture_index: TextureIndex
}

pub struct ListView<B: Backend> {
	pub header: Vec<String>,
	pub content: Vec<Vec<String>>,
	pub on_click_events: Vec<OnEvent<B>>,
}

pub enum MenuKind<B: Backend> {
	Button(Button<B>),
	EditField(EditField),
	Label(Label),
	Image(Image),
	Panel,
}

impl<B: Backend> MenuElement<B> {
	pub fn new_button(name: String, position: CanvasVec, size: CanvasVec, text: String, color: Color, font_size: f32, image: Option<TextureId>, on_click: OnEvent<B>) -> MenuElement<B> {
		MenuElement {
			name,
			kind: MenuKind::Button(Button { text, on_click, font_size, image } ),
			position,
			size,
			color,
		}
	}

	pub fn new_edit_field(name: String, position: CanvasVec, size: CanvasVec, color: Color, template_text: &str) -> MenuElement<B> {
		MenuElement {
			name,
			kind: MenuKind::EditField( EditField::new(template_text) ),
			position,
			size,
			color,
		}
	}

	pub fn new_label(name: String, position: CanvasVec, size: CanvasVec, font_size: f32, text: &str, align: TextAlign) -> MenuElement<B> {
		MenuElement {
			name,
			kind: MenuKind::Label(Label {
				text: text.to_string(),
				align,
				font_size,
			}),
			position,
			size,
			color: Color::WHITE
		}
	}

	pub fn new_image(name: String, position: CanvasVec, size: CanvasVec, texture_index: impl IntoTextureIndex) -> MenuElement<B> {
		MenuElement {
			name,
			kind: MenuKind::Image(Image {
				texture_index: texture_index.into_texture_index(),
			}),
			position,
			size,
			color: Color::WHITE
		}
	}

	pub fn new_panel(name: String, position: CanvasVec, size: CanvasVec, color: Color) -> MenuElement<B> {
		MenuElement {
			name,
			kind: MenuKind::Panel,
			position,
			size,
			color
		}
	}

	pub fn new_list_view_elements(name: String, position: CanvasVec, size: CanvasVec, spacing: Vec<f32>, header: Vec<String>, content: Vec<Vec<String>>, opt_on_click_events: Option<Vec<OnEvent<B>>>, menu_cache: &MenuCache) -> Vec<MenuElement<B>> {
		// add header
		let mut elements = vec![
			MenuElement {
				name: format!("{}:headerpanel", name),
				kind: MenuKind::Panel,
				position: CanvasVec::new(position.x, position.y + size.y - LIST_VIEW_HEADER_SIZE/2.0),
				size: CanvasVec::new(size.x, LIST_VIEW_HEADER_SIZE),
				color: Color::rgb(0.2, 0.4, 0.6),
			}
		];
		assert_eq!(spacing.len(), header.len());
		for (i, space) in spacing.iter().enumerate() {
			elements.push(
				MenuElement {
					name: format!("{}:header{}", name, i),
					kind: MenuKind::Label(Label {
						text: header[i].clone(),
						align: TextAlign::Left,
						font_size: LIST_VIEW_HEADER_SIZE,
					}),
					position: CanvasVec::new(position.x + space, position.y + size.y - LIST_VIEW_HEADER_SIZE/2.0),
					size: CanvasVec::new(size.x, LIST_VIEW_HEADER_SIZE),
					color: Color::WHITE
				}
			)
		}

		if let Some(on_click_events) = &opt_on_click_events {
			assert_eq!(on_click_events.len(), content.len());
		}
		let max_num_lines = ((size.y*2.0 - LIST_VIEW_HEADER_SIZE) / LIST_VIEW_CONTENT_SIZE) as usize;
		let num_to_skip = menu_cache.list_view.get(&name).map(|lvc| lvc.scroll_position).unwrap_or(0) as usize;

		for row_index in 0..max_num_lines {
			let entry_index = row_index + num_to_skip;
			if let Some(line) = content.get(entry_index) {
				assert_eq!(line.len(), spacing.len());
				for (column_index, space) in spacing.iter().enumerate() {
					let content = line[column_index].clone();
					elements.push(
						MenuElement {
							name: format!("{}:content{}_{}", name, entry_index, column_index),
							kind: MenuKind::Label(Label {
								text: content,
								align: TextAlign::Left,
								font_size: LIST_VIEW_CONTENT_SIZE,
							}),
							position: CanvasVec::new(position.x + space, position.y + size.y - LIST_VIEW_HEADER_SIZE - LIST_VIEW_CONTENT_SIZE*(row_index+1) as f32),
							size: CanvasVec::new(size.x, LIST_VIEW_CONTENT_SIZE),
							color: Color::WHITE
						}
					);
				}
				elements.push(
					MenuElement {
						name: format!("{}:content_field{}", name, entry_index),
						kind: MenuKind::Button(Button {
							text: String::new(),
							on_click: opt_on_click_events.as_ref().map(|e| e[entry_index].clone()).unwrap_or(Box::new(|_, _| {})).clone(),
							font_size: LIST_VIEW_CONTENT_SIZE,
							image: None
						}),
						position: CanvasVec::new(position.x, position.y + size.y - LIST_VIEW_HEADER_SIZE - LIST_VIEW_CONTENT_SIZE*(row_index+1) as f32),
						size: CanvasVec::new(size.x, LIST_VIEW_CONTENT_SIZE/2.0),
						color: Color::rgb(0.2, 0.4, 0.6)*(0.8+((entry_index%2) as f32*0.02)),
					}
				)
			} else {
				elements.push(
					MenuElement {
						name: format!("{}:content_field_empty{}", name, entry_index),
						kind: MenuKind::Button(Button {
							text: String::new(),
							on_click: Box::new(|_, _| {}),
							font_size: LIST_VIEW_CONTENT_SIZE,
							image: None
						}),
						position: CanvasVec::new(position.x, position.y + size.y - LIST_VIEW_HEADER_SIZE - LIST_VIEW_CONTENT_SIZE*(row_index+1) as f32),
						size: CanvasVec::new(size.x, LIST_VIEW_CONTENT_SIZE/2.0),
						color: Color::rgb(0.2, 0.4, 0.6)*(0.8+((entry_index%2) as f32*0.02)),
					}
				)
			}
		}

		let name_copy = name.clone();
		let content_len = content.len();

		elements.push(
			MenuElement {
				name: format!("{}:down", name),
				kind: MenuKind::Button(Button {
					text: "v".to_string(),
					on_click: Box::new(move |app: &mut App<B>, _| {
						app.menu_cache.list_view.entry(name_copy.clone())
							.or_insert(ListViewCache { scroll_position: 0 });

						app.menu_cache.list_view.entry(name_copy.clone())
							.and_modify(|lvc| lvc.scroll_position = (lvc.scroll_position+1).min(content_len.checked_sub(1).unwrap_or(0)));
					}),
					font_size: LIST_VIEW_HEADER_SIZE,
					image: None
				}),
				position: CanvasVec::new(position.x + size.x - LIST_VIEW_BUTTON_SIZE, position.y + size.y - LIST_VIEW_HEADER_SIZE - LIST_VIEW_CONTENT_SIZE*max_num_lines as f32 + LIST_VIEW_BUTTON_SIZE - LIST_VIEW_CONTENT_SIZE/2.0),
				size: CanvasVec::new(LIST_VIEW_BUTTON_SIZE, LIST_VIEW_BUTTON_SIZE),
				color: Color::rgb(0.2, 0.4, 0.6),
			}
		);

		elements.push(
			MenuElement {
				name: format!("{}:up", name),
				kind: MenuKind::Button(Button {
					text: "^".to_string(),
					on_click: Box::new(move |app: &mut App<B>, _| {
						app.menu_cache.list_view.entry(name.clone())
							.and_modify(|lvc| lvc.scroll_position = lvc.scroll_position.checked_sub(1).unwrap_or(0))
							.or_insert(ListViewCache { scroll_position: 0 });
					}),
					font_size: LIST_VIEW_HEADER_SIZE,
					image: None
				}),
				position: CanvasVec::new(position.x + size.x - LIST_VIEW_BUTTON_SIZE, position.y + size.y - LIST_VIEW_HEADER_SIZE - LIST_VIEW_CONTENT_SIZE*max_num_lines as f32 + LIST_VIEW_BUTTON_SIZE*3.0 - LIST_VIEW_CONTENT_SIZE/2.0),
				size: CanvasVec::new(LIST_VIEW_BUTTON_SIZE, LIST_VIEW_BUTTON_SIZE),
				color: Color::rgb(0.2, 0.4, 0.6),
			}
		);

		elements
	}

	pub fn is_colliding(&self, pos: CanvasVec) -> bool {
		pos.x >= self.position.x - self.size.x && pos.x <= self.position.x + self.size.x &&
		pos.y >= self.position.y - self.size.y && pos.y <= self.position.y + self.size.y
	}

	pub fn tick(&mut self, graphics_backend: &impl GraphicsBackend, menu_cache: &mut MenuCache) {
		if let Some(edit_field_cache) = menu_cache.edit_field.get_mut(&self.name) {
			edit_field_cache.cursor_blink_counter = (edit_field_cache.cursor_blink_counter + 1) % EDIT_FIELD_CURSOR_BLINK_INTERVAL;

			// view offset
			if let MenuKind::EditField(edit_field) = &mut self.kind {
				edit_field.adapt_view(graphics_backend, self.size, edit_field_cache);
			}
		}
	}

	pub fn draw(&self, draw: &mut Draw, graphics_backend: &impl GraphicsBackend, menu_cache: &MenuCache) {
		let clicked = Some(&self.name) == menu_cache.clicked_element.as_ref();
		let hovered = Some(&self.name) == menu_cache.hovered_element.as_ref();
		let color = if clicked {
			self.color * 2.0
		} else if hovered {
			self.color * 1.5
		} else {
			self.color
		};
		match &self.kind {
			MenuKind::Button(button) => {
				self.draw_button(draw, button, color, graphics_backend)
			},
			MenuKind::EditField (edit_field) => {
				let edit_field_cache = menu_cache.edit_field.get(&self.name).unwrap();
				self.draw_edit_field(draw, edit_field, edit_field_cache, menu_cache, color, graphics_backend)
			},
			MenuKind::Label(label) => {
				self.draw_label(draw, label, color, graphics_backend)
			},
			MenuKind::Image(image) => {
				self.draw_image(draw, image)
			},
			MenuKind::Panel => {
				self.draw_panel(draw, color)
			}
		}
	}

	fn draw_button(&self, draw: &mut Draw, button: &Button<B>, color: Color, graphics_backend: &impl GraphicsBackend) {
		let Button { text, font_size, image, .. } = button;
		let left_bot = self.position - self.size;
		let right_top = self.position + self.size;
		draw.rectangle(left_bot, right_top, color);

		let mut text_pos = center_position(left_bot, right_top, graphics_backend.get_text_size(text, *font_size));
		if image.is_some() {
			text_pos.y -= 0.03;
		}
		draw.text(text_pos, *font_size, Color::WHITE, text);

		if let Some(texture_id) = image {
			let mut image_pos = center_position(left_bot, right_top, BUTTON_IMAGE_SIZE);
			image_pos.y += 0.014;
			draw.texture(image_pos, image_pos + BUTTON_IMAGE_SIZE, *texture_id, Flip::Normal, None);
		}
	}

	fn draw_edit_field(&self, draw: &mut Draw, edit_field: &EditField, edit_field_cache: &EditFieldCache, menu_cache: &MenuCache, color: Color, graphics_backend: &impl GraphicsBackend) {
		let EditField { template_text, font_size, .. } = edit_field;
		let EditFieldCache { cursor_blink_counter, cursor, view_offset, .. } = edit_field_cache;
		let selected = Some(&self.name) == menu_cache.selected_element.as_ref();
		draw.rectangle(self.position - self.size, self.position + self.size, color);
		draw.rectangle(
			self.position - self.size + EDIT_FIELD_BORDER_WIDTH,
			self.position + self.size - EDIT_FIELD_BORDER_WIDTH,
			Color::rgb(0.0, 0.03, 0.15),
		);

		let text = edit_field.get_render_text(edit_field_cache);

		let text_size = if !text.is_empty() {
			graphics_backend.get_text_size(text, *font_size)
		} else {
			graphics_backend.get_text_size(&template_text, *font_size)
		};

		let text_pos = CanvasVec::new(
			self.position.x - self.size.x + EDIT_FIELD_BORDER_WIDTH * 2.7,
			center_position(self.position.y - self.size.y, self.position.y + self.size.y, text_size.y)
		);

		if edit_field_cache.text.is_empty() {
			draw.text(text_pos, *font_size, Color::gray(0.04), &template_text);
		} else {
			draw.text(text_pos, *font_size, Color::WHITE, text);
		}

		// draw cursor
		if selected && *cursor_blink_counter < EDIT_FIELD_CURSOR_BLINK_INTERVAL / 2 {
			let subtext = &text[0..get_byte_pos(text, *cursor - view_offset)];
			let text_width = graphics_backend.get_text_size(subtext, *font_size).x;
			let left_bot = CanvasVec::new(
				self.position.x - self.size.x + text_width + EDIT_FIELD_BORDER_WIDTH * 2.0,
				self.position.y - self.size.y + EDIT_FIELD_BORDER_WIDTH * 2.0
			);
			let right_top = CanvasVec::new(
				left_bot.x + EDIT_FIELD_CURSOR_WIDTH,
				self.position.y + self.size.y - EDIT_FIELD_BORDER_WIDTH * 2.0
			);
			draw.rectangle(left_bot, right_top, Color::WHITE);
		}
	}

	fn draw_label(&self, draw: &mut Draw, label: &Label, color: Color, graphics_backend: &impl GraphicsBackend) {
		let Label { text, font_size, align, .. } = label;

		let text_size = graphics_backend.get_text_size(text, *font_size);
		let text_pos = match align {
			TextAlign::Left => CanvasVec::new(
				self.position.x - self.size.x,
				center_position(self.position.y - self.size.y, self.position.y + self.size.y, text_size.y)
			),
			TextAlign::Center => center_position::<CanvasVec>(self.position - self.size, self.position + self.size, text_size),
		};

		draw.text(text_pos, *font_size, color, text);
	}

	fn draw_image(&self, draw: &mut Draw, image: &Image) {
		let Image { texture_index } = *image;
		let radius = self.size / 2.0;
		let left_bot = self.position - radius;
		let right_top = self.position + radius;
		draw.texture(left_bot, right_top, texture_index, Flip::Normal, None);
	}

	fn draw_panel(&self, draw: &mut Draw, color: Color) {
		let left_bot = self.position - self.size;
		let right_top = self.position + self.size;
		draw.rectangle(left_bot, right_top, color);
	}

	pub fn apply_text(&mut self, event_text: &[Character], edit_field: &mut EditFieldCache) {
		let EditFieldCache { text, cursor, cursor_blink_counter, .. } = edit_field;
		for character in event_text {
			match character {
				Character::Char(c) => {
					text.insert(get_byte_pos(text, *cursor), *c);
					*cursor += 1;
					*cursor_blink_counter = 0;
				},
				Character::Backspace => {
					if *cursor != 0 {
						text.drain(get_byte_pos(text, *cursor - 1)..get_byte_pos(text, *cursor));
						*cursor = (*cursor - 1).max(0);
					}
					*cursor_blink_counter = 0;
				},
				Character::Delete => {
					if *cursor < text.chars().count() {
						text.drain(get_byte_pos(text, *cursor)..get_byte_pos(text, *cursor + 1));
					}
					*cursor_blink_counter = 0;
				},
				_ => {},
			}
		}
	}

	pub fn apply_key_events(&mut self, peripherals_state: &PeripheralsState, edit_field: &mut EditFieldCache) {
		let EditFieldCache { cursor, text, cursor_blink_counter, .. } = edit_field;
		if peripherals_state.key_firing(Key::Left) {
			*cursor = cursor.checked_sub(1).unwrap_or(0);
			*cursor_blink_counter = 0;
		}
		if peripherals_state.key_firing(Key::Right) {
			*cursor = (*cursor + 1).min(text.len());
			*cursor_blink_counter = 0;
		}
	}
}

impl EditField {
	fn new(template_text: &str) -> EditField {
		EditField {
			template_text: String::from(template_text),
			font_size: EDIT_FIELD_TEXT_SIZE,
		}
	}

	fn adapt_view(&self, graphics_backend: &impl GraphicsBackend, size: CanvasVec, edit_field: &mut EditFieldCache) {
		let allowed_width = size.x * 2.0 - EDIT_FIELD_BORDER_WIDTH * 2.0;

		// if text is not right aligned -> decrease view offset
		while graphics_backend.get_text_size(self.get_text_post_view_offset(edit_field), self.font_size).x <= allowed_width - 0.03 {
			if edit_field.view_offset == 0 {
				break;
			}
			edit_field.view_offset -= 1;
		}

		// test if cursor is left out of view range
		// -> decrease view offset
		if edit_field.cursor < edit_field.view_offset {
			edit_field.view_offset = edit_field.cursor;
		} else {
			// test if cursor is right out of view range
			// -> increase view offset
			while self.get_cursor_render_offset(edit_field, graphics_backend) > allowed_width {
				edit_field.view_offset += 1;
			}
		}


		// view limit
		edit_field.view_limit = edit_field.view_limit.min(edit_field.text.len());
		while graphics_backend.get_text_size(self.get_render_text(edit_field), self.font_size).x <= allowed_width {
			if edit_field.view_limit == edit_field.text.len() {
				break;
			}
			edit_field.view_limit += 1;
		}
		while graphics_backend.get_text_size(self.get_render_text(edit_field), self.font_size).x > allowed_width {
			if edit_field.view_limit == 0 {
				break;
			}
			edit_field.view_limit -= 1;
		}
	}

	fn get_render_text<'a>(&self, edit_field: &'a EditFieldCache) -> &'a str {
		&edit_field.text[get_byte_pos(&edit_field.text, edit_field.view_offset)..get_byte_pos(&edit_field.text, edit_field.view_limit)]
	}

	fn get_cursor_render_offset(&self, edit_field_cache: &EditFieldCache, graphics_backend: &impl GraphicsBackend) -> f32 {
		graphics_backend.get_text_size(self.get_pre_cursor_text(edit_field_cache), self.font_size).x
	}

	// text after view offset but before cursor
	fn get_pre_cursor_text<'a>(&self, edit_field_cache: &'a EditFieldCache) -> &'a str {
		&edit_field_cache.text[get_byte_pos(&edit_field_cache.text, edit_field_cache.view_offset)..get_byte_pos(&edit_field_cache.text, edit_field_cache.cursor)]
	}

	fn get_text_post_view_offset<'a>(&self, edit_field: &'a EditFieldCache) -> &'a str {
		&edit_field.text[get_byte_pos(&edit_field.text, edit_field.view_offset)..]
	}
}

fn center_position<T>(outer_left: T, outer_right: T, inner_size: T) -> T
	where T: Add<Output=T> + Sub<Output=T> + Mul<f32, Output=T> + Copy
{
	let outer_size = outer_right - outer_left;
	let space = (outer_size - inner_size) * 0.5;
	outer_left + space
}

fn get_byte_pos(text: &str, char_pos: usize) -> usize {
	text.char_indices()
		.nth(char_pos)
		.map(|(i, _)| i)
		.unwrap_or(text.len())
}

// OnEvent impl
impl<B: Backend, F: Fn(&mut App<B>, &mut Runnable<B>) + Clone + 'static> OnEventImpl<B> for F {
	fn clone_box(&self) -> Box<dyn OnEventImpl<B>> {
		Box::new(self.clone())
	}
}

impl<B: Backend> Clone for Box<dyn OnEventImpl<B>> {
	fn clone(&self) -> Self { (**self).clone_box() }
}
