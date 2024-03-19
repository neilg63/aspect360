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

#[test]
fn test_best_aspects() {
  let lng_1 = 192.928202;
  let lng_2 = 249.325729;

  let angle_1 = lng_1.to_360();
  let angle_2 = lng_2.to_360();


  let septile_degree = 360.0 / 7.0;

  let targets = [
    AspectOrb(30.0, 5.0), // semisextile
    AspectOrb(45.0, 8.0), // semisquare
    AspectOrb(septile_degree, 9.0), // septile
    AspectOrb(60.0, 8.0), // sextile
    AspectOrb(90.0, 8.0), // square
    AspectOrb(120.0, 8.0), // trine
    AspectOrb(150.0, 10.0) // quincunx
  ];

  let aspect_matches = angle_1.find_aspects(&angle_2, &targets);

  // Two aspects are matched
  assert_eq!(aspect_matches.len(), 2);

  let first_match = angle_1.find_aspect(&angle_2, &targets);

  // Two aspects are matched
  assert_eq!(first_match.unwrap().target(), septile_degree);

  // However, this best match is sextile and not septile, which would have been matched first
  let aspect_match = angle_1.find_best_aspect(&angle_2, &targets);

  assert!(aspect_match.is_some());

  assert!(aspect_match.unwrap().matched());

  assert_eq!(aspect_match.unwrap().target(), 60.0);

}
