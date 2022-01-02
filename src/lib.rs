#![warn(
    clippy::complexity,
    clippy::correctness,
    clippy::perf,
    clippy::style,
    missing_docs,
    rust_2018_idioms
)]
#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
use self::macos as platform;

mod key_code;
pub use self::{key_code::*, platform::*};

#[cfg(test)]
mod tests {
    use std::{thread, time::Duration};

    use super::*;

    #[test]
    fn test() {
        let hook = Hook::new().unwrap();
        hook.register(KeyCode::Numpad1, || println!("A")).unwrap();
        println!("Press Numpad1");
        thread::sleep(Duration::from_secs(5));
        hook.unregister(KeyCode::Numpad1).unwrap();
        hook.register(KeyCode::KeyN, || println!("B")).unwrap();
        println!("Press KeyN");
        thread::sleep(Duration::from_secs(5));
        hook.unregister(KeyCode::KeyN).unwrap();
        hook.register(KeyCode::Numpad1, || println!("C")).unwrap();
        println!("Press Numpad1");
        thread::sleep(Duration::from_secs(5));
        hook.unregister(KeyCode::Numpad1).unwrap();
    }

    #[test]
    fn resolve() {
        // Based on German keyboard layout.
        println!("ß: {}", KeyCode::Minus.resolve());
        println!("ü: {}", KeyCode::BracketLeft.resolve());
        println!("#: {}", KeyCode::Backslash.resolve());
        println!("+: {}", KeyCode::BracketRight.resolve());
        println!("z: {}", KeyCode::KeyY.resolve());
        println!("^: {}", KeyCode::Backquote.resolve());
        println!("<: {}", KeyCode::IntlBackslash.resolve());
        println!("Yen: {}", KeyCode::IntlYen.resolve());
        println!("Enter: {}", KeyCode::Enter.resolve());
        println!("Space: {}", KeyCode::Space.resolve());
        println!("Tab: {}", KeyCode::Tab.resolve());
        println!("Numpad0: {}", KeyCode::Numpad0.resolve());
    }
}
