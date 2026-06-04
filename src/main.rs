use dioxus::prelude::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");

#[manganis::ffi("src/android")]
extern "Kotlin" {
    pub type MinimalFFIReproAndroid;
    pub fn f(this : &MinimalFFIReproAndroid) -> i32;
    pub fn g(this : &MinimalFFIReproAndroid) -> String;
}

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Hero {}

    }
}

#[component]
pub fn Hero() -> Element {
    with_activity(|
        mut env,
        activity|
    {
        // Trying to load some classes with both find_class and load_class_from_classloader
        let classes = vec![
            "java/lang/String",
            "java/lang/ClassLoader",
            "com/google/android/material/badge/BadgeDrawable",
            "com/example/minimalffirepro/BuildConfig",
            "dev/dioxus/main/Logger",
            "com/example/minimalffirepro/MinimalFFIReproAndroid",
        ];
        for class in classes.iter() {
            match find_class(env,class) { 
                Ok(_v) => println!("Found (find_class): {}", class),
                Err(_e) => { println!("Not found (find_class): {}",class); }
            };
            let _ = env.exception_clear();
        }

        for class in classes.iter() {
            match load_class_from_classloader(env,class) { 
                Ok(_v) => println!("Found (load_class_from_classloader): {}", class),
                Err(_e) => { println!("Not found (load_class_from_classloader): {}",class); }
            };
            let _ = env.exception_clear();
        }
        Some(1)
    });

    
    let testObj = MinimalFFIReproAndroid::new();
    let gValue = match testObj
    {
        Ok(v) => g(&v).unwrap(),
        Err(e) => {println!("Issue calling g() - {}",e); "Can't get from Kotlin".to_string()},
    };

    rsx! {
        div {
            id: "hero",
        p {
            {gValue.to_string()}
        }
    }
}
}
