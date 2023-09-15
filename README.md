# blur-test-plugin
Boilerplate code for DLL that can be loaded from Blur with https://github.com/tobii-dev/blur-hooks-rs

```bash
cargo +nightly build --target=i686-pc-windows-msvc --release
COPY /Y "target\i686-pc-windows-msvc\release\my_dummy_plugin.dll" "<BLUR_DIR>\amax\dlls\dummy_plugin.dll"
```
