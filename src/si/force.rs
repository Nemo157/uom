//! Force (base unit newton, kg<sup>1</sup> · m<sup>1</sup> · s<sup>-2</sup>).

quantity! {
    /// Force (base unit newton, kg<sup>1</sup> · m<sup>1</sup> · s<sup>-2</sup>).
    quantity: Force; "force";
    /// Force dimension, kg<sup>1</sup> · m<sup>1</sup> · s<sup>-2</sup>.
    dimension: ISQ<
        P1,     // length
        P1,     // mass
        N2,     // time
        Z0,     // electric current
        Z0,     // thermodynamic temperature
        Z0,     // amount of substance
        Z0>;    // luminous intensity
    units {
        @yottanewton: prefix!(yotta); "YN", "yottanewton", "yottanewtons";
        @zettanewton: prefix!(zetta); "ZN", "zettanewton", "zettanewtons";
        @exanewton: prefix!(exa); "EN", "exanewton", "exanewtons";
        @petanewton: prefix!(peta); "PN", "petanewton", "petanewtons";
        @teranewton: prefix!(tera); "TN", "teranewton", "teranewtons";
        @giganewton: prefix!(giga); "GN", "giganewton", "giganewtons";
        @meganewton: prefix!(mega); "MN", "meganewton", "meganewtons";
        @kilonewton: prefix!(kilo); "kN", "kilonewton", "kilonewtons";
        @hectonewton: prefix!(hecto); "hN", "hectonewton", "hectonewtons";
        @decanewton: prefix!(deca); "daN", "decanewton", "decanewtons";
        /// Derived unit of force.
        @newton: prefix!(none); "N", "newton", "newtons";
        @decinewton: prefix!(deci); "dN", "decinewton", "decinewtons";
        @centinewton: prefix!(centi); "cN", "centinewton", "centinewtons";
        @millinewton: prefix!(milli); "mN", "millinewton", "millinewtons";
        @micronewton: prefix!(micro); "µN", "micronewton", "micronewtons";
        @nanonewton: prefix!(nano); "nN", "nanonewton", "nanonewtons";
        @piconewton: prefix!(pico); "pN", "piconewton", "piconewtons";
        @femtonewton: prefix!(femto); "fN", "femtonewton", "femtonewtons";
        @attonewton: prefix!(atto); "aN", "attonewton", "attonewtons";
        @zeptonewton: prefix!(zepto); "zN", "zeptonewton", "zeptonewtons";
        @yoctonewton: prefix!(yocto); "yN", "yoctonewton", "yoctonewtons";
    }
}

#[cfg(test)]
mod tests {
    storage_types! {
        use num::One;
        use si::quantities::*;
        use si::force as f;
        use si::length as l;
        use si::mass as m;
        use si::time as t;
        use tests::Test;

        #[test]
        fn check_dimension() {
            let _: Force<V> = (Mass::new::<m::kilogram>(V::one())
                    * Length::new::<l::meter>(V::one()))
                / (Time::new::<t::second>(V::one()) * Time::new::<t::second>(V::one()));
        }

        #[test]
        fn check_units() {
            test::<m::yottagram, l::meter, t::second, f::zettanewton>();
            test::<m::zettagram, l::meter, t::second, f::exanewton>();
            test::<m::exagram, l::meter, t::second, f::petanewton>();
            test::<m::petagram, l::meter, t::second, f::teranewton>();
            test::<m::teragram, l::meter, t::second, f::giganewton>();
            test::<m::gigagram, l::meter, t::second, f::meganewton>();
            test::<m::megagram, l::meter, t::second, f::kilonewton>();
            test::<m::kilogram, l::meter, t::second, f::newton>();
            test::<m::gram, l::meter, t::second, f::millinewton>();
            test::<m::milligram, l::meter, t::second, f::micronewton>();
            test::<m::microgram, l::meter, t::second, f::nanonewton>();
            test::<m::nanogram, l::meter, t::second, f::piconewton>();
            test::<m::picogram, l::meter, t::second, f::femtonewton>();
            test::<m::femtogram, l::meter, t::second, f::attonewton>();
            test::<m::attogram, l::meter, t::second, f::zeptonewton>();
            test::<m::zeptogram, l::meter, t::second, f::yoctonewton>();

            test::<m::kilogram, l::yottameter, t::second, f::yottanewton>();
            test::<m::kilogram, l::zettameter, t::second, f::zettanewton>();
            test::<m::kilogram, l::exameter, t::second, f::exanewton>();
            test::<m::kilogram, l::petameter, t::second, f::petanewton>();
            test::<m::kilogram, l::terameter, t::second, f::teranewton>();
            test::<m::kilogram, l::gigameter, t::second, f::giganewton>();
            test::<m::kilogram, l::megameter, t::second, f::meganewton>();
            test::<m::kilogram, l::kilometer, t::second, f::kilonewton>();
            test::<m::kilogram, l::hectometer, t::second, f::hectonewton>();
            test::<m::kilogram, l::decameter, t::second, f::decanewton>();
            test::<m::kilogram, l::meter, t::second, f::newton>();
            test::<m::kilogram, l::decimeter, t::second, f::decinewton>();
            test::<m::kilogram, l::centimeter, t::second, f::centinewton>();
            test::<m::kilogram, l::millimeter, t::second, f::millinewton>();
            test::<m::kilogram, l::micrometer, t::second, f::micronewton>();
            test::<m::kilogram, l::nanometer, t::second, f::nanonewton>();
            test::<m::kilogram, l::picometer, t::second, f::piconewton>();
            test::<m::kilogram, l::femtometer, t::second, f::femtonewton>();
            test::<m::kilogram, l::attometer, t::second, f::attonewton>();
            test::<m::kilogram, l::zeptometer, t::second, f::zeptonewton>();
            test::<m::kilogram, l::meter, t::second, f::newton>();

            test::<m::kilogram, l::meter, t::terasecond, f::yoctonewton>();
            test::<m::kilogram, l::meter, t::gigasecond, f::attonewton>();
            test::<m::kilogram, l::meter, t::megasecond, f::piconewton>();
            test::<m::kilogram, l::meter, t::kilosecond, f::micronewton>();
            test::<m::kilogram, l::meter, t::decasecond, f::centinewton>();
            test::<m::kilogram, l::meter, t::decisecond, f::hectonewton>();
            test::<m::kilogram, l::meter, t::millisecond, f::meganewton>();
            test::<m::kilogram, l::meter, t::microsecond, f::teranewton>();
            test::<m::kilogram, l::meter, t::nanosecond, f::exanewton>();
            test::<m::kilogram, l::meter, t::picosecond, f::yottanewton>();

            fn test<
                M: m::Conversion<V>,
                L: l::Conversion<V>,
                T: t::Conversion<V>,
                F: f::Conversion<V>>()
            {
                Test::assert_approx_eq(&Force::new::<F>(V::one()),
                    &((Mass::new::<M>(V::one())
                        * Length::new::<L>(V::one()))
                        / (Time::new::<T>(V::one()) * Time::new::<T>(V::one()))));
            }
        }
    }
}
