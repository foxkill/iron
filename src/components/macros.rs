// !# Contains all application macros.

#[macro_export]
macro_rules! rows {
    ($($key:expr => $value:expr),*) => ({
        let mut vc = Vec::new();
        $(vc.push(SlMap{ key: SharedString::from($key), value: SharedString::from($value) });)*
        vc
    });
}