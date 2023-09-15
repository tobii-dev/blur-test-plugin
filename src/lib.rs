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
		match &event {
			BlurEvent::NoEvent => {}
			BlurEvent::Login(_u) => {}
			BlurEvent::Screen(_u) => {}
		}
		log::info!("Event: {event:?} from plugin: {}", self.name());
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
	let log_path = std::format!(".\\amax\\log\\{}.log", plugin.name());

	CombinedLogger::init(vec![
		TermLogger::new(
			LevelFilter::Trace,
			cfg,
			TerminalMode::Mixed,
			ColorChoice::Auto,
		),
		WriteLogger::new(
			LevelFilter::Trace,
			Config::default(),
			std::fs::File::create(&log_path)
				.expect(&std::format!("Couldn't create log file: {log_path}")),
		),
	])
	.unwrap();
	log::info!("Init plugin: {}", plugin.name());

	Box::new(plugin)
}
