# Dioxus FFI Issue - Minimal Reproduction

To run, use `dx serve --android`.

# Overview
This repo is a simple reproduction of an issue I was having using `manganis::ffi` for Android. Here, the class `MinimalFFIReproAndroid` from Kotlin is not able to be used using `manganis::ffi`.

I investigated the issue, including attempting to manually get the class using `find_class` and `load_class_from_classloader`, neither of which work successfully. There's a simple version of that included in this minimal version within an `with_activity` in `main.rs`.

The class `MinimalFFIReproAndroid` is included within the APK (investigated using Android Studio.)

I have, at various points, gotten the error:
```
JNI DETECTED ERROR IN APPLICATION: JNI FindClass called with pending exception java.lang.ClassNotFoundException: Didn't find class "com.example.minimalffirepro.MinimalFFIReproAndroid" on path: DexPathList[[directory "."],nativeLibraryDirectories=[/system/lib64, /system_ext/lib64, /system/lib64, /system_ext/lib64]]
```

So I investigated issues with which classloader was being used. I actually managed to get this working on a patched version of Dioxus (more about this in the Dioxus Issue I'm going to write), where the `activity`'s classloader was used instead of `classloader.loadClass()` or `Class.forName()`.

# Thanks
Thanks for reading!