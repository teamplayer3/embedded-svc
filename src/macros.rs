//Based on smoltcp's macro_rules
#[cfg(not(test))]
#[cfg(feature = "log")]
#[macro_export]
macro_rules! net_log {
    (info, $($arg:expr),*) => { log::info!($($arg),*) };
    (warn, $($arg:expr),*) => { log::warn!($($arg),*) };
}

#[cfg(test)]
#[cfg(feature = "log")]
#[macro_export]
macro_rules! net_log {
    (info, $($arg:expr),*) => { println!($($arg),*) };
    (warn, $($arg:expr),*) => { println!($($arg),*) };
}

#[cfg(feature = "defmt")]
#[macro_export]
macro_rules! net_log {
    (info, $($arg:expr),*) => { defmt::info!($($arg),*) };
    (warn, $($arg:expr),*) => { defmt::warn!($($arg),*) };
}

#[cfg(not(any(feature = "log", feature = "defmt")))]
#[macro_export]
macro_rules! net_log {
    ($level:ident, $($arg:expr),*) => {{ $( let _ = $arg; )* }}
}

pub(crate) use net_log;