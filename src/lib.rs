use blur_plugins_core::{BlurAPI, BlurEvent, BlurPlugin};

#[repr(C)]
struct MyDummyPlugin {}

impl BlurPlugin for MyDummyPlugin {
	fn name(&self) -> &'static str {
		"MyTestPlugin"
	}

	fn on_event(&self, _event: &BlurEvent) {
		match &_event {
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
	fn free(&self) {}
}

#[no_mangle]
fn plugin_init(_api: &mut dyn BlurAPI) -> Box<dyn BlurPlugin> {
	let plugin = MyDummyPlugin {};
	println!("{}: Hello, world!", plugin.name());
	Box::new(plugin)
}
