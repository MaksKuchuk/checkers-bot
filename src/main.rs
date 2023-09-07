use game_checkers::controller::Controller;

#[macroquad::main("Checkers")]
async fn main() {
    let mut controller = Controller::new();

    controller.run_player2player_game().await;
}
