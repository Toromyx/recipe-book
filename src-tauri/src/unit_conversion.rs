//! This module implements unit conversion for known units.

#[derive(Debug)]
pub enum Unit {
    Mass(MassUnit),
    Volume(VolumeUnit),
}

#[derive(Debug)]
pub enum MassUnit {
    Kilogram,
    Gram,
    Pound,
}

#[derive(Debug)]
pub enum VolumeUnit {
    Litre,
    Millilitre,
    UsCup,
}

/// This trait enables implementors to [convert](Self::convert) values of type [`f64`] between two of them.
///
/// The implementors need to provide their conversion factors with [`Self::factor`].
/// This conversion factor must always be in relation to the same reference.
pub trait Convert: Sized {
    /// Convert a value from [`Self`] to a target [`Self`].
    fn convert(&self, value: f64, to: &Self) -> f64 {
        let reference_quantity = value * self.factor();
        reference_quantity / to.factor()
    }

    /// Get the factor of the unit to convert the unit to the reference unit.
    fn factor(&self) -> f64;
}

impl Convert for MassUnit {
    /// https://www.nist.gov/pml/special-publication-811/nist-guide-si-appendix-b-conversion-factors/nist-guide-si-appendix-b9#MASSinertia
    fn factor(&self) -> f64 {
        match self {
            MassUnit::Kilogram => 1e0,
            MassUnit::Gram => 1e-3,
            MassUnit::Pound => 4.535924e-1,
        }
    }
}

impl Convert for VolumeUnit {
    /// https://www.nist.gov/pml/special-publication-811/nist-guide-si-appendix-b-conversion-factors/nist-guide-si-appendix-b9#VOLUME
    fn factor(&self) -> f64 {
        match self {
            VolumeUnit::Litre => 1e0,
            VolumeUnit::Millilitre => 1e-3,
            VolumeUnit::UsCup => 2.365882e-1,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::unit_conversion::{Convert, MassUnit, VolumeUnit};

    #[test]
    fn test_pound_conversion() {
        assert_eq!(453.5924, MassUnit::Pound.convert(1.0, &MassUnit::Gram));
    }

    #[test]
    fn test_us_cup_conversion() {
        assert_eq!(
            236.5882,
            VolumeUnit::UsCup.convert(1.0, &VolumeUnit::Millilitre)
        );
    }
}
