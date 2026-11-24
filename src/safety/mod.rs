//! Safety limits for robot commands and sensor readings.
//!
//! Pure Rust, no external deps. Same input → same output.
//! Fail-closed: NaN/inf and out-of-band values are rejected.

use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SafetyError {
    BelowMin { name: String, value: f64, min: f64 },
    AboveMax { name: String, value: f64, max: f64 },
    NotFinite { name: String, value: f64 },
    Missing { name: String },
    RuleViolated { rule: &'static str, detail: String },
}

impl fmt::Display for SafetyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SafetyError::BelowMin { name, value, min } =>
                write!(f, "{name}={value} below min {min}"),
            SafetyError::AboveMax { name, value, max } =>
                write!(f, "{name}={value} above max {max}"),
            SafetyError::NotFinite { name, value } =>
                write!(f, "{name}={value} is not finite"),
            SafetyError::Missing { name } =>
                write!(f, "missing field: {name}"),
            SafetyError::RuleViolated { rule, detail } =>
                write!(f, "rule '{rule}' violated: {detail}"),
        }
    }
}

impl std::error::Error for SafetyError {}

#[derive(Debug, Clone)]
pub struct SafetyLimits {
    pub max_speed_mps: f64,
    pub max_force_n: f64,
    pub max_temp_c: f64,
    pub max_voltage_v: f64,
    pub max_current_a: f64,
    pub max_pwm_duty: f64,
    pub max_combined_motion: Option<f64>,
}

impl Default for SafetyLimits {
    fn default() -> Self {
        Self {
            max_speed_mps: 0.5,
            max_force_n: 20.0,
            max_temp_c: 80.0,
            max_voltage_v: 48.0,
            max_current_a: 5.0,
            max_pwm_duty: 100.0,
            max_combined_motion: Some(0.7),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Command {
    pub move_linear_mps: Option<f64>,
    pub move_angular_rps: Option<f64>,
    pub grab_force_n: Option<f64>,
    pub pwm_duty: Option<f64>,
    pub wait_ms: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Decision {
    Allow,
    Clamp { reasons: Vec<String> },
    Reject { reasons: Vec<String> },
}

impl Decision {
    pub fn is_allow(&self) -> bool { matches!(self, Decision::Allow) }
    pub fn is_reject(&self) -> bool { matches!(self, Decision::Reject { .. }) }
}

fn check_finite(name: &str, v: f64) -> Result<(), SafetyError> {
    if !v.is_finite() {
        return Err(SafetyError::NotFinite { name: name.to_string(), value: v });
    }
    Ok(())
}

fn check_range(name: &str, v: f64, min: f64, max: f64) -> Result<(), SafetyError> {
    check_finite(name, v)?;
    if v < min {
        return Err(SafetyError::BelowMin { name: name.to_string(), value: v, min });
    }
    if v > max {
        return Err(SafetyError::AboveMax { name: name.to_string(), value: v, max });
    }
    Ok(())
}

pub fn check_sensor(name: &str, value: f64, min: f64, max: f64) -> Result<(), SafetyError> {
    check_range(name, value, min, max)
}

pub fn check_command(cmd: &Command, limits: &SafetyLimits) -> Decision {
    let mut reasons: Vec<String> = Vec::new();

    if let Some(v) = cmd.move_linear_mps {
        if let Err(e) = check_range("move_linear_mps", v, -limits.max_speed_mps, limits.max_speed_mps) {
            reasons.push(e.to_string());
        }
    }

    if let Some(v) = cmd.move_angular_rps {
        let lim = limits.max_speed_mps * 2.0;
        if let Err(e) = check_range("move_angular_rps", v, -lim, lim) {
            reasons.push(e.to_string());
        }
    }

    if let (Some(l), Some(a), Some(max_combined)) =
        (cmd.move_linear_mps, cmd.move_angular_rps, limits.max_combined_motion)
    {
        let mag = (l * l + a * a).sqrt();
        if mag > max_combined {
            reasons.push(format!("combined motion {mag:.3} exceeds {max_combined:.3}"));
        }
    }

    if let Some(v) = cmd.grab_force_n {
        if let Err(e) = check_range("grab_force_n", v, 0.0, limits.max_force_n) {
            reasons.push(e.to_string());
        }
    }

    if let Some(v) = cmd.pwm_duty {
        if let Err(e) = check_range("pwm_duty", v, 0.0, limits.max_pwm_duty) {
            reasons.push(e.to_string());
        }
    }

    if let Some(v) = cmd.wait_ms {
        if v > 60_000 {
            reasons.push(format!("wait_ms={v} exceeds 60000"));
        }
    }

    if reasons.is_empty() {
        Decision::Allow
    } else {
        Decision::Reject { reasons }
    }
}

pub struct SensorSpec<'a> {
    pub name: &'a str,
    pub value: f64,
    pub min: f64,
    pub max: f64,
}

pub fn check_sensors(specs: &[SensorSpec<'_>]) -> Vec<SafetyError> {
    specs.iter()
        .filter_map(|s| check_sensor(s.name, s.value, s.min, s.max).err())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn limits() -> SafetyLimits {
        SafetyLimits {
            max_speed_mps: 0.5,
            max_force_n: 20.0,
            max_temp_c: 80.0,
            max_voltage_v: 48.0,
            max_current_a: 5.0,
            max_pwm_duty: 100.0,
            max_combined_motion: Some(0.7),
        }
    }

    #[test]
    fn allow_normal_move() {
        let cmd = Command { move_linear_mps: Some(0.3), ..Default::default() };
        assert!(check_command(&cmd, &limits()).is_allow());
    }

    #[test]
    fn reject_over_speed() {
        let cmd = Command { move_linear_mps: Some(2.0), ..Default::default() };
        let d = check_command(&cmd, &limits());
        assert!(d.is_reject());
        if let Decision::Reject { reasons } = d {
            assert!(reasons.iter().any(|r| r.contains("move_linear_mps")));
        }
    }

    #[test]
    fn reject_nan_speed() {
        let cmd = Command { move_linear_mps: Some(f64::NAN), ..Default::default() };
        assert!(check_command(&cmd, &limits()).is_reject());
    }

    #[test]
    fn reject_inf_force() {
        let cmd = Command { grab_force_n: Some(f64::INFINITY), ..Default::default() };
        assert!(check_command(&cmd, &limits()).is_reject());
    }

    #[test]
    fn reject_pwm_over_100() {
        let cmd = Command { pwm_duty: Some(150.0), ..Default::default() };
        assert!(check_command(&cmd, &limits()).is_reject());
    }

    #[test]
    fn reject_combined_motion_too_large() {
        let cmd = Command {
            move_linear_mps: Some(0.5),
            move_angular_rps: Some(0.5),
            ..Default::default()
        };
        assert!(check_command(&cmd, &limits()).is_reject());
    }

    #[test]
    fn allow_combined_motion_within_band() {
        let cmd = Command {
            move_linear_mps: Some(0.3),
            move_angular_rps: Some(0.3),
            ..Default::default()
        };
        assert!(check_command(&cmd, &limits()).is_allow());
    }

    #[test]
    fn reject_wait_too_long() {
        let cmd = Command { wait_ms: Some(120_000), ..Default::default() };
        assert!(check_command(&cmd, &limits()).is_reject());
    }

    #[test]
    fn sensor_in_band_ok() {
        assert!(check_sensor("cpu", 62.0, -40.0, 85.0).is_ok());
    }

    #[test]
    fn sensor_above_max_fails() {
        let e = check_sensor("cpu", 95.0, -40.0, 85.0).unwrap_err();
        match e {
            SafetyError::AboveMax { name, value, max } => {
                assert_eq!(name, "cpu");
                assert_eq!(value, 95.0);
                assert_eq!(max, 85.0);
            }
            other => panic!("unexpected error: {other:?}"),
        }
    }

    #[test]
    fn sensor_nan_fails() {
        assert!(matches!(
            check_sensor("cpu", f64::NAN, -40.0, 85.0),
            Err(SafetyError::NotFinite { .. })
        ));
    }

    #[test]
    fn batch_sensors_reports_only_failures() {
        let specs = [
            SensorSpec { name: "cpu", value: 60.0, min: 0.0, max: 85.0 },
            SensorSpec { name: "motor", value: 90.0, min: 0.0, max: 80.0 },
            SensorSpec { name: "battery", value: 12.0, min: 9.0, max: 16.8 },
        ];
        let errs = check_sensors(&specs);
        assert_eq!(errs.len(), 1);
        assert!(errs[0].to_string().contains("motor"));
    }

    #[test]
    fn defaults_are_sane() {
        let l = SafetyLimits::default();
        assert!(l.max_speed_mps > 0.0);
        assert!(l.max_voltage_v > 0.0);
        assert!(l.max_pwm_duty <= 100.0);
    }
}

    #[test]
    fn test_check_limits() {
        let l = SafetyLimits::default();
        assert!(l.max_speed_mps > 0.0);
        assert!(l.max_voltage_v > 0.0);
        assert!(l.max_pwm_duty <= 100.0);
        assert!(!l.is_valid());
    }