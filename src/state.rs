use action::Action;

pub trait State {
    fn get_player_just_moved(&self) -> usize;

    ///Actions must be translated into u32 for generic processing
    fn get_actions(&self) -> Vec<u32>;
    fn get_action_strings(&self) -> Vec<String>;

    /// Perform the move according to the rules of the game
    fn do_action(&mut self, action: u32);

    /// Given a player, return a result number from 0.0 - 1.0
    fn get_result(&self, player: usize) -> f32;
}
