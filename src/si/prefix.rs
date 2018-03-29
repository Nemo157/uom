/// Macro to implement the [SI][si] prefixes for [multiples of units][mult] and
/// [submultiples of units][submult].
///
/// Implemented using `macro_rules!` instead of `const` so that type inference at call sites can
/// generate the appropriate float type. Using explicit constants would require duplicate
/// definitions for `f32` and `f64` or casting from `f64` in `f32` contexts.
///
/// [si]: http://jcgm.bipm.org/vim/en/1.16.html
/// [mult]: http://jcgm.bipm.org/vim/en/1.17.html
/// [submult]: http://jcgm.bipm.org/vim/en/1.18.html
#[macro_export]
macro_rules! prefix {
    (yotta) => { (1_000_000_000_000_000_000_000, 1) };
    (zetta) => { (1_000_000_000_000_000_000, 1) };
    (exa) => { (1_000_000_000_000_000_000, 1) };
    (peta) => { (1_000_000_000_000_000, 1) };
    (tera) => { (1_000_000_000_000, 1) };
    (giga) => { (1_000_000_000, 1) };
    (mega) => { (1_000_000, 1) };
    (kilo) => { (1000, 1) };
    (hecto) => { (100, 1) };
    (deca) => { (10, 1) };
    (none) => { (1, 1) };
    (deci) => { (1, 10) };
    (centi) => { (1, 100) };
    (milli) => { (1, 1000) };
    (micro) => { (1, 1_000_000) };
    (nano) => { (1, 1_000_000_000) };
    (pico) => { (1, 1_000_000_000_000) };
    (femto) => { (1, 1_000_000_000_000_000) };
    (atto) => { (1, 1_000_000_000_000_000_000) };
    (zepto) => { (1, 1_000_000_000_000_000_000_000) };
    (yocto) => { (1, 1_000_000_000_000_000_000_000_000) };
}
