macro_rules! _lazy_static {
    ($(#[attr:meta])* static ref $N:ident : $T:ty = $e:expr; $($t:tt)*) => {
        __lazy_static_internal!($(#[$attr])* () static ref $N : $T = $e; $($t)*)
    };

    ($(#[$attr:meta])* ($($vis:tt)*) static ref $N:ident: $T:ty = $e:expr; $($t:tt)*) => {
        __lazy_static_internal!(
            @MAKE TY,
            $(#[$attr])*,
            ($($vis)*),
            $N,
        );

        __lazy_static_internal!(@TAIL, $N : $T = $e);
        // ...
    }
}
fn main() {
    println!("Hello, world!");
}
