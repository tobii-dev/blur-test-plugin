use blur_plugins_core::{BlurAPI, BlurEvent, BlurPlugin};

#[repr(C)]
struct MyDummyPlugin {}

impl BlurPlugin for MyDummyPlugin {
	fn name(&self) -> &'static str {
		"MyDummyPlugin"
	}

	fn on_event(&self, _event: &BlurEvent) {}

	fn free(&self) {}
}

#[no_mangle]
fn plugin_init(_api: &mut dyn BlurAPI) -> Box<dyn BlurPlugin> {
	let plugin = MyDummyPlugin {};
	Box::new(plugin)
}
