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
  AspectOrb(90.0, 5.0),
  AspectOrb(120.0, 2.0),
  AspectOrb(150.0, 2.0)
];

let aspect_match_opt = angle_1.find_aspect(&angle_2, &targets);
if let Some(aspect_match) = aspect_match_opt {

  println!(
    "{:.6}º and  {:.6}º have an an aspect match of {:.6}º within {:.6}º",
    angle_1,
    angle_2,
    aspect_match.target(),
    aspect_match.distance(),
  );
  // Should read: 98.202928º and  249.325729º have an an aspect match of 150.000000º within 1.122801º
}
```

## Structs
### AspectResult

#### Instance Methods

- *aspect()-> f64*  The aspect of the two angle irrespective of whether they match or not.
- *distance()-> f64*  Angular distance from the target aspect
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
- *is_aspected_f64(other: f64, target: f64, orb: f64) -> bool* 

### Dev notes

This is alpha release.