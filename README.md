# Dioxus FFI Issue - Minimal Reproduction

To run, use `dx serve --android --verbose`.

# Overview
`main.rs` is using `manganis::ffi` to get the type `MinimalFFIRepro` (from `MinimalFFIRepro.kt`) and the `f()` method within it, from within the Kotlin file `MinimalFFIRepro.kt`.

In `main()`, we attempt to make a new `MinimalFFIRepro` and then call `f()` using it:

```rust
    let testObj = MinimalFFIRepro::new().unwrap();
    println!("{}",f(&testObj).unwrap());
 ```

 # The Problem
 When run using the `dx serve` command, the application compiles and starts to run, but immediately crashes with the error:

```
16:00:19 [android] E/AndroidRuntime( 7573): FATAL EXCEPTION: Thread-2
16:00:19 [android] E/AndroidRuntime( 7573): Process: com.example.minimalffirepro, PID: 7573
16:00:19 [android] E/AndroidRuntime( 7573): java.lang.ClassNotFoundException: Didn't find class "com.example.minimalffirepro.MinimalFFIRepro" on path: DexPathList[[directory "."],nativeLibraryDirectories=[/system/lib64, /system_ext/lib64, /system/lib64, /system_ext/lib64]]
16:00:19 [android] E/AndroidRuntime( 7573): 	at dalvik.system.BaseDexClassLoader.findClass(BaseDexClassLoader.java:259)
16:00:19 [android] E/AndroidRuntime( 7573): 	at java.lang.ClassLoader.loadClass(ClassLoader.java:642)
16:00:19 [android] E/AndroidRuntime( 7573): 	at java.lang.ClassLoader.loadClass(ClassLoader.java:578)
```

This is despite the fact that the `dx serve` (verbose) output suggests that it has been picked up: 

```
15:28:59 [dev] Bundling Android plugin 'minimalffirepro' from source: /Users/capage/meaningandmechanism/minimal-ffi-repro/src/android 
15:28:59 [dev] Stripped version specifiers from /Users/capage/meaningandmechanism/minimal-ffi-repro/target/dx/minimal-ffi-repro/debug/android/app/plugins/minimalffirepro/build.gradle.kts 
15:28:59 [dev] Added Android plugin module :plugins:minimalffirepro from /Users/capage/meaningandmechanism/minimal-ffi-repro/src/android 
```

 # Other Info
 * A manual look at the APK suggests that the class is not included, even if the `com.example.minimalffirepro` package is present.
    * It contains *just* the `R` and `BuildConfig` files within it.
* Running `gradle build` manually in `src/android` does compile.
    * and produces a `.aar` that contains the compiled class.


 # Thanks
 Thanks for reading!