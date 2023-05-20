import type { Unit } from "./entity/unit-name-interface.ts";

export type UnitConversion = {
  values: ConvertedValue[];
};

export type ConvertedValue = {
  value: number;
  unit: Unit;
};
