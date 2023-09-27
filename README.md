# blur-test-plugin

Boilerplate for creating the most minimal BlurPlugin

## Build

```bash
cargo +nightly build --target=i686-pc-windows-msvc --release
COPY /Y "target\i686-pc-windows-msvc\release\my_test_plugin.dll" "<BLUR_DIR>\amax\dlls\my_test_plugin.asi"
```

## Loading

The created DLL, `my_test_plugin.asi` can then be loaded into Blur as a BlurPlugin.

- https://github.com/tobii-dev/blur-plugins-core

- https://github.com/tobii-dev/blur-hooks-rs
