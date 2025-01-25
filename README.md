# Windows
`cargo run -p playground-windows`

# Android
## Install
```nu
$env.ANDROID_HOME = "/home/bfall/android-sdk"
$env.NDK_HOME = (ls ($env.ANDROID_HOME | path join "ndk" ) | sort-by name --reverse | get name.0)

use std/util "path add"
path add ($env.ANDROID_HOME | path join "cmdline-tools/bin")
```

```bash
sudo apt install lldb llvm clang openjdk-17-jdk-headless pkg-config libssl-dev unzip build-essential android-tools-adb
mkdir android-sdk
wget https://dl.google.com/android/repository/commandlinetools-linux-11076708_latest.zip
unzip commandlinetools-linux-11076708_latest.zip -d $env.ANDROID_HOME
yes | sdkmanager --licenses
sdkmanager "build-tools" "ndk" "platforms"
cargo binstall xbuild
```

## Check
`x doctor`

## Build 
`x build --format apk --platform android --arch arm64 -p playground-android`