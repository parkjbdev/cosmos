///
/// Prints to the standard output.
///
/// Adapted from [`std::print`].
///
/// [`std::print`]: https://doc.rust-lang.org/stable/std/macro.print.html
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        #[cfg(target_arch = "aarch64")]
        {
            use core::fmt::Write;
            $crate::bsp::driver::arm::pl011::CONSOLE.lock().as_mut().unwrap().write_fmt(format_args!($($arg)*)).unwrap();
            // $crate::console::print(::core::format_args!($($arg)*));
        }
    };
}

/// Prints to the standard output, with a newline.
///
/// Adapted from [`std::println`].
///
/// [`std::println`]: https://doc.rust-lang.org/stable/std/macro.println.html
#[macro_export]
macro_rules! println {
    () => {
        $crate::print!("\n")
    };
    ($($arg:tt)*) => {
        #[cfg(target_arch = "aarch64")]
        {
            // print!("{}\n", format_args!($($arg)*));
            use core::fmt::Write;
            $crate::bsp::driver::arm::pl011::CONSOLE.lock().as_mut().unwrap().write_fmt(::core::format_args!("{}\n", format_args!($($arg)*))).unwrap();
            // $crate::console::print(::core::format_args!("{}\n", format_args!($($arg)*)));
        }
    };
}

///
/// Prints to the standard output.
///
/// Adapted from [`std::print`].
///
/// [`std::print`]: https://doc.rust-lang.org/stable/std/macro.print.html
#[macro_export]
macro_rules! phys_print {
    ($($arg:tt)*) => {
        #[cfg(target_arch = "aarch64")]
        {
            use core::fmt::Write;
            $crate::arch::aarch64::serial::CONSOLE.lock().write_fmt(format_args!($($arg)*)).unwrap();
        }
    };
}

/// Prints to the standard output, with a newline.
///
/// Adapted from [`std::println`].
///
/// [`std::println`]: https://doc.rust-lang.org/stable/std/macro.println.html
#[macro_export]
macro_rules! phys_println {
    () => {
        $crate::print!("\n")
    };
    ($($arg:tt)*) => {
        #[cfg(target_arch = "aarch64")]
        {
            use core::fmt::Write;
            $crate::arch::aarch64::serial::CONSOLE.lock().write_fmt(::core::format_args!("{}\n", format_args!($($arg)*))).unwrap();
        }
    };
}

/// Prints and returns the value of a given expression for quick and dirty
/// debugging.
// Copied from std/macros.rs
#[macro_export]
macro_rules! dbg {
    // NOTE: We cannot use `concat!` to make a static string as a format argument
    // of `eprintln!` because `file!` could contain a `{` or
    // `$val` expression could be a block (`{ .. }`), in which case the `eprintln!`
    // will be malformed.
    () => {
        $crate::println!("[{}:{}]", ::core::file!(), ::core::line!())
    };
    ($val:expr $(,)?) => {
        // Use of `match` here is intentional because it affects the lifetimes
        // of temporaries - https://stackoverflow.com/a/48732525/1063961
        match $val {
            tmp => {
                $crate::println!("[{}:{}] {} = {:#?}",
                    ::core::file!(), ::core::line!(), ::core::stringify!($val), &tmp);
                tmp
            }
        }
    };
    ($($val:expr),+ $(,)?) => {
        ($($crate::dbg!($val)),+,)
    };
}
