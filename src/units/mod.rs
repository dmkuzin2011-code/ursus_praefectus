//! Physical units and range checks.

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Unit {
    Meter, Millimeter,
    Radian, Degree,
    Newton,
    Volt, Ampere,
    Celsius,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnitError {
    Incompatible { from: Unit, to: Unit },
    NotFinite(f64),
    BelowMin { value: f64, min: f64 },
    AboveMax { value: f64, max: f64 },
}

impl fmt::Display for UnitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UnitError::Incompatible { from, to } =>
                write!(f, "incompatible units: {from:?} -> {to:?}"),
            UnitError::NotFinite(v) => write!(f, "value {v} is not finite"),
            UnitError::BelowMin { value, min } =>
                write!(f, "value {value} below min {min}"),
            UnitError::AboveMax { value, max } =>
                write!(f, "value {value} above max {max}"),
        }
    }
}

impl std::error::Error for UnitError {}

pub fn convert(value: f64, from: Unit, to: Unit) -> Result<f64, UnitError> {
    if !value.is_finite() {
        return Err(UnitError::NotFinite(value));
    }
    if from == to {
        return Ok(value);
    }
    let v = match (from, to) {
        (Unit::Meter, Unit::Millimeter) => value * 1000.0,
        (Unit::Millimeter, Unit::Meter) => value / 1000.0,
        (Unit::Radian, Unit::Degree) => value.to_degrees(),
        (Unit::Degree, Unit::Radian) => value.to_radians(),
        _ => return Err(UnitError::Incompatible { from, to }),
    };
    Ok(v)
}

pub fn ensure_range(value: f64, min: f64, max: f64) -> Result<(), UnitError> {
    if !value.is_finite() {
        return Err(UnitError::NotFinite(value));
    }
    if value < min {
        return Err(UnitError::BelowMin { value, min });
    }
    if value > max {
        return Err(UnitError::AboveMax { value, max });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mm_to_m() {
        assert!((convert(1500.0, Unit::Millimeter, Unit::Meter).unwrap() - 1.5).abs() < 1e-9);
    }

    #[test]
    fn m_to_mm() {
        assert!((convert(1.5, Unit::Meter, Unit::Millimeter).unwrap() - 1500.0).abs() < 1e-9);
    }

    #[test]
    fn deg_to_rad() {
        let r = convert(180.0, Unit::Degree, Unit::Radian).unwrap();
        assert!((r - std::f64::consts::PI).abs() < 1e-9);
    }

    #[test]
    fn incompatible_units_rejected() {
        assert!(matches!(
            convert(1.0, Unit::Volt, Unit::Meter),
            Err(UnitError::Incompatible { .. })
        ));
    }

    #[test]
    fn nan_rejected() {
        assert!(matches!(convert(f64::NAN, Unit::Meter, Unit::Millimeter),
                         Err(UnitError::NotFinite(_))));
    }

    #[test]
    fn range_ok() {
        assert!(ensure_range(24.0, 0.0, 48.0).is_ok());
    }

    #[test]
    fn range_below() {
        assert!(matches!(ensure_range(-1.0, 0.0, 48.0),
                         Err(UnitError::BelowMin { .. })));
    }

    #[test]
    fn range_above() {
        assert!(matches!(ensure_range(50.0, 0.0, 48.0),
                         Err(UnitError::AboveMax { .. })));
    }
}