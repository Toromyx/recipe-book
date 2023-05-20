//! This module implements the [`tauri::command`] for unit conversion.
//!
//! The decision to which units to convert, happens inside [`ConversionBuilder::build`].

use serde::Serialize;

use crate::{
    entity::unit_name::unit::Unit as EntityUnit,
    unit_conversion::{Convert, MassUnit, Unit, VolumeUnit},
};

/// This struct represents a finished unit conversion.
#[derive(Debug, Serialize)]
pub struct Conversion {
    pub values: Vec<ConvertedValue>,
}

impl Conversion {
    pub fn builder(original_value: f64, original_unit: Unit) -> ConversionBuilder {
        ConversionBuilder::new(original_value, original_unit)
    }
}

#[derive(Debug, Serialize)]
pub struct ConvertedValue {
    pub value: f64,
    pub unit: EntityUnit,
}

pub struct ConversionBuilder {
    original_value: f64,
    original_unit: Unit,
}

impl ConversionBuilder {
    pub fn new(original_value: f64, original_unit: Unit) -> Self {
        Self {
            original_value,
            original_unit,
        }
    }

    /// Build the [conversion](Conversion), including only metric units.
    pub fn build(self) -> Conversion {
        let values = match self.original_unit {
            Unit::Mass(mass_unit) => vec![
                ConvertedValue {
                    value: mass_unit.convert(self.original_value, &MassUnit::Kilogram),
                    unit: EntityUnit::from(Unit::Mass(MassUnit::Kilogram)),
                },
                ConvertedValue {
                    value: mass_unit.convert(self.original_value, &MassUnit::Gram),
                    unit: EntityUnit::from(Unit::Mass(MassUnit::Gram)),
                },
            ],
            Unit::Volume(volume_unit) => vec![
                ConvertedValue {
                    value: volume_unit.convert(self.original_value, &VolumeUnit::Litre),
                    unit: EntityUnit::from(Unit::Volume(VolumeUnit::Litre)),
                },
                ConvertedValue {
                    value: volume_unit.convert(self.original_value, &VolumeUnit::Millilitre),
                    unit: EntityUnit::from(Unit::Volume(VolumeUnit::Millilitre)),
                },
            ],
        };
        Conversion { values }
    }
}

#[tauri::command]
pub async fn unit_convert(value: f64, unit: EntityUnit) -> Conversion {
    Conversion::builder(value, Unit::from(unit)).build()
}
