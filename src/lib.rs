pub use ring360::*;

/// Defines an aspect result
/// All angle pairs have an aspect, but only some match the target within the specified orb (± tolerance)
#[derive(Debug, Clone, Copy)]
pub struct AspectResult(pub f64, pub f64, pub f64, pub bool, pub f64);

impl AspectResult {
  pub fn calculate(target: f64, angle: f64, orb: f64) -> Self {
    let mut distance = target.angle_360(angle);
    if target < Ring360::half_turn() && target > 0f64 {
      let inverse_target = Ring360::BASE - target;
      let distance_2 = inverse_target.angle_360(angle);
      if distance_2.abs() < distance.abs() {
        distance = distance_2;
      }
    }
    let neg_orb = 0f64 - orb;
    let distance_abs = distance.abs();
    let in_range = distance_abs >= neg_orb && distance_abs <= orb;
    AspectResult(angle, target, distance, in_range, orb)
  }

  /// aspect between two angles
  pub fn aspect(&self) -> f64 {
    self.0
  }

  /// target aspect between two angles
  pub fn target(&self) -> f64 {
    self.1
  }

  /// distance between the true aspect and the target, may be negative or positive depending on the direction
  pub fn distance(&self) -> f64 {
    self.2
  }

  /// absolute distance between the true aspect and the target. May only be positive
  pub fn divergence(&self) -> f64 {
    self.2.abs()
  }

  /// does the aspect distance from the target fall within the specified range (orb)
  pub fn matched(&self) -> bool {
    self.3
  }

  /// ± tolerance or range for a valid match
  pub fn orb(&self) -> f64 {
    self.4
  }
}

/// Defines a target aspect with its orb (± tolerance). Used with the find_aspect() method defined in Aspect360
/// All aspects are symmetrical e.g. 120º will also match 240º or -120º
#[derive(Debug, Clone, Copy)]
pub struct AspectOrb(pub f64, pub f64);

impl AspectOrb {

  /// target aspect
  pub fn target(&self) -> f64 {
      self.0
  }

  /// ± tolerance or orb for a valid match
  pub fn orb(&self) -> f64 {
      self.1
  }
}

/// Provides method to calculate aspect matches from f64 values cast to Ring360 with a target aspect and orb (± tolerance)
pub trait Aspect360 {

  /// Calculate an aspect result with a symmetrical flag (i.e. if false may only be the ± target, 90º => ±90º)
  fn calc_aspect(&self, other: &Ring360, target: f64, orb: f64) -> AspectResult;

  /// find the first matched aspect. If no aspects fall within the specified orbs, None will be returned
  /// This method is faster than calling find_best_aspect, as it will return first matched target aspect and not evaulate any others
  /// It's preferable to find_best_aspect where 
  fn find_aspect(&self, other: &Ring360, targets: &[AspectOrb]) -> Option<AspectResult> {
    for aspect_orb in targets {
        let aspect = self.calc_aspect(other, aspect_orb.target(), aspect_orb.orb());
        if aspect.matched() {
            return Some(aspect);
        }
    }
    None
  }

  /// find all matching aspects, where they may potentially overlap
  fn find_aspects(&self, other: &Ring360, targets: &[AspectOrb]) -> Vec<AspectResult> {
    let mut matched_aspects: Vec<AspectResult> = Vec::new();
    for aspect_orb in targets {
        let aspect = self.calc_aspect(other, aspect_orb.target(), aspect_orb.orb());
        if aspect.matched() {
          matched_aspects.push(aspect);
        }
    }
    matched_aspects
  }

  /// Find the nearest matching aspect, if two aspects could potentially overlap.
  /// The method will return the nearest aspect wrapped in a Some Option.
  /// If no aspects fall within the specified orbs, None will be returned
  fn find_best_aspect(&self, other: &Ring360, targets: &[AspectOrb]) -> Option<AspectResult> {
    let mut matched_aspects = self.find_aspects(other, targets);
    if matched_aspects.is_empty() {
      None
    } else {
      matched_aspects.sort_by(|a, b| a.divergence().partial_cmp(&b.divergence()).unwrap());
      matched_aspects.first().map(|ar| *ar)
    }
  }

  /// Calculate an aspect from a normal f64 value representing a degree
  fn calc_aspect_f64(&self, other: f64, target: f64, orb: f64) -> AspectResult {
    self.calc_aspect(&other.to_360(), target, orb)
  }

  /// Calculate an aspect with symmetrical logic and return true if it's within the orb
  fn is_aspected(&self, other: &Ring360, target: f64, orb: f64) -> bool {
    self.calc_aspect(other, target, orb).matched()
  }

  /// Calculate an aspect with symmetrical logic from a normal f64 value and return true if it's within the orb
  fn is_aspected_f64(&self, other: f64, target: f64, orb: f64) -> bool {
    self.calc_aspect(&other.to_360(), target, orb).matched()
  }
}

/// Implement only the core calc_aspect() method from which all other extension methods derive
impl Aspect360 for Ring360 {

  /// Calculate an aspect result with a symmetrical flag (i.e. if false may only be the ± target, 90º => ±90º)
  fn calc_aspect(&self, other: &Ring360, target: f64, orb: f64) -> AspectResult {
    let angle = self.angle(*other);
    AspectResult::calculate(target, angle, orb)
  }

}
