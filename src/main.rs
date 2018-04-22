#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]
#![allow(dead_code)]
#![feature(trace_macros)]

#[macro_use(c)]
extern crate cute;

trace_macros!(true);

mod tic_tac_toe;

use tic_tac_toe::*;

fn render(model: &Model, i: &mut i32) -> () {
    *i += 1;
    println!("[{}] {:?}", i, model)
}

fn main() {
    let mut model = Model::new();
    let _view = view();
    let mut i = 0;

    model.update(Msg::Play(Pos::new(0, 0)), |msg| println!("{}", msg));
    render(&model, &mut i);

    model.update(Msg::Play(Pos::new(2, 2)), |msg| println!("{}", msg));
    render(&model, &mut i);
}

enum Planet {
    Mercury,
    Venus,
    Earth,
    Mars { color: i32 },
    Jupiter,
    Saturn,
    Uranus,
    Neptune,
}

use Planet::*;

impl Planet {
    fn mass(&self) -> f64 {
        match self {
            Mercury => 3.303e+23,
            Venus => 4.869e+24,
            Earth => 5.976e+24,
            Mars { .. } => 6.421e+23,
            Jupiter => 1.9e+27,
            Saturn => 5.688e+26,
            Uranus => 8.686e+25,
            Neptune => 1.024e+26,
        }
    }

    fn radius(&self) -> f64 {
        match self {
            Mercury => 2.4397e6,
            Venus => 6.0518e6,
            Earth => 6.37814e6,
            Mars { .. } => 3.3972e6,
            Jupiter => 7.1492e7,
            Saturn => 6.0268e7,
            Uranus => 2.5559e7,
            Neptune => 2.4746e7,
        }
    }

    const G: f64 = 6.67300E-11;
    fn surface_gravity(&self) -> f64 { Planet::G * self.mass() / (self.radius() * self.radius()) }
    fn surface_weight(&self, other_mass: f64) -> f64 { other_mass * self.surface_gravity() }
}

fn test() {
    let p = Mars { color: 42 };
    let _thing = match p {
        Mars { color } => p.mass() * f64::from(color),
        _ => p.mass()
    };
}

