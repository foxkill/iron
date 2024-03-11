// !# Contains all application macros.

#[macro_export]
macro_rules! rows {
    ($($key:expr => $value:expr),*) => ({
        let mut vc = Vec::new();
        $(vc.push(SlMap{ key: SharedString::from($key), value: SharedString::from($value) });)*
        vc
    });
}

#[macro_export]
macro_rules! add {
    ($($to: ident, $key:expr, $value:expr),*) => {
        $(
            let items = Rc::new(VecModel::default());
            items.push($key.into());
            items.push($value.into());
            $to.push(items.into())
        )*
    };
}