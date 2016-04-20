#[derive(Hash, Eq, PartialEq, Debug)]
/// Describes the current state at which the system is in. At the moment it only allows for one i32 variable but I will extend it to maybe 5 by the end.
pub struct State {
	x_1: i32,
}

impl State {
	// Create a new State
	pub fn new(x_1: i32) -> State {
		State { x_1: x_1}
	}
}

/// A tuple of a state and an action (which is a string), which is used as a key in the q_values HashMap to find the value of taking a certain action while in a certain state
#[derive(Hash, Eq, PartialEq, Debug)]
pub struct Key {
	state: State,
	action: String,
}

impl Key {
	// Create a new State
	pub fn new(state: State, action: String) -> Key {
		Key { state: state, action: action }
	}
}
