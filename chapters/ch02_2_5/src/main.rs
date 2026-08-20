fn add_one(x: i32) -> i32 {
    x + 1
}

fn stringify(x: i32) -> String {
    x.to_string()
}

fn prefix_with(prefix: &str) -> impl Fn(String) -> String + '_ {
    move |x| format!("{}{}", prefix, x)
}

fn compose_two<FIRST, SECOND, THIRD, F, G>(f: F, g: G) -> impl Fn(FIRST) -> THIRD
where
    F: Fn(FIRST) -> SECOND,
    G: Fn(SECOND) -> THIRD,
{
    move |x| g(f(x))
}

macro_rules! compose {
    ($last:expr) => {
        $last
    };

    ($head:expr, $($tail:expr),+) => {
        compose_two($head, compose!($($tail),+))
    };

}

macro_rules! compose_alt {
    ($last:expr) => {
        $last
    };

    ($head:expr => $($tail:expr) =>+) => {
        compose_two($head, compose_alt!($($tail) =>+))
    };
}

// macro_rules! pipe {
//     ($last:expr) => {
//         $last
//     };

//     ($head:expr |> $($tail:expr) |>+) => {
//         compose_two($head, compose_alt!($($tail) |>+))
//     };
// }

// Assisted-by: Claude code Opus 5 (default - high)
// `$head:expr |> ...` is rejected: an `expr` fragment may only be followed by
// `=>`, `,` or `;`. So we never use `expr` in the matcher -- instead we munch
// raw token trees into an accumulator until a top-level `|>` shows up, and only
// then hand the accumulated tokens to the compiler as an expression.
// macro_rules! pipe {
//     // a `|>` at this position: everything accumulated so far is the head
//     (@munch [$($head:tt)+] |> $($tail:tt)*) => {
//         compose_two($($head)+, pipe!(@munch [] $($tail)*))
//     };

//     // not a `|>`: swallow one more token tree and keep looking
//     (@munch [$($head:tt)*] $next:tt $($tail:tt)*) => {
//         pipe!(@munch [$($head)* $next] $($tail)*)
//     };

//     // input exhausted: the accumulator is the final expression
//     (@munch [$($last:tt)+]) => {
//         $($last)+
//     };

//     // entry point
//     ($($input:tt)+) => {
//         pipe!(@munch [] $($input)+)
//     };
// }

fn main() {
    let two_fn_composed = compose_two(compose_two(add_one, stringify), prefix_with("Result: "));
    println!("{} - compose_two (chanined)", two_fn_composed(5));

    let composed = compose!(add_one, stringify, prefix_with("Result: "));
    println!("{} - compose!", composed(5));

    let composed = compose_alt!(add_one=> stringify=> prefix_with("Result: "));
    println!("{} - compose_alt!", composed(5));

    // let piped = pipe!(
    //     add_one |> stringify |> prefix_with("Result: ")
    // );
    // println!("{}", piped(5));

    // // multi-token stages work too, since we accumulate token trees
    // let piped_closure = pipe!(
    //     |x: i32| x * 2 + 1 |> stringify |> prefix_with("Doubled: ")
    // );
    // println!("{}", piped_closure(5));
}
