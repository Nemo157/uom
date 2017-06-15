/// Macro to implement a [system of quantities](http://jcgm.bipm.org/vim/en/1.3.html).
///
/// * `$quantities_attr`: System of quantities attributes. Generally used to set documentation
///   comments for the system of quantities.
/// * `$quantities`: Name of the system of quantities (e.g. `ISQ`).
/// * `$name`: Name of the base quantities for the system of quantities (e.g. `length`, `mass`,
///   ...). Note that this name must match the module name of the quantity.
/// * `$unit`: Base unit of the quantity (e.g. `meter`, `kilogram`).
/// * `$symbol`: Dimension symbol of the quantity.
/// * `$units_attr`: System of units attributes. Generally used to set documentation comments for
///   the system of units.
/// * `$units`: Name of the system of units (e.g. `SI`).
/// * `$module`: Optional. Module name of the quantity. A `#[macro_use] pub mod $module;` statement
///   is generated if this variable is given.
/// * `$quantity`: Quantity name (e.g. `Length`, `Mass`, ...).
///
/// An example invocation is given below for a meter-kilogram-second system.
///
/// ```
/// # #[macro_use] extern crate uom;
/// # fn main() { }
/// # mod mks {
/// #     #[macro_use]
/// #     mod length {
/// #         quantity! {
/// #             /// Length (base unit meter, m<sup>1</sup>).
/// #             quantity: MksLength; "length";
/// #             /// Length dimension, m<sup>1</sup>.
/// #             dimension: Q<P1 /*length*/, Z0 /*mass*/, Z0 /*time*/>;
/// #             units {
/// #                 @meter: 1.0E0; "m", "meter", "meters";
/// #                 @foot: 3.048E-1; "ft", "foot", "feet";
/// #             }
/// #         }
/// #     }
/// #     #[macro_use]
/// #     mod mass {
/// #         quantity! {
/// #             /// Mass (base unit kilogram, kg<sup>1</sup>).
/// #             quantity: MksMass; "mass";
/// #             /// Mass dimension, kg<sup>1</sup>.
/// #             dimension: Q<Z0 /*length*/, P1 /*mass*/, Z0 /*time*/>;
/// #             units {
/// #                 @kilogram: 1.0; "kg", "kilogram", "kilograms";
/// #             }
/// #         }
/// #     }
/// #     #[macro_use]
/// #     mod time {
/// #         quantity! {
/// #             /// Time (base unit second, s<sup>1</sup>).
/// #             quantity: MksTime; "time";
/// #             /// Time dimension, s<sup>1</sup>.
/// #             dimension: Q<Z0 /*length*/, Z0 /*mass*/, P1 /*time*/>;
/// #             units {
/// #                 @second: 1.0; "s", "second", "seconds";
/// #             }
/// #         }
/// #     }
/// system! {
///     /// System of quantities, Q.
///     quantities: Q {
///         length: meter, L;
///         mass: kilogram, M;
///         time: second, T;
///     }
///     /// System of units, U.
///     units: U {
///         MksLength,
///         MksMass,
///         MksTime,
///     }
/// }
/// #     mod f32 {
/// #         Q!(mks, f32/*, (centimeter, gram, second)*/);
/// #     }
/// # }
/// ```
#[macro_export]
macro_rules! system {
    (
        $(#[$quantities_attr:meta])* quantities: $quantities:ident {
            $($name:ident: $unit:ident, $symbol:ident;)+
        }
        $(#[$units_attr:meta])* units: $units:ident {
            $($module:ident::$quantity:ident,)+
        }
    ) =>
    {
        $(#[macro_use]
        pub mod $module;)+

        system! {
            $(#[$quantities_attr])*
            quantities: $quantities {
                $($name: $unit, $symbol;)+
            }
            $(#[$units_attr])*
            units: $units {
                $($quantity,)+
            }
        }
    };
    (
        $(#[$quantities_attr:meta])* quantities: $quantities:ident {
            $($name:ident: $unit:ident, $symbol:ident;)+
        }
        $(#[$units_attr:meta])* units: $units:ident {
            $($quantity:ident,)+
        }
    ) =>
    {
        mod system { pub use super::*; }

        /// Property of a phenomenon, body or substance, where the property has a magnitude that
        /// can be expressed as a number and a reference.
        ///
        /// The preferred method of creating a `Quantity` instance is to use the `new` constructor
        /// which is generic over the input unit and accepts the input value as it's only
        /// parameter.
        ///
        /// ```
        #[cfg_attr(feature = "f32", doc = " # use uom::si::f32::*;")]
        #[cfg_attr(not(feature = "f32"), doc = " # use uom::si::f64::*;")]
        /// # use uom::si::length::meter;
        /// // Create a length of 1 meter.
        /// let L = Length::new::<meter>(1.0);
        /// ```
        ///
        /// `Quantity` fields are public to allow for the creation of `const` values and instances
        /// of non-named `Quantity`s. This functionality will be deprecated and subsequently removed
        /// once the `const fn` feature is stabilized.
        ///
        /// ```
        /// # use uom::si::{Quantity, ISQ, SI};
        #[cfg_attr(feature = "f32", doc = " # use uom::si::f32::*;")]
        #[cfg_attr(not(feature = "f32"), doc = " # use uom::si::f64::*;")]
        /// # use uom::stdlib::marker::PhantomData;
        /// # use uom::typenum::{P2, Z0};
        /// // Create a `const` length of 1 meter.
        /// const L: Length = Length { dimension: PhantomData, units: PhantomData, value: 1.0, };
        /// // Create a `const` area of 1 square meter explicitly without using the `Area` alias.
        #[cfg_attr(feature = "f32", doc = " const A: Quantity<ISQ<P2, Z0, Z0, Z0, Z0, Z0, Z0>, SI<f32>, f32> =")]
        #[cfg_attr(not(feature = "f32"), doc = " const A: Quantity<ISQ<P2, Z0, Z0, Z0, Z0, Z0, Z0>, SI<f64>, f64> =")]
        ///     Quantity { dimension: PhantomData, units: PhantomData, value: 1.0, };
        /// ```
        ///
        /// * http://jcgm.bipm.org/vim/en/1.1.html
        #[derive(Copy, Clone)]
        pub struct Quantity<D, U, V>
        where
            D: Dimension,
            U: Units<D, V>,
        {
            /// Quantity dimension. See [`Dimension`](./trait.Dimension.html).
            pub dimension: $crate::stdlib::marker::PhantomData<D>,
            /// Quantity base units. See [`Units`](./trait.Units.html).
            pub units: $crate::stdlib::marker::PhantomData<U>,
            /// Quantity value stored in the base units for the quantity.
            pub value: V,
        }

        /// Marker trait to express the dependence of a [quantity][quantity] on the
        /// [base quantities][base] of a [system of quantities][quantities] as a product of powers
        /// of factors corresponding to the base quantities, omitting any numerical factor.
        ///
        /// * http://jcgm.bipm.org/vim/en/1.7.html
        ///
        /// [quantity]: http://jcgm.bipm.org/vim/en/1.1.html
        /// [base]: http://jcgm.bipm.org/vim/en/1.4.html
        /// [quantities]: http://jcgm.bipm.org/vim/en/1.3.html
        pub trait Dimension {}

        /// Trait to identify a [system of units][units] based on a set of [base units][base] of a
        /// [system of quantities][quantities].
        ///
        /// [units]: http://jcgm.bipm.org/vim/en/1.13.html
        /// [base]: http://jcgm.bipm.org/vim/en/1.10.html
        /// [quantities]: http://jcgm.bipm.org/vim/en/1.3.html
        pub trait Units<D, V>
        where
            D: Dimension,
        {
            /// Conversion factor for the given base units to the base units for the system of
            /// quantities.
            fn conversion() -> V;
        }

        /// Trait to identify [measurement units][measurement] of individual
        /// [quantities][quantity].
        ///
        /// [measurement]: http://jcgm.bipm.org/vim/en/1.9.html
        /// [quantity]: http://jcgm.bipm.org/vim/en/1.1.html
        pub trait Unit {
            /// Unit abbreviation.
            fn abbreviation() -> &'static str;

            /// Unit singular description.
            fn singular() -> &'static str;

            /// Unit plural description.
            fn plural() -> &'static str;
        }

        /// Trait to identify conversion factors for measurement units. See
        /// [`Unit`](./trait.Unit.html).
        pub trait Conversion<V>: Unit {
            /// Conversion factor for the given unit to the base unit for the quantity.
            fn conversion() -> V;
        }

        $(#[$quantities_attr])*
        #[allow(missing_debug_implementations)]
        #[derive(Clone, Copy)]
        pub struct $quantities<$($symbol),+>
        where
            $($symbol: $crate::typenum::Integer,)+
        {
            $($name: $crate::stdlib::marker::PhantomData<$symbol>),+
        }

        /// Type alias for [dimension one][one] for which all the exponents of the factors
        /// corresponding to the [base quantities][base] are zero.
        ///
        /// [one]: http://jcgm.bipm.org/vim/en/1.8.html
        /// [base]: http://jcgm.bipm.org/vim/en/1.4.html
        pub type One = $quantities<$(replace_ty!($symbol $crate::typenum::Z0)),+>;

        // Type aliases for dimensions where all exponents of the factors are the given value.
        #[cfg(feature = "std")]
        type DN1 = $quantities<$(replace_ty!($symbol $crate::typenum::N1)),+>;
        #[cfg(feature = "std")]
        type DP2 = $quantities<$(replace_ty!($symbol $crate::typenum::P2)),+>;
        #[cfg(feature = "std")]
        type DP3 = $quantities<$(replace_ty!($symbol $crate::typenum::P3)),+>;

        #[allow(non_camel_case_types)]
        impl<$($name,)+ $($symbol,)+ V> $crate::stdlib::fmt::Debug
            for Quantity<$quantities<$($name),+>, BaseUnits<$($symbol,)+ V>, V>
        where
            $quantities<$($name),+>: Dimension,
            $($name: $crate::typenum::Integer,)+
            BaseUnits<$($symbol,)+ V>: Units<$quantities<$($name),+>, V>,
            $($symbol: system::$name::Unit<V>,)+
            V: $crate::stdlib::fmt::Debug,
        {
            fn fmt(&self, f: &mut $crate::stdlib::fmt::Formatter) -> $crate::stdlib::fmt::Result {
                if let Some(precision) = f.precision() {
                    write!(f, "{:.*?}", precision, self.value)
                }
                else {
                    write!(f, "{:?}", self.value)
                }
                $(.and_then(|_| {
                    let d = $name::to_i32();

                    if 0 != d {
                        write!(f, " {}^{}", $symbol::abbreviation(), d)
                    }
                    else {
                        Ok(())
                    }
                }))+
            }
        }

        impl<$($symbol),+> Dimension for $quantities<$($symbol),+>
        where
            $($symbol: $crate::typenum::Integer,)+
        {
        }

        /// Marker struct to identify the [base units][base] of the
        /// [system of quantities][quantities] to be used in the internal representation of a
        /// [quantity][quantity] value.
        ///
        /// [base]: http://jcgm.bipm.org/vim/en/1.10.html
        /// [quantities]: http://jcgm.bipm.org/vim/en/1.3.html
        /// [quantity]: http://jcgm.bipm.org/vim/en/1.1.html
        #[allow(missing_debug_implementations)]
        #[derive(Clone, Copy)]
        pub struct BaseUnits<$($symbol,)+ V>
        where
            $($symbol: system::$name::Unit<V>,)+
        {
            $($name: $crate::stdlib::marker::PhantomData<$symbol>,)+
            value: $crate::stdlib::marker::PhantomData<V>,
        }

        $(#[$units_attr])*
        pub type $units<V> = BaseUnits<$(system::$name::$unit),+, V>;

        #[doc(hidden)]
        macro_rules! impl_marker_ops {
            ($Trait:ident, $fun:ident) => {
                #[allow(non_camel_case_types)]
                impl<$($name,)+ $($symbol),+> $crate::stdlib::ops::$Trait<$quantities<$($name),+>>
                    for $quantities<$($symbol),+>
                where
                    $($name: $crate::typenum::Integer,)+
                    $($symbol: $crate::typenum::Integer
                        + $crate::stdlib::ops::$Trait<$name>,)+
                    $(<$symbol as $crate::stdlib::ops::$Trait<$name>>::Output: $crate::typenum::Integer,)+
                {
                    type Output = $quantities<
                        $(<$symbol as $crate::stdlib::ops::$Trait<$name>>::Output),+>;

                    fn $fun(self, _rhs: $quantities<$($name),+>) -> Self::Output {
                        unreachable!()
                    }
                }
            };
        }
        impl_marker_ops!(Add, add);
        impl_marker_ops!(Sub, sub);
        impl_marker_ops!(Mul, mul);
        impl_marker_ops!(PartialDiv, partial_div);

        #[doc(hidden)]
        macro_rules! impl_ops {
            (
                $AddSubTrait:ident, $addsub_fun:ident, $addsub_op:tt,
                $AddSubAssignTrait:ident, $addsubassign_fun:ident, $addsubassign_op:tt,
                $MulDivTrait:ident, $muldiv_fun:ident, $muldiv_op:tt,
                $MulDivAssignTrait:ident, $muldivassign_fun:ident, $muldivassign_op:tt,
                $V:ty
            ) =>
            {
                impl<D, Ul, Ur> $crate::stdlib::ops::$AddSubTrait<Quantity<D, Ur, $V>>
                    for Quantity<D, Ul, $V>
                where
                    D: Dimension,
                    Ul: Units<D, $V>,
                    Ur: Units<D, $V>,
                {
                    type Output = Quantity<D, Ul, $V>;

                    #[inline(always)]
                    fn $addsub_fun(self, rhs: Quantity<D, Ur, $V>) -> Self::Output {
                        Quantity {
                            dimension: $crate::stdlib::marker::PhantomData,
                            units: $crate::stdlib::marker::PhantomData,
                            value: self.value
                                $addsub_op (rhs.value * <Ur as Units<D, $V>>::conversion()
                                    / <Ul as Units<D, $V>>::conversion()),
                        }
                    }
                }

                impl<D, Ul, Ur> $crate::stdlib::ops::$AddSubAssignTrait<Quantity<D, Ur, $V>>
                    for Quantity<D, Ul, $V>
                where
                    D: Dimension,
                    Ul: Units<D, $V>,
                    Ur: Units<D, $V>,
                {
                    #[inline(always)]
                    fn $addsubassign_fun(&mut self, rhs: Quantity<D, Ur, $V>) {
                        self.value $addsubassign_op rhs.value * <Ur as Units<D, $V>>::conversion()
                            / <Ul as Units<D, $V>>::conversion();
                    }
                }

                impl<Dl, Dr, Ul, Ur> $crate::stdlib::ops::$MulDivTrait<Quantity<Dr, Ur, $V>>
                    for Quantity<Dl, Ul, $V>
                where
                    Dl: Dimension + $crate::stdlib::ops::$AddSubTrait<Dr>,
                    Dr: Dimension,
                    Ul: Units<Dl, $V> + Units<Dr, $V>
                        + Units<<Dl as $crate::stdlib::ops::$AddSubTrait<Dr>>::Output, $V>,
                    Ur: Units<Dr, $V>,
                    <Dl as $crate::stdlib::ops::$AddSubTrait<Dr>>::Output: Dimension,
                {
                    type Output = Quantity<<Dl as $crate::stdlib::ops::$AddSubTrait<Dr>>::Output,
                        Ul, $V>;

                    #[inline(always)]
                    fn $muldiv_fun(self, rhs: Quantity<Dr, Ur, $V>) -> Self::Output {
                        Quantity {
                            dimension: $crate::stdlib::marker::PhantomData,
                            units: $crate::stdlib::marker::PhantomData,
                            value: self.value
                                $muldiv_op (rhs.value * <Ur as Units<Dr, $V>>::conversion()
                                    / <Ul as Units<Dr, $V>>::conversion()),
                        }
                    }
                }

                impl<D, U> $crate::stdlib::ops::$MulDivTrait<$V> for Quantity<D, U, $V>
                where
                    D: Dimension + $crate::stdlib::ops::$AddSubTrait<One>,
                    U: Units<D, $V>
                        + Units<<D as $crate::stdlib::ops::$AddSubTrait<One>>::Output, $V>,
                    <D as $crate::stdlib::ops::$AddSubTrait<One>>::Output: Dimension,
                {
                    type Output = Quantity<<D as $crate::stdlib::ops::$AddSubTrait<One>>::Output,
                        U, $V>;

                    #[inline(always)]
                    fn $muldiv_fun(self, rhs: $V) -> Self::Output {
                        Quantity {
                            dimension: $crate::stdlib::marker::PhantomData,
                            units: $crate::stdlib::marker::PhantomData,
                            value: self.value $muldiv_op rhs,
                        }
                    }
                }

                impl<D, U> $crate::stdlib::ops::$MulDivTrait<Quantity<D, U, $V>> for $V
                where
                    D: Dimension,
                    U: Units<D, $V>
                        + Units<<One as $crate::stdlib::ops::$AddSubTrait<D>>::Output, $V>,
                    One: $crate::stdlib::ops::$AddSubTrait<D>,
                    <One as $crate::stdlib::ops::$AddSubTrait<D>>::Output: Dimension,
                {
                    type Output = Quantity<<One as $crate::stdlib::ops::$AddSubTrait<D>>::Output,
                        U, $V>;

                    #[inline(always)]
                    fn $muldiv_fun(self, rhs: Quantity<D, U, $V>) -> Self::Output {
                        Quantity {
                            dimension: $crate::stdlib::marker::PhantomData,
                            units: $crate::stdlib::marker::PhantomData,
                            value: self $muldiv_op rhs.value,
                        }
                    }
                }

                impl<D, U> $crate::stdlib::ops::$MulDivAssignTrait<$V> for Quantity<D, U, $V>
                where
                    D: Dimension,
                    U: Units<D, $V>,
                {
                    #[inline(always)]
                    fn $muldivassign_fun(&mut self, rhs: $V) {
                        self.value $muldivassign_op rhs;
                    }
                }
            };
        }

        #[doc(hidden)]
        macro_rules! impl_units {
            ($V:ty) => {
                impl<D, U> Quantity<D, U, $V>
                where
                    D: Dimension,
                    U: Units<D, $V>,
                {
                    /// Returns `true` if this value is `NaN` and `false` otherwise.
                    #[cfg_attr(feature = "clippy", allow(wrong_self_convention))]
                    #[inline(always)]
                    pub fn is_nan(self) -> bool {
                        #[cfg(not(feature = "std"))]
                        #[allow(unused_imports)]
                        use $crate::stdlib::num::*;

                        self.value.is_nan()
                    }

                    /// Returns `true` if this value is positive infinity or negative infinity and
                    /// `false` otherwise.
                    #[cfg_attr(feature = "clippy", allow(wrong_self_convention))]
                    #[inline(always)]
                    pub fn is_infinite(self) -> bool {
                        #[cfg(not(feature = "std"))]
                        #[allow(unused_imports)]
                        use $crate::stdlib::num::*;

                        self.value.is_infinite()
                    }

                    /// Returns `true` if this number is neither infinite nor `NaN`.
                    #[cfg_attr(feature = "clippy", allow(wrong_self_convention))]
                    #[inline(always)]
                    pub fn is_finite(self) -> bool {
                        #[cfg(not(feature = "std"))]
                        #[allow(unused_imports)]
                        use $crate::stdlib::num::*;

                        self.value.is_finite()
                    }

                    /// Returns `true` if the number is neither zero, infinite, subnormal, or `NaN`.
                    #[cfg_attr(feature = "clippy", allow(wrong_self_convention))]
                    #[inline(always)]
                    pub fn is_normal(self) -> bool {
                        #[cfg(not(feature = "std"))]
                        #[allow(unused_imports)]
                        use $crate::stdlib::num::*;

                        self.value.is_normal()
                    }

                    /// Returns the floating point category of the number. If only one property is
                    /// going to be tested, it is generally faster to use the specific predicate
                    /// instead.
                    #[inline(always)]
                    pub fn classify(self) -> $crate::stdlib::num::FpCategory {
                        #[cfg(not(feature = "std"))]
                        #[allow(unused_imports)]
                        use $crate::stdlib::num::*;

                        self.value.classify()
                    }

                    /// Takes the cubic root of a number.
                    ///
                    /// ```
                    /// # use uom::stdlib::marker::PhantomData;
                    /// # use uom::typenum::{P3, Z0};
                    /// # use uom::si::{Quantity, ISQ, SI};
                    #[cfg_attr(feature = "f32", doc = " # use uom::si::f32::*;")]
                    #[cfg_attr(not(feature = "f32"), doc = " # use uom::si::f64::*;")]
                    /// # //use uom::si::volume::cubic_meter;
                    /// // TODO #7 implement Volume.
                    /// //let l: Length = Volume::new::<cubic_meter>(8.0).cbrt();
                    #[cfg_attr(feature = "f32", doc = " let l: Length = Quantity::<ISQ<P3, Z0, Z0, Z0, Z0, Z0, Z0>, SI<f32>, f32>")]
                    #[cfg_attr(not(feature = "f32"), doc = " let l: Length = Quantity::<ISQ<P3, Z0, Z0, Z0, Z0, Z0, Z0>, SI<f64>, f64>")]
                    ///     { dimension: PhantomData, units: PhantomData, value: 8.0, }.cbrt();
                    /// ```
                    #[cfg(feature = "std")]
                    #[inline(always)]
                    pub fn cbrt(self) ->
                        Quantity<<D as $crate::stdlib::ops::PartialDiv<DP3>>::Output, U, $V>
                    where
                        D: $crate::stdlib::ops::PartialDiv<DP3>,
                        U: Units<<D as $crate::stdlib::ops::PartialDiv<DP3>>::Output, $V>,
                        <D as $crate::stdlib::ops::PartialDiv<DP3>>::Output: Dimension,
                    {
                        Quantity {
                            dimension: $crate::stdlib::marker::PhantomData,
                            units: $crate::stdlib::marker::PhantomData,
                            value: self.value.cbrt(),
                        }
                    }

                    /// Takes the reciprocal (inverse) of a number, `1/x`.
                    ///
                    /// ```
                    #[cfg_attr(feature = "f32", doc = " # use uom::si::f32::*;")]
                    #[cfg_attr(not(feature = "f32"), doc = " # use uom::si::f64::*;")]
                    /// # use uom::si::time::second;
                    /// // TODO #30 implement Frequency.
                    /// let f/*: Frequency*/ = Time::new::<second>(1.0).recip();
                    /// ```
                    #[cfg(feature = "std")]
                    #[inline(always)]
                    pub fn recip(self) ->
                        Quantity<<D as $crate::stdlib::ops::Mul<DN1>>::Output, U, $V>
                    where
                        D: $crate::stdlib::ops::Mul<DN1>,
                        U: Units<<D as $crate::stdlib::ops::Mul<DN1>>::Output, $V>,
                        <D as $crate::stdlib::ops::Mul<DN1>>::Output: Dimension,
                    {
                        Quantity {
                            dimension: $crate::stdlib::marker::PhantomData,
                            units: $crate::stdlib::marker::PhantomData,
                            value: self.value.recip(),
                        }
                    }

                    /// Takes the square root of a number. Returns `NaN` if `self` is a negative
                    /// number.
                    ///
                    /// ```
                    #[cfg_attr(feature = "f32", doc = " # use uom::si::f32::*;")]
                    #[cfg_attr(not(feature = "f32"), doc = " # use uom::si::f64::*;")]
                    /// # use uom::si::area::square_meter;
                    /// let l: Length = Area::new::<square_meter>(4.0).sqrt();
                    /// ```
                    #[cfg(feature = "std")]
                    #[inline(always)]
                    pub fn sqrt(self) ->
                        Quantity<<D as $crate::stdlib::ops::PartialDiv<DP2>>::Output, U, $V>
                    where
                        D: $crate::stdlib::ops::PartialDiv<DP2>,
                        U: Units<<D as $crate::stdlib::ops::PartialDiv<DP2>>::Output, $V>,
                        <D as $crate::stdlib::ops::PartialDiv<DP2>>::Output: Dimension,
                    {
                        Quantity {
                            dimension: $crate::stdlib::marker::PhantomData,
                            units: $crate::stdlib::marker::PhantomData,
                            value: self.value.sqrt(),
                        }
                    }

                    /// Returns `e^(self)`, (the exponential function).
                    #[cfg(feature = "std")]
                    #[inline(always)]
                    pub fn exp(self) -> Self {
                        Quantity {
                            dimension: $crate::stdlib::marker::PhantomData,
                            units: $crate::stdlib::marker::PhantomData,
                            value: self.value.exp(),
                        }
                    }

                    /// Returns `2^(self)`.
                    #[cfg(feature = "std")]
                    #[inline(always)]
                    pub fn exp2(self) -> Self {
                        Quantity {
                            dimension: $crate::stdlib::marker::PhantomData,
                            units: $crate::stdlib::marker::PhantomData,
                            value: self.value.exp2(),
                        }
                    }

                    /// Returns the natural logarithm of the number.
                    #[cfg(feature = "std")]
                    #[inline(always)]
                    pub fn ln(self) -> Self {
                        Quantity {
                            dimension: $crate::stdlib::marker::PhantomData,
                            units: $crate::stdlib::marker::PhantomData,
                            value: self.value.ln(),
                        }
                    }

                    /// Returns the logarithm of the number with respect to an abritrary base.
                    #[cfg(feature = "std")]
                    #[inline(always)]
                    pub fn log(self, base: $V) -> Self {
                        Quantity {
                            dimension: $crate::stdlib::marker::PhantomData,
                            units: $crate::stdlib::marker::PhantomData,
                            value: self.value.log(base),
                        }
                    }

                    /// Returns the base 2 logarithm of the number.
                    #[cfg(feature = "std")]
                    #[inline(always)]
                    pub fn log2(self) -> Self {
                        Quantity {
                            dimension: $crate::stdlib::marker::PhantomData,
                            units: $crate::stdlib::marker::PhantomData,
                            value: self.value.log2(),
                        }
                    }

                    /// Returns the base 10 logarithm of the number.
                    #[cfg(feature = "std")]
                    #[inline(always)]
                    pub fn log10(self) -> Self {
                        Quantity {
                            dimension: $crate::stdlib::marker::PhantomData,
                            units: $crate::stdlib::marker::PhantomData,
                            value: self.value.log10(),
                        }
                    }

                    /// Returns the maximum of the two quantities.
                    #[cfg(feature = "std")]
                    #[inline(always)]
                    pub fn max(self, other: Self) -> Self {
                        Quantity {
                            dimension: $crate::stdlib::marker::PhantomData,
                            units: $crate::stdlib::marker::PhantomData,
                            value: self.value.max(other.value),
                        }
                    }

                    /// Returns the minimum of the two quantities.
                    #[cfg(feature = "std")]
                    #[inline(always)]
                    pub fn min(self, other: Self) -> Self {
                        Quantity {
                            dimension: $crate::stdlib::marker::PhantomData,
                            units: $crate::stdlib::marker::PhantomData,
                            value: self.value.min(other.value),
                        }
                    }
                }

                #[allow(non_camel_case_types)]
                impl<$($name,)+ $($symbol),+> Units<$quantities<$($name),+>, $V>
                    for BaseUnits<$($symbol,)+ $V>
                where
                    $($name: $crate::typenum::Integer,)+
                    $($symbol: system::$name::Unit<$V>,)+
                {
                    #[inline(always)]
                    fn conversion() -> $V {
                        #[cfg(not(feature = "std"))]
                        #[allow(unused_imports)]
                        use $crate::stdlib::num::*;

                        1.0 $(* <$symbol as Conversion<$V>>::conversion().powi($name::to_i32()))+
                    }
                }

                impl_ops!(Add, add, +, AddAssign, add_assign, +=,
                    Mul, mul, *, MulAssign, mul_assign, *=,
                    $V);
                impl_ops!(Sub, sub, -, SubAssign, sub_assign, -=,
                    Div, div, /, DivAssign, div_assign, /=,
                    $V);

                impl<D, U> $crate::stdlib::ops::Neg for Quantity<D, U, $V>
                where
                    D: Dimension,
                    U: Units<D, $V>
                {
                    type Output = Quantity<D, U, $V>;

                    #[inline(always)]
                    fn neg(self) -> Self::Output {
                        Quantity {
                            dimension: $crate::stdlib::marker::PhantomData,
                            units: $crate::stdlib::marker::PhantomData,
                            value: -self.value,
                        }
                    }
                }

                impl<D, Ul, Ur> $crate::stdlib::ops::Rem<Quantity<D, Ur, $V>>
                    for Quantity<D, Ul, $V>
                where
                    D: Dimension,
                    Ul: Units<D, $V>,
                    Ur: Units<D, $V>,
                {
                    type Output = Quantity<D, Ul, $V>;

                    #[inline(always)]
                    fn rem(self, rhs: Quantity<D, Ur, $V>) -> Self::Output {
                        Quantity {
                            dimension: $crate::stdlib::marker::PhantomData,
                            units: $crate::stdlib::marker::PhantomData,
                            value: self.value
                                % (rhs.value * <Ur as Units<D, $V>>::conversion()
                                    / <Ul as Units<D, $V>>::conversion()),
                        }
                    }
                }

                impl<D, Ul, Ur> $crate::stdlib::ops::RemAssign<Quantity<D, Ur, $V>>
                    for Quantity<D, Ul, $V>
                where
                    D: Dimension,
                    Ul: Units<D, $V>,
                    Ur: Units<D, $V>,
                {
                    #[inline(always)]
                    fn rem_assign(&mut self, rhs: Quantity<D, Ur, $V>) {
                        self.value %= rhs.value * <Ur as Units<D, $V>>::conversion()
                            / <Ul as Units<D, $V>>::conversion()
                    }
                }
            };
        }
        #[cfg(feature = "f32")]
        impl_units!(f32);
        #[cfg(feature = "f64")]
        impl_units!(f64);

        /// Macro to implement [`quantity`](si/struct.Quantity.html) type aliases for a specific
        /// [system of units][units] and value storage type.
        ///
        /// * `$path`: Path to the module where the [`system!`](macro.system.html) macro was run
        ///   (e.g. `::uom::si`).
        /// * `$V`: Underlying value storage type (e.g. `f32`).
        /// * `$U`: Optional. Base units. Pass as a tuple with the desired units: `(meter, kilogram,
        ///   second, ampere, kelvin, mole, candela)`. The system's base units will be used if no
        ///   value is provided.
        ///
        /// An example invocation is given below for a meter-kilogram-second system setup in the
        /// module `mks` with a system of quantities name `Q`. The optional units parameter to
        /// change the base units is included commented out.
        ///
        /// ```
        /// # #[macro_use] extern crate uom;
        /// # fn main() { }
        /// # mod mks {
        /// #     #[macro_use]
        /// #     mod length {
        /// #         quantity! {
        /// #             /// Length (base unit meter, m<sup>1</sup>).
        /// #             quantity: MksLength; "length";
        /// #             /// Length dimension, m<sup>1</sup>.
        /// #             dimension: Q<P1 /*length*/, Z0 /*mass*/, Z0 /*time*/>;
        /// #             units {
        /// #                 @meter: 1.0E0; "m", "meter", "meters";
        /// #                 @foot: 3.048E-1; "ft", "foot", "feet";
        /// #             }
        /// #         }
        /// #     }
        /// #     #[macro_use]
        /// #     mod mass {
        /// #         quantity! {
        /// #             /// Mass (base unit kilogram, kg<sup>1</sup>).
        /// #             quantity: MksMass; "mass";
        /// #             /// Mass dimension, kg<sup>1</sup>.
        /// #             dimension: Q<Z0 /*length*/, P1 /*mass*/, Z0 /*time*/>;
        /// #             units {
        /// #                 @kilogram: 1.0; "kg", "kilogram", "kilograms";
        /// #             }
        /// #         }
        /// #     }
        /// #     #[macro_use]
        /// #     mod time {
        /// #         quantity! {
        /// #             /// Time (base unit second, s<sup>1</sup>).
        /// #             quantity: MksTime; "time";
        /// #             /// Time dimension, s<sup>1</sup>.
        /// #             dimension: Q<Z0 /*length*/, Z0 /*mass*/, P1 /*time*/>;
        /// #             units {
        /// #                 @second: 1.0; "s", "second", "seconds";
        /// #             }
        /// #         }
        /// #     }
        /// #     system! {
        /// #         /// System of quantities, Q.
        /// #         quantities: Q {
        /// #             length: meter, L;
        /// #             mass: kilogram, M;
        /// #             time: second, T;
        /// #         }
        /// #         /// System of units, U.
        /// #         units: U {
        /// #             MksLength,
        /// #             MksMass,
        /// #             MksTime,
        /// #         }
        /// #     }
        /// mod f32 {
        ///     Q!(mks, f32/*, (centimeter, gram, second)*/);
        /// }
        /// # }
        /// ```
        ///
        /// [units]: http://jcgm.bipm.org/vim/en/1.13.html
        #[macro_export]
        macro_rules! $quantities {
            ($path:path, $V:ty) => {
                quantities!($path, $V, $units; $($quantity),+);
            };
            ($path:path, $V:ty, $U:tt) => {
                quantities!($path, $V; $($name),+; $U; $($quantity),+);
            };
        }
    };
}

/// Macro to implement a [quantity][quantity] and associated [measurement units][measurement]. Note
/// that this macro must be executed in direct submodules of the module where the
/// [`system!`](macro.system.html) macro was executed.
///
/// * `$quantity_attr`: Quantity attributes. Generally used to set documentation comments for the
///   quantity.
/// * `$quantity`: Quantity name (e.g. `Length`).
/// * `$description`: Quantity description (e.g. `"length"`).
/// * `$dim_attr`: Dimension attributes. Generally used to set documentation comments for the
///   quantity's dimension type alias.
/// * `$system`: System of quantities type (e.g. `ISQ`).
/// * `$dimension`: Power of a factor for each base quantity in the system. Power should be
///   represented as a `typenum` type-level integer (e.g. `N1`, `Z0`, `P1`, `P2`, ...).
/// * `$unit`: Unit name (e.g. `meter`, `foot`).
/// * `$conversion`: Conversion from the unit to the base unit of the quantity (e.g. `3.048E-1` to
///   convert `foot` to `meter`).
/// * `$abbreviation`: Unit abbreviation (e.g. `"m"`).
/// * `$singular`: Singular unit description (e.g. `"meter"`).
/// * `$plural`: Plural unit description (e.g. `"meters"`).
///
/// An example invocation is given below for the quantity of length in a meter-kilogram-second
/// system.
///
/// ```
/// # #[macro_use] extern crate uom;
/// # fn main() { }
/// # mod mks {
/// #[macro_use]
/// mod length {
///     quantity! {
///         /// Length (base unit meter, m<sup>1</sup>).
///         quantity: MksLength; "length";
///         /// Length dimension, m<sup>1</sup>.
///         dimension: Q<P1 /*length*/, Z0 /*mass*/, Z0 /*time*/>;
///         units {
///             @meter: 1.0E0; "m", "meter", "meters";
///             @foot: 3.048E-1; "ft", "foot", "feet";
///         }
///     }
/// }
/// #     #[macro_use]
/// #     mod mass {
/// #         quantity! {
/// #             /// Mass (base unit kilogram, kg<sup>1</sup>).
/// #             quantity: MksMass; "mass";
/// #             /// Mass dimension, kg<sup>1</sup>.
/// #             dimension: Q<Z0 /*length*/, P1 /*mass*/, Z0 /*time*/>;
/// #             units {
/// #                 @kilogram: 1.0; "kg", "kilogram", "kilograms";
/// #             }
/// #         }
/// #     }
/// #     #[macro_use]
/// #     mod time {
/// #         quantity! {
/// #             /// Time (base unit second, s<sup>1</sup>).
/// #             quantity: MksTime; "time";
/// #             /// Time dimension, s<sup>1</sup>.
/// #             dimension: Q<Z0 /*length*/, Z0 /*mass*/, P1 /*time*/>;
/// #             units {
/// #                 @second: 1.0; "s", "second", "seconds";
/// #             }
/// #         }
/// #     }
/// #     system! {
/// #         /// System of quantities, Q.
/// #         quantities: Q {
/// #             length: meter, L;
/// #             mass: kilogram, M;
/// #             time: second, T;
/// #         }
/// #         /// System of units, U.
/// #         units: U {
/// #             MksLength,
/// #             MksMass,
/// #             MksTime,
/// #         }
/// #     }
/// #     mod f32 {
/// #         Q!(mks, f32/*, (centimeter, gram, second)*/);
/// #     }
/// # }
/// ```
///
/// [quantity]: http://jcgm.bipm.org/vim/en/1.1.html
/// [measurement]: http://jcgm.bipm.org/vim/en/1.9.html
#[macro_export]
macro_rules! quantity {
    (
        $(#[$quantity_attr:meta])* quantity: $quantity:ident; $description:expr;
        $(#[$dim_attr:meta])* dimension: $system:ident<$($dimension:ident),+>;
        units {
            $($(#[$unit_attr:meta])* @$unit:ident: $conversion:expr;
                $abbreviation:expr, $singular:expr, $plural:expr;)+
        }
    ) =>
    {
        $(#[$dim_attr])*
        #[allow(dead_code)]
        pub type Dimension = super::$system<$($crate::typenum::$dimension),+>;

        $(#[$quantity_attr])*
        #[allow(dead_code)]
        pub type $quantity<U, V> = super::Quantity<Dimension, U, V>;

        /// Marker trait to identify measurement units for the quantity. See
        /// [`Unit`](../trait.Unit.html).
        pub trait Unit<V>: super::Conversion<V> {}

        $(unit!($(#[$unit_attr])* @$unit);

        impl super::Unit for $unit {
            #[inline(always)]
            fn abbreviation() -> &'static str {
                $abbreviation
            }

            #[inline(always)]
            fn singular() -> &'static str {
                $singular
            }

            #[inline(always)]
            fn plural() -> &'static str {
                $plural
            }
        })+

        /// Quantity description.
        #[allow(dead_code)]
        #[inline(always)]
        pub fn description() -> &'static str {
            $description
        }

        #[doc(hidden)]
        macro_rules! impl_quantity {
            ($V:ty) => {
                impl<U> $quantity<U, $V>
                where
                    U: super::Units<Dimension, $V>,
                {
                    /// Create a new quantity from the given value and measurement unit.
                    #[inline(always)]
                    pub fn new<N>(v: $V) -> Self
                    where
                        N: Unit<$V>,
                    {
                        $quantity {
                            dimension: $crate::stdlib::marker::PhantomData,
                            units: $crate::stdlib::marker::PhantomData,
                            value: v * <N as super::Conversion<$V>>::conversion()
                                / <U as super::Units<Dimension, $V>>::conversion(),
                        }
                    }

                    /// Retrieve the value of the quantity in the given measurement unit.
                    #[inline(always)]
                    pub fn get<N>(self, _unit: N) -> $V
                    where
                        N: Unit<$V>,
                    {
                        self.value * <U as super::Units<Dimension, $V>>::conversion()
                            / <N as super::Conversion<$V>>::conversion()
                    }

                    /// Returns the largest integer less than or equal to a number in the given
                    /// measurement unit.
                    #[cfg(feature = "std")]
                    #[inline(always)]
                    pub fn floor<N>(self, _unit: N) -> Self
                    where
                        N: Unit<$V>,
                    {
                        Self::new::<N>(self.get(_unit).floor())
                    }

                    /// Returns the smallest integer less than or equal to a number in the given
                    /// measurement unit.
                    #[cfg(feature = "std")]
                    #[inline(always)]
                    pub fn ceil<N>(self, _unit: N) -> Self
                    where
                        N: Unit<$V>,
                    {
                        Self::new::<N>(self.get(_unit).ceil())
                    }

                    /// Returns the nearest integer to a number in the in given measurement unit.
                    /// Round half-way cases away from 0.0.
                    #[cfg(feature = "std")]
                    #[inline(always)]
                    pub fn round<N>(self, _unit: N) -> Self
                    where
                        N: Unit<$V>,
                    {
                        Self::new::<N>(self.get(_unit).round())
                    }

                    /// Returns the integer part of a number in the given measurement unit.
                    #[cfg(feature = "std")]
                    #[inline(always)]
                    pub fn trunc<N>(self, _unit: N) -> Self
                    where
                        N: Unit<$V>,
                    {
                        Self::new::<N>(self.get(_unit).trunc())
                    }

                    /// Returns the fractional part of a number in the given measurement unit.
                    #[cfg(feature = "std")]
                    #[inline(always)]
                    pub fn fract<N>(self, _unit: N) -> Self
                    where
                        N: Unit<$V>,
                    {
                        Self::new::<N>(self.get(_unit).fract())
                    }
                }

                $(impl Unit<$V> for $unit {}

                impl super::Conversion<$V> for $unit {
                    #[inline(always)]
                    fn conversion() -> $V {
                        $conversion
                    }
                })+
            };
        }
        #[cfg(feature = "f32")]
        impl_quantity!(f32);
        #[cfg(feature = "f64")]
        impl_quantity!(f64);

        #[doc(hidden)]
        #[macro_export]
        macro_rules! $quantity {
            ($U:ty, $V:ty) => {
                $(#[$quantity_attr])*
                #[allow(dead_code)]
                pub type $quantity =
                    system::Quantity<system::$system<$($crate::typenum::$dimension),+>, $U, $V>;
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! quantities {
    ($path:path, $V:ty, $U:ident; $($quantity:ident),+) => {
        use $path as system;

        $($quantity!(system::$U<$V>, $V);)+
    };
    ($path:path, $V:ty; $($name:ident),+; ($($U:ident),+); $($quantity:ident),+) => {
        use $path as system;

        type Units = system::BaseUnits<$(system::$name::$U,)+ $V>;

        $($quantity!(Units, $V);)+
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! unit {
    ($(#[$unit_attr:meta])+ @$unit:ident) => {
        $(#[$unit_attr])*
        #[allow(non_camel_case_types)]
        #[derive(Clone, Copy, Debug)]
        pub struct $unit;
    };
    (@$unit:ident) => {
        /// Measurement unit.
        #[allow(non_camel_case_types)]
        #[derive(Clone, Copy, Debug)]
        pub struct $unit;
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! replace_ty {
    ($_t:tt $sub:ty) => { $sub };
}
