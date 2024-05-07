[![mirror](https://img.shields.io/badge/mirror-github-blue)](https://github.com/neilg63/aspect360)
[![crates.io](https://img.shields.io/crates/v/aspect360.svg)](https://crates.io/crates/aspect360)
[![docs.rs](https://docs.rs/aspect360/badge.svg)](https://docs.rs/aspect360)

# aspect360: Calculate Aspects between two Angles

This crate builds up on the *ring360* library to calculate aspects between two angles with a target aspect and orb or an array of aspect/orb pairs.

This crate re-exports *ring360*, so there is no need to install it separately.

### Add and subtract degree values
```rust
/// Cast two 64-bit floats to Ring360 values (re-exported from )
 let angle = 98.922.to_360();
let angle_2 = 197.2938.to_360();
let aspect_2 = angle.calc_aspect(&angle_2, 90.0, 2.0);
println!("{:?}", aspect_2);

let angle_1 = 74.7.to_360(); 
let angle_2 = 164.4.to_360();

let result = angle_1.calc_aspect(&angle_2, 90.0, 2.0);

println!("{:.1}º and {:.1}º are {:.1}º apart, {:.1}º from the target aspect of {:.1}º with an orb of {:.1}. Matched: {}",
    angle_1.degrees(),
    angle_2.degrees(),
    result.aspect(),
    result.distance(),
    result.target(),
    result.orb(),
    result.matched(),
);
/// Should read: 74.7º and 164.4º are 89.7º apart, -0.3 from the target aspect of 90.0º with an orb of 2.0. Match: true
```

### Find the best aspect match from many options
```rust
let lng_1 = 98.202928;
let lng_2 = 249.325729;

let angle_1 = lng_1.to_360();
let angle_2 = lng_2.to_360();

let targets = [
  AspectOrb(0.0, 8.0), // conjunction
  AspectOrb(90.0, 5.0), // square
  AspectOrb(120.0, 3.0), // trine
  AspectOrb(150.0, 2.0), // quincunx
  AspectOrb(180.0, 4.0) // opposition
];

let aspect_match_opt = angle_1.find_aspect(&angle_2, &targets);
if let Some(aspect_match) = aspect_match_opt {

  println!(
    "{:.6}º and  {:.6}º have an an aspect match of {:.0}º within {:.6}º",
    angle_1,
    angle_2,
    aspect_match.target(),
    aspect_match.distance(),
  );
  // Should read: 98.202928º and  249.325729º have an an aspect match of 150º within 1.122801º
}
```

## Find the best matched aspect where aspect definitions may overlap
In practice, where aspect definitions cannot overlap, the find_aspect() method will be more efficient as it will return the first matched AspectOrb object.
However, when definitions overlap (e.g. a sextile is only around 8.57º away from septile), the first match may not be the best match as the example below illustrates.
```rust
  let lng_1 = 192.928202;
  let lng_2 = 249.325729;

  let angle_1 = lng_1.to_360();
  let angle_2 = lng_2.to_360();


  let septile_degree = 360.0 / 7.0; // 51.42857142857143

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
  println!("{} aspects have been matched within the specified ranges", aspect_matches.len());
  // yields a vector with 2 matched AspectResults 
  // Two aspects are matched.
  

  let first_match = angle_1.find_aspect(&angle_2, &targets);
  // yields the first matched aspect even if it's not best aspect. In this case a septile with a wide orb of 9.0
  println!("The first match is {}º from the target aspect of {}º", first_match.unwrap().divergence(), first_match.unwrap().target());
  // prints: The first match is 4.968955571428566º from the target aspect of 51.42857142857143º

  // However, this best match is sextile and not septile, which would have been matched first
  let best_aspect_match = angle_1.find_best_aspect(&angle_2, &targets);
  // Yields a sextile
  
  println!("The best match is {}º from the target aspect of {}º", best_aspect_match.unwrap().divergence(), best_aspect_match.unwrap().target());
  // prints: The best match is 3.6024730000000034º from the target aspect of 60º
```

## Structs
### AspectResult

#### Instance Methods

- *aspect()-> f64*  The aspect of the two angle irrespective of whether they match or not.
- *distance()-> f64*  Angular distance from the target aspect. May yield negative values
- *divergence()-> f64*  Absolute angular distance from the target aspect. May only yield zero or positive values
- *target()-> f64*  Target aspect, which is always symmetrical, e.g. 90º will match ±90 or ±270
- *matched()-> bool*  True if the angles are aspected with the target within the specified orb.
- *orb() -> f64* The tolerance of the aspect match

#### AspectOrb

A simple tuple struct with the target aspect and an orb, both 64-bit floats.

### Instance Methods

- *target()-> f64*  The target aspect
- *orb() -> f64* The tolerance of the aspect match

## Traits

### Aspect360

This trait is implemented only for *Ring360*, but any 64-bit float can be cast to a Ring360 via the *.to_360()* extension method and variant methods ending in *_f64* accept normal f64 values as comparison angles, which will be normalised within the 0º to º360 range.

- *calc_aspect(other: &Ring360, target: f64, orb: f64) -> AspectResult*
- *is_aspected(other: &Ring360, target: f64, orb: f64) -> bool* 
- *calc_aspect_f64(other: f64, target: f64, orb: f64) -> AspectResult*
- *find_aspect(other: &Ring360, targets: &[AspectOrb]) -> Option<AspectResult> 
- *find_aspects(other: &Ring360, targets: &[AspectOrb]) -> Vec<AspectResult> 
- *find_best_aspects(other: &Ring360, targets: &[AspectOrb]) -> Option<AspectResult> 
- *is_aspected_f64(other: f64, target: f64, orb: f64) -> bool* 

### Dev notes

This is an alpha release. Version 0.1.4 introduced two new methods find_aspects() and find_best_aspect() deal with situations where aspect definitions (AspectOrb) may overlap.