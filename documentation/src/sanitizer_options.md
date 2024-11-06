# Sanitizer Options

Sanitizers are tools that help detect various types of bugs and vulnerabilities during runtime by instrumenting code to catch errors such as memory corruption, data races, and undefined behavior. The `SanitizerOptions` structure lets you enable or disable specific sanitizers to improve the effectiveness of your fuzzing process.

## Enabling Sanitizers

To activate sanitizers, set `sanitizer_enabled` to `true` in your fuzzer configuration. Then, use `SanitizerOptions` to specify which sanitizers to enable. Here’s an example:

```rust
use fuzzer::FuzzerConfig;
use fuzzer::sanitizer_options::SanitizerOptions;

let sanitizer_options = SanitizerOptions {
    address: true,
    thread: false,
    memory: false,
    undefined_behavior: true,
    leak: true,
};

let config = FuzzerConfig::builder()
    .sanitizer_enabled(true)
    .sanitizer_options(sanitizer_options)
    .build();
``` 
