//! This module implements the [`Unit`] enum for usage in the [unit name entity](super::Model).

use sea_orm::{entity::prelude::*, ActiveValue, IntoActiveValue};
use serde::{Deserialize, Serialize};

use crate::{
    unit_conversion,
    unit_conversion::{MassUnit, VolumeUnit},
};

/// This enum is invertibly related to [`unit_conversion::Unit`], but necessary for usage in the ORM.
#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "String(Some(1))")]
pub enum Unit {
    #[sea_orm(string_value = "MassKilogram")]
    MassKilogram,
    #[sea_orm(string_value = "MassGram")]
    MassGram,
    #[sea_orm(string_value = "MassPound")]
    MassPound,
    #[sea_orm(string_value = "VolumeLitre")]
    VolumeLitre,
    #[sea_orm(string_value = "VolumeMillilitre")]
    VolumeMillilitre,
    #[sea_orm(string_value = "VolumeUsCup")]
    VolumeUsCup,
}

impl IntoActiveValue<Unit> for Unit {
    fn into_active_value(self) -> ActiveValue<Unit> {
        ActiveValue::Set(self)
    }
}

impl From<unit_conversion::Unit> for Unit {
    fn from(value: unit_conversion::Unit) -> Self {
        match value {
            unit_conversion::Unit::Mass(mass_unit) => match mass_unit {
                MassUnit::Kilogram => Self::MassKilogram,
                MassUnit::Gram => Self::MassGram,
                MassUnit::Pound => Self::MassPound,
            },
            unit_conversion::Unit::Volume(volume_unit) => match volume_unit {
                VolumeUnit::Litre => Self::VolumeLitre,
                VolumeUnit::Millilitre => Self::VolumeMillilitre,
                VolumeUnit::UsCup => Self::VolumeUsCup,
            },
        }
    }
}

impl From<Unit> for unit_conversion::Unit {
    fn from(value: Unit) -> Self {
        match value {
            Unit::MassKilogram => unit_conversion::Unit::Mass(MassUnit::Kilogram),
            Unit::MassGram => unit_conversion::Unit::Mass(MassUnit::Gram),
            Unit::MassPound => unit_conversion::Unit::Mass(MassUnit::Pound),
            Unit::VolumeLitre => unit_conversion::Unit::Volume(VolumeUnit::Litre),
            Unit::VolumeMillilitre => unit_conversion::Unit::Volume(VolumeUnit::Millilitre),
            Unit::VolumeUsCup => unit_conversion::Unit::Volume(VolumeUnit::UsCup),
        }
    }
}

#[cfg(test)]
mod tests {
    use sea_orm::Iterable;

    use super::*;

    #[test]
    fn test_unit_string_value() {
        for unit in Unit::iter() {
            assert_eq!(
                format!("\"{}\"", unit.to_value()),
                serde_json::to_string(&unit).unwrap(),
            )
        }
    }

    #[test]
    fn test_unit_from() {
        for unit in Unit::iter() {
            assert_eq!(
                unit.clone(),
                Unit::from(unit_conversion::Unit::from(unit.clone())),
            )
        }
    }
}
