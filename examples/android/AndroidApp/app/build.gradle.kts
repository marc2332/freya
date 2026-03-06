plugins {
    alias(libs.plugins.android.application)
}

android {
    namespace = "com.freya.androidapp"
    compileSdk {
        version = release(36) {
            minorApiLevel = 1
        }
    }

    defaultConfig {
        applicationId = "com.freya.androidapp"
        minSdk = 36
        targetSdk = 36
        versionCode = 1
        versionName = "1.0"

        testInstrumentationRunner = "androidx.test.runner.AndroidJUnitRunner"
    }

    buildTypes {
        release {
            isMinifyEnabled = false
            proguardFiles(
                getDefaultProguardFile("proguard-android-optimize.txt"),
                "proguard-rules.pro"
            )
        }
    }
    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_17
        targetCompatibility = JavaVersion.VERSION_17
    }
    buildFeatures {
        viewBinding = true
    }
}

tasks.register<Exec>("buildRustLibrary") {
    workingDir("../..")
    commandLine("cargo", "ndk",
        "-o", "AndroidApp/app/src/main/jniLibs/",
        "-t", "arm64-v8a",
        "build", "--lib", "--release")
}

tasks.named("preBuild") {
    dependsOn("buildRustLibrary")
}

dependencies {
    implementation(libs.androidx.games.activity)
    implementation(libs.androidx.core.ktx)
    implementation(libs.androidx.appcompat)
    implementation(libs.material)
    implementation(libs.androidx.constraintlayout)
    testImplementation(libs.junit)
    androidTestImplementation(libs.androidx.junit)
    androidTestImplementation(libs.androidx.espresso.core)
    implementation(libs.androidx.startup.runtime)
}
