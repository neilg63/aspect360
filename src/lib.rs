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

  pub fn aspect(&self) -> f64 {
    self.0
  }

  pub fn target(&self) -> f64 {
    self.1
  }

  pub fn distance(&self) -> f64 {
    self.2
  }

  pub fn matched(&self) -> bool {
    self.3
  }

  pub fn orb(&self) -> f64 {
    self.4
  }
}

/// Defines a target aspect with its orb (± tolerance). Used with the find_aspect() method defined in Aspect360
/// All aspects are symmetrical e.g. 120º will also match 240º or -120º
#[derive(Debug, Clone, Copy)]
pub struct AspectOrb(pub f64, pub f64);

impl AspectOrb {

    pub fn target(&self) -> f64 {
        self.0
    }

    pub fn orb(&self) -> f64 {
        self.1
    }
}

/// Provides method to calculate aspect matches from f64 values cast to Ring360 with a target aspect and orb (± tolerance)
pub trait Aspect360 {

  /// Calculate an aspect result with a symmetrical flag (i.e. if false may only be the ± target, 90º => ±90º)
  fn calc_aspect(&self, other: &Ring360, target: f64, orb: f64) -> AspectResult;

  fn find_aspect(&self, other: &Ring360, targets: &[AspectOrb]) -> Option<AspectResult> {
    for aspect_orb in targets {
        let aspect = self.calc_aspect(other, aspect_orb.target(), aspect_orb.orb());
        if aspect.matched() {
            return Some(aspect);
        }
    }
    None
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
