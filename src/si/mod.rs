#[macro_use] pub mod prefix;

#[macro_use] pub mod amount_of_substance;
#[macro_use] pub mod electric_current;
#[macro_use] pub mod length;
#[macro_use] pub mod luminous_intensity;
#[macro_use] pub mod mass;
#[macro_use] pub mod thermodynamic_temperature;
#[macro_use] pub mod time;
#[macro_use] pub mod velocity;

system!(SI; SIUnits:
    dimensions {
        length: L;
        mass: M;
        time: T;
        electric_current: I;
        thermodynamic_temperature: Th;
        amount_of_substance: N;
        luminous_intensity: J;
    });

pub type BaseUnits = SIUnits<
    length::units::meter,
    mass::units::kilogram,
    time::units::second,
    electric_current::units::ampere,
    thermodynamic_temperature::units::kelvin,
    amount_of_substance::units::mole,
    luminous_intensity::units::candela>;

pub mod f32 {
    pub type U = super::BaseUnits;
    pub type V = f32;

    amount_of_substance!();
    electric_current!();
    length!();
    luminous_intensity!();
    mass!();
    thermodynamic_temperature!();
    time!();
    velocity!();
}

pub mod f64 {
    pub type U = super::BaseUnits;
    pub type V = f64;

    amount_of_substance!();
    electric_current!();
    length!();
    luminous_intensity!();
    mass!();
    thermodynamic_temperature!();
    time!();
    velocity!();
}
