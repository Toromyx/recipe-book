import type { IdentifiableInterface } from "../identifiable-interface.ts";

export const enum Unit {
  MASS_KILOGRAM = "MassKilogram",
  MASS_GRAM = "MassGram",
  MASS_POUND = "MassPound",
  VOLUME_LITRE = "VolumeLitre",
  VOLUME_MILLILITRE = "VolumeMillilitre",
  VOLUME_US_CUP = "VolumeUsCup",
}

export interface UnitNameInterface extends IdentifiableInterface {
  name: string;
  unit: Unit;
}

export interface UnitNameCreateInterface {
  name: string;
  unit: Unit;
}

export interface UnitNameUpdateInterface extends IdentifiableInterface {
  name?: string;
  unit?: Unit;
}
