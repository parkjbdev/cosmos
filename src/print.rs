///
/// Prints to the standard output.
///
/// Adapted from [`std::print`].
///
/// [`std::print`]: https://doc.rust-lang.org/stable/std/macro.print.html
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        #[cfg(target_os = "none")]
        {
            $crate::console::print(::core::format_args!($($arg)*));
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
        #[cfg(target_os = "none")]
        {
            $crate::console::print(::core::format_args!("{}\n", format_args!($($arg)*)));
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

// RAW MACROS FOR TESTING PURPOSES ONLY. DO NOT USE IN PRODUCTION CODE.
// #[macro_export]
// macro_rules! __println {
//     ($($arg:tt)*) => {
//         {
//             use crate::arch::drivers::pl011::PL011Uart;
//             use crate::driver::interface::DeviceDriver;
//             use crate::console::interface::Write;
//
//             let uart = PL011Uart::new(0x0900_0000, 24_000_000, 115_200);
//             let _ = uart.init();
//             let _ = uart.write_fmt(core::format_args!("{}\n", core::format_args!($($arg)*)));
//         }
//     };
// }
