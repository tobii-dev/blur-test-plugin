use blur_plugins_core::{BlurAPI, BlurEvent, BlurPlugin};
use log::LevelFilter;
use simplelog::{
	ColorChoice, CombinedLogger, Config, ConfigBuilder, TermLogger, TerminalMode, WriteLogger,
};

#[repr(C)]
struct MyDummyPlugin {}

impl BlurPlugin for MyDummyPlugin {
	fn name(&self) -> &'static str {
		"MyDummyPlugin"
	}

	fn on_event(&self, event: &BlurEvent) {
		log::trace!("Event!!! {event:?}");
		match &event {
			BlurEvent::NoEvent => {
				// NoEvent
			}
			BlurEvent::LoginStart { username: _ } => {
				// LoginStart
			}
			BlurEvent::LoginEnd {
				username: _,
				success: _,
			} => {
				// LoginEnd
			}
			BlurEvent::Screen { name: _ } => {
				// Screen
			}
		}
	}

	fn free(&self) {
		log::info!("Unloading plugin: {}", self.name());
	}
}

#[no_mangle]
fn plugin_init(_api: &mut dyn BlurAPI) -> Box<dyn BlurPlugin> {
	let plugin = MyDummyPlugin {};

	let cfg = ConfigBuilder::new()
		.set_time_offset_to_local()
		.unwrap()
		.build();

	let log_file = blur_plugins_core::create_log_file("my_dummy_plugin.log").unwrap();
	CombinedLogger::init(vec![
		TermLogger::new(
			LevelFilter::Trace,
			cfg,
			TerminalMode::Mixed,
			ColorChoice::Auto,
		),
		WriteLogger::new(LevelFilter::Trace, Config::default(), log_file),
	])
	.unwrap();

	log::info!("Init plugin: {}", plugin.name());

	Box::new(plugin)
}
