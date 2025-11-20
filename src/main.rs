mod view;
mod util;
mod error;
mod service;
mod domain;
mod controller;

use controller::lotto_controller::LottoController;

fn main() {
    let lotto_controller = LottoController::new();
    lotto_controller.run();
}
