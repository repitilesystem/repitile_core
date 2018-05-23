//! Regulator trait.

/// Describes a regulator, a device used to regulate part of the environment.
/// As the names suggest, `activate` will activate the regulator and periodically
/// check the current conditions
pub trait Regulator {
    /// Called whenever a profile change is performed, so the regulator
    /// can update its thresholds.
    fn profile_changed(&mut self, profile: &::profile::Profile);
    /// Called periodically with a snapshot of the current conditions
    /// so it can activate if needbe.
    fn update(&mut self, current_conditions: &super::CurrentConditions);
}
