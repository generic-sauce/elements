use crate::prelude::*;

pub trait OnEventImpl<B: Backend>: Fn(&mut App<B>, &mut Runnable<B>) {
	fn clone_box(&self) -> Box<dyn OnEventImpl<B>>;
}

pub type OnEvent<B> = Box<dyn OnEventImpl<B>>;

pub fn create_local<B: Backend>(best_of_n: u32) -> OnEvent<B> {
	Box::new(move |_app, runnable| {
		*runnable = Runnable::Local(Local::new(best_of_n));
	})
}

pub fn create_server_connector<B: Backend>(_app: &mut App<B>, _runnable: &mut Runnable<B>) {
	unimplemented!()
}

pub fn unpause<B: Backend>(_app: &mut App<B>, runnable: &mut Runnable<B>) {
	runnable.toggle_active();
}

pub fn create_online_menu<B: Backend>(_app: &mut App<B>, runnable: &mut Runnable<B>) {
	*runnable = Runnable::OnlineMenu(OnlineMenu::new());
}

pub fn create_local_menu<B: Backend>(_app: &mut App<B>, runnable: &mut Runnable<B>) {
	*runnable = Runnable::LocalMenu;
}

pub fn create_tutorial_menu<B: Backend>(_app: &mut App<B>, runnable: &mut Runnable<B>) {
	*runnable = Runnable::TutorialMenu;
}
