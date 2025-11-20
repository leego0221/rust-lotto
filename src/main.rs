mod controller;
mod domain;
mod error;
mod service;
mod util;
mod view;

use controller::LottoController;

fn main() {
    let lotto_controller = LottoController::new();
    lotto_controller.run();
}
