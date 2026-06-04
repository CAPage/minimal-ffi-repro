import org.gradle.api.tasks.bundling.AbstractArchiveTask
plugins {
    id("com.android.library") version "9.4.1"
    kotlin("android") version "2.3.21"
}

android {
    namespace = "com.example.minimalffirepro"
    compileSdk = 34

    defaultConfig {
        minSdk = 24
        targetSdk = 34
        consumerProguardFiles("consumer-rules.pro")
    }

    buildTypes {
        getByName("release") {
            isMinifyEnabled = false
        }
        getByName("debug") {
            isMinifyEnabled = false
            
        }
    }

    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_17
        targetCompatibility = JavaVersion.VERSION_17
    }

    kotlinOptions {
        jvmTarget = "17"
    }

}

repositories {
        google()
        mavenCentral()
}

dependencies {
    implementation("androidx.core:core-ktx:1.12.0")
}


tasks.withType<AbstractArchiveTask>().configureEach {
    archiveBaseName.set("minimalffireproandroid")
}