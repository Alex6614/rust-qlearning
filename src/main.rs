mod structs;

use structs::{State, Key};
use std::collections::HashMap;
use std::io;
/// This is the main function that will interact with the system/game. In terms of how it works, it creates a HashMap that stores the reward for taking an action at a particular state. After peforming the action, the value is updated with the reward found in the next state, and the max reward that can be gotten from any actions taken from the next state. This causes a form of backpropogation which allows for the algorithm to know how to get to the state with the highest reward.
/// 
/// For the time being, this program is not too complicated. It is simply split between a file that contains the main function that loops through the game and performs the algorithm, and another file that contains other structs that are useful.
///
/// GOALS FOR NEXT WEEK: be able to interact with the modified castle game I created, refine the loop, introduce randomness in deciding on how to find the next action, split up main() into separate functions to better see how things fit together and allow for orthogonal coding.
fn main() {
    println!("Hello, world!");

    // 1. Initialize structures and learning rates for learning
    let mut q_values = HashMap::new();
    let gamma = 0.1;

    // 2. Run game and grab: state, reward, actions

    // 2a. TODO: Run the game

    // 2b. Taking in input from the game
    // (EXTENSION: expect more than one state on a line, split by |)
    let mut states = String::new();
    let mut rewards = String::new();
    let mut actions = String::new();

    io::stdin().read_line(&mut states);
    io::stdin().read_line(&mut rewards);
    io::stdin().read_line(&mut actions);
    states = states.trim().to_string();
    rewards = rewards.trim().to_string();
    actions = actions.trim().to_string();
    // 2bi. Collecting Actions
    let mut splitted_actions = actions.split("|");
    let action_vec = splitted_actions.collect::<Vec<&str>>();

    // (PROTOTYPE: assuming states will always be a number)
    let mut states: i32 = states.parse().ok().expect("Wanted a number");

    // BEGIN LOOP HERE
    // 3. Decide on next action
    // 3a. Create chosen_action: String, score: i32
    let mut chosen_action = "";
    let mut score = -1;
    // 3b. Iterate through the vec action_vec
    for next_action in action_vec.iter(){
    	// 3bi. Create next_action: String, this_score: i32
    	let mut this_score: i32 = 0;
    	let current_state = states;
    	// 3bii. Get the corresponding score from the HashMap
    	match q_values.get(&Key::new(State::new(current_state), next_action)){
    		Some(reward) => this_score = *reward,
    		None => this_score = 0,
    	}
    	// 3biii. Check if this_score > score and update chosen_action if needed
    	if this_score > score {
    		chosen_action = next_action;
    		score = this_score;
    	}
    }

    // 4. Grab next state, reward, actions
    // Be sure not to overwrite previous states, and chosen_action

    let prev_action = chosen_action;
    let prev_state = states;

    // 4a. TODO: Rerun the game

    // 4b. Taking input from the next run
    let mut states = String::new();
    let mut rewards = String::new();
    let mut actions = String::new();

    io::stdin().read_line(&mut states);
    io::stdin().read_line(&mut rewards);
    io::stdin().read_line(&mut actions);
    states = states.trim().to_string();
    rewards = rewards.trim().to_string();
    actions = actions.trim().to_string();

    

    // 5. Update previous state

    // (PROTOTYPE: assuming states will always be a number)
    let mut rewards: i32 = rewards.parse().ok().expect("Wanted a number");

    // prev_reward = rewards + gamm
    // q_values.insert(Key::new(State::new(prev_state), prev_action), )


    // END LOOP HERE

    let string = "";
    q_values.insert(Key::new(State::new(0), string), 99);
    println!("{:?}", q_values.get(&Key::new(State::new(0), string)));
}