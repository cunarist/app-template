# 🆎 About This Template

> You can remove this section after you've created your own repository.

This template provides instant capabilities to developers who want to embrace the power of **Rust** and **Flutter** together. Simply duplicate this template and you're ready to go!

Based on the default Flutter template, many additional packages and modifications are applied to make sure everything is super-ready.

Extra features added to default Flutter template are:

- Rust integration with ability to use arbitrary number of library crates
- MVVM pattern with easy viewmodel binding from Dart and viewmodel update from Rust
- Convenient configuration management
- Preserving Rust logic on Dart hot reload
- Convenient app naming and icon generation
- Setting desktop window properties

## Platform Support

Configuring Flutter project targetting various platforms is not an easy task. It gets much harder when Rust is involved. With this template, you don't need to reinvent the wheel.

- Windows
- Linux
- Android
- macOS
- iOS

## Contribution

We love contributions! If you have any suggestion or want to report a bug, please leave it as an issue or a pull request. We will try to respond as quick as possible.

The goal of this template is to enable full power of Rust while using Flutter for front-end development. Rust code and Dart code should be detachable from each other.

# 🧱 Project Structure

**Flutter** deals with cross-platform user interface while **Rust** handles the internal logic. The front-end and back-end are completely separated. These two worlds communicate through native steams.

This repository is based on [Cunarist App Template](https://github.com/cunarist/app-template). It is possible to receive latest commits from this template repository with the automated Python command stated below.

# 👜 System Preparation

Flutter and Rust are required for building the app itself. Python is needed to automate complicated procedures. Git is responsible for version-control and template update.

You can use an IDE of your choice. However, [Visual Studio Code](https://code.visualstudio.com/) is recommended because it has extensive support from Flutter and Rust communities.

## Preparing Git

Go to the [official downloads page](https://git-scm.com/downloads)

## Preparing Python

Download it from the app store if your system doesn't provide a pre-installed version. It's also available at [official downloads page](https://www.python.org/downloads/). Make sure Python is available in the path environment variable.

## Preparing Rust

Refer to the [official docs](https://doc.rust-lang.org/book/ch01-01-installation.html).

## Preparing Flutter

Refer to the [official docs](https://docs.flutter.dev/get-started/install).

## System Verification

You can make sure if your system is ready for development in the terminal.

```
git --version
python --version
rustc --version
flutter doctor
```

## Extra Steps

If you are targeting Android, iOS or web, there are extra steps involved. Refer to [flutter_rust_bridge docs](https://cjycode.com/flutter_rust_bridge/template/setup.html).

# 🗃️ Setting Up

Install Dart packages written in `./pubspec.yaml` from [Pub](https://pub.dev/).

```
flutter pub get
```

Install Python packages written in `./requirements.txt` from [PyPI](https://pypi.org/).

```
pip install -r requirements.txt
```

Set the app name and domain. This only works once and you cannot revert this.

```
python automate app-naming
```

Convert icon files in `./assets` to apply the icon to multiple platforms with [Flutter Launcher Icons](https://pub.dev/packages/flutter_launcher_icons).

File `app.icon.png` is applied to:

- Windows
- Linux
- macOS
- Android

File `app_icon_full.png` is applied to:

- iOS

For platforms below, icon file should be included at release.

- Linux

```
flutter pub run flutter_launcher_icons
```

Receive latest commits from [Cunarist App Template](https://github.com/cunarist/app-template).

```
python automate template-update
```

# 📝 Configuration Files

Before you proceed, you need to prepare files for system configuration.

This terminal command will generate configuration files or update them from template files if they already exist.

```
python automate config-filling
```

Files for environment variables are not version-controlled. You might be wondering about why there are multiple files for managing environment variables. It's basically because this template combines multiple programming languages.

- File `./android/local.properties` contains information about Android toolchain on the computer.
- File `./native/.cargo/config.toml` includes environment variables loaded in Rust. You might need them to locate external C++ library paths through environment variables for compilation.
- There is no environment variable file for Dart. Use Dart's [hot reload](https://docs.flutter.dev/development/tools/hot-reload) feature instead. You might want to change variables directly in Dart to experiment UI stuffs such as dark mode.

You should change values of environment variables inside these files during development to suit your needs. Environment variable files are only used in production and not included in the final release.

# 🍳 Actual Development

You might need to dive into this section quite often.

Check and fix problems in `./native` Rust code and `./lib` Dart code.

```
python automate code-quality
```

Check the actual sizes of compiled Rust libraries in release mode.

```
python automate size-check (platform)
```

Run the app in debug mode.

```
flutter run
```

Build the app in release mode.

```
flutter build (platform) --release
```

# 📜 Rules

## Allowed Modification

Be careful all the time! You shouldn't be editing any file without enough knowledge on how it works. Below are the top-level files and folders that are allowed to edit during app development:

### Dart Related

- `lib`: Dart modules.
  - Do not modify `bridge` folder inside.
- `pubspec.yaml`: Dart settings and dependencies.

### Rust Related

- `native`: Rust crates. The name of the library crate folder should be exactly the same as that of library crate's name.
  - Do not modify `bridge` crate inside.
  - `config.toml.template` file is okay to be modified if needed.
  - Think of `hub` as your Rust entry point crate. It is not a binary crate but it is similar.

## Comments

Please write kind and readable comments next to your code. You are probably not going to be developing on your own. Other developers should to be able to get a grasp of complex code that you wrote. Long and detailed comments are also welcomed.

## User Interface Texts

Always write user interface texts in `./assets/translations`.

When an app gains popularity, there comes a need to support multiple human languages. However, manually replacing thousands of text widgets in the user interface is not a trivial task. Therefore it is a must to write texts that will be presented to normal users in translation files.

Refer to [Easy Localization](https://pub.dev/packages/easy_localization) docs for more details.

## Division of Functions

Dart should only be used for front-end user interface and Rust should handle all other back-end logics such as file handling, event handling, timer repetition, calculation, network communication, etc. There can be an exception though if Rust or Dart has trouble dealing with multiple platforms on one's side.

If the characteristic of a specific Rust API is totally different from other Rust APIs, it should be detached into a separate Rust crate. All crates should provide a clean API with descriptive function names.

## Python Automation Scripts

For faster and easier development, Cunarist App Template relies on Python scripts in `./automate` for automation.

[Black](https://black.readthedocs.io/en/stable/) formatter is used for maintaining quality Python code.

# 📁 Folder Structure

Most of the top-level folders comes from default Flutter template.

- `windows`: Platform-specific files
- `linux`: Platform-specific files
- `macos`: Platform-specific files
- `android`: Platform-specific files
- `ios`: Platform-specific files
- `web`: Platform-specific files
- `lib`: Dart modules empowering the Flutter application.

However, there are some extra folders created in Cunarist App Template in order to integrate other elements into development.

- `automate`: Python scripts for automating development process. These scripts have nothing to do with actual build and doesn't get included in the app release. Only for developers.
- `native`: A workspace Rust crate that includes many other library crates. Each crate inside this folder gets compiled into a library file(`.dll`/`.so`). This means if there are 10 crates inside this folder, then there would be 10 library file next to the executable after compilation, each with a file name corresponding to their original crate.
- `assets`: A place for asset files such as images.

In addition, there might be some other temporary folders generated by tools or IDE you are using. Those should not be version-controlled.
