use ring360::*;
use aspect360::*;

#[cfg(test)]


#[test]
fn test_aspects() {
  let lng_1 = 98.202928;
  let lng_2 = 187.932742;
  let lng_3 = 7.932742;
  let angle_1 = lng_1.to_360();
  let angle_2 = lng_2.to_360();
  let target = 90.0;
  let aspect = angle_1.calc_aspect(&angle_2, target, 2.0);
  assert!(aspect.matched());
  assert!(aspect.distance() < 2.0);
  assert_eq!(aspect.distance(), lng_2 - lng_1 - 90.0);
  assert_eq!(aspect.target(), 90.0);

  let aspect_2 = angle_1.calc_aspect_f64(lng_3, target, 2.0);
  
  assert!(aspect_2.matched());
  
  let target_2 = 120.0;
  let aspect_3 = angle_1.calc_aspect(&angle_2, target_2, 2.0);

  assert_eq!(aspect_3.matched(), false);
}

#[test]
fn test_find_aspects() {
  let lng_1 = 98.202928;
  let lng_2 = 249.325729;

  let angle_1 = lng_1.to_360();
  let angle_2 = lng_2.to_360();

  let targets = [
    AspectOrb(90.0, 5.0),
    AspectOrb(120.0, 2.0),
    AspectOrb(150.0, 2.0)
  ];

  let aspect_match = angle_1.find_aspect(&angle_2, &targets);

  assert!(aspect_match.is_some());

  assert!(aspect_match.unwrap().matched());

  assert_eq!(aspect_match.unwrap().target(), 150.0);

}
