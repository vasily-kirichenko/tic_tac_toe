#[macro_use(c)]
extern crate cute;

mod tic_tac_toe;

use tic_tac_toe::*;

fn render(model: &Model, i: &mut i32) -> () {
    *i += 1;
    println!("[{}] {:?}", i, model)
}

fn main() {
    let mut model = Model::new();
    let view = view();
    let mut i = 0;

    model.update(Msg::Play(Pos::new(0, 0)), |msg| println!("{}", msg));
    render(&model, &mut i);

    model.update(Msg::Play(Pos::new(2, 2)), |msg| println!("{}", msg));
    render(&model, &mut i)
}
