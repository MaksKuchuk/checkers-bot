use game_checkers::{checker::Order, game_object::GameObject};

fn main() {
    let mut game = GameObject::new((8, 8));

    loop {
        game.show_board();
        let mut buf = String::new();

        std::io::stdin().read_line(&mut buf).unwrap();
        let buf_vec: Vec<u32> = buf.split(" ").map(|e| e.trim().parse().unwrap()).collect();

        println!(
            "{}",
            game.make_step((buf_vec[0], buf_vec[1]), (buf_vec[2], buf_vec[3]))
        );

        match game.is_win() {
            None => {
                println!("nobodywin")
            }
            Some(Order::WHITE) => println!("white win"),
            Some(Order::BLACK) => println!("black win"),
        }
    }
}
