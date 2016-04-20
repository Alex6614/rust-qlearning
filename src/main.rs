extern crate rand;
mod structs;

use rand::Rng;
use structs::{State, Key};
use std::collections::HashMap;
use std::io;
use std::process::{Command, ChildStdout, Stdio};
use std::io::prelude::*;
/// This is the main function that will interact with the system/game. In terms of how it works, it creates a HashMap that stores the reward for taking an action at a particular state. After peforming the action, the value is updated with the reward found in the next state, and the max reward that can be gotten from any actions taken from the next state. This causes a form of backpropogation which allows for the algorithm to know how to get to the state with the highest reward.
/// 
/// For the time being, this program is not too complicated. It is simply split between a file that contains the main function that loops through the game and performs the algorithm, and another file that contains other structs that are useful.
///
/// GOALS FOR NEXT WEEK: be able to interact with the modified castle game I created, refine the loop, introduce randomness in deciding on how to find the next action, split up main() into separate functions to better see how things fit together and allow for orthogonal coding.
fn main() {
    println!("Hello, world!");

    // 1. Initialize structures and learning rates for learning
    let mut q_values = HashMap::new();
    let gamma : f32 = 0.1;

    // Initialize other things...
    // Run game
    loop{
        println!("{}", run_game(&mut q_values, gamma));
    }
}

fn run_game(q_values: &mut HashMap<Key, i32>, gamma: f32) -> String {
    // 2. Run game and grab: state, reward, actions
    // 2a. TODO: Run the game
    let mut child = Command::new("solution")
                    .stdin(Stdio::piped()).stdout(Stdio::piped())
                    .spawn().unwrap_or_else(|e| { panic!("failed to execute child: {}", e)});
    let mut buffer = std::io::BufReader::new(child.stdout.take().unwrap());
    let mut input = child.stdin.take().unwrap();
    // 2b. Taking in input from the game
    // (EXTENSION: expect more than one state on a line, split by |)
    let mut states_initial = String::new();
    let mut rewards_initial = String::new();
    let mut actions_initial = String::new();
    buffer.read_line(&mut states_initial);
    buffer.read_line(&mut rewards_initial);
    buffer.read_line(&mut actions_initial);
    states_initial = states_initial.trim().to_string();
    rewards_initial = rewards_initial.trim().to_string();
    actions_initial = actions_initial.trim().to_string();
    // 2bi. Collecting Actions
    let mut splitted_actions_initial = actions_initial.split("|");
    let action_vec_initial = splitted_actions_initial.collect::<Vec<&str>>();

    // (PROTOTYPE: assuming states will always be a number)
    let mut states_initial: i32 = match states_initial.parse() {
        Ok(n) => n,
        Err(err) => return rewards_initial,
    };
    // 3. Decide on next action
    // 3a. Create chosen_action: String, score: i32
    let mut chosen_action = "";
    let mut score = -1;
    // 3b. Iterate through the vec action_vec
    let mut states = states_initial;
    // let mut previous_action = chosen_action.to_string();
    let mut previous_action = decide(&q_values, states_initial.clone(),action_vec_initial);
    let mut previous_reward = 0;
    // BEGIN LOOP HERE ========================================================
    loop{
        // 4. Grab next state, reward, actions
        // Be sure not to overwrite previous states, and chosen_action

        let prev_action = previous_action.clone();
        let prev_state = states.clone();
        // 4a. TODO: Rerun the game with prev_action as the input
        input.write((prev_action.clone() + "\n").as_bytes());
        // 4b. Taking input from the next run
        let mut next_states = String::new();
        let mut next_reward = String::new();
        let mut next_actions = String::new();
        buffer.read_line(&mut next_states);
        buffer.read_line(&mut next_reward);
        buffer.read_line(&mut next_actions);
        next_states = next_states.trim().to_string();
        next_reward = next_reward.trim().to_string();
        next_actions = next_actions.trim().to_string();
        // 4bi. Collecting Actions
        let mut next_splitted_actions = next_actions.split("|");
        let next_action_vec = next_splitted_actions.collect::<Vec<&str>>();
        // (PROTOTYPE: assuming states will always be a number)
        let mut next_states: i32 = match next_states.parse() {
            Ok(n) => n,
            Err(err) => return previous_reward.to_string(),
        };
        // (PROTOTYPE: assuming states will always be a number)
        let mut next_reward: i32 = match next_reward.parse() {
            Ok(n) => n,
            Err(err) => return previous_reward.to_string(),
        };
        states = next_states.clone();

        // 5. Find next action
        previous_action = decide(&q_values,next_states.clone(), next_action_vec);
        // 6. Update previous state
        // println!("The previous action was:{}", prev_action);
        // println!("The current action is:{}", previous_action);
        // println!("The previous state was:{}", prev_state);
        // println!("The current state is:{}", next_states);
        let mut max_reward:f32 = 0 as f32;
        match q_values.get(&Key::new(State::new(next_states), previous_action.clone().to_string())){
            Some(reward) => max_reward = *reward as f32,
            None => max_reward = 0 as f32,
        }
        let mut updated_value = next_reward as f32 + gamma * max_reward;
        q_values.insert(Key::new(State::new(prev_state), prev_action), updated_value as i32);

        previous_reward = next_reward;
    }
    // END LOOP HERE ========================================================
}


fn decide(q_values: &HashMap<Key, i32>, current_state: i32, vec: Vec<&str>) -> String {
    let mut rng = rand::thread_rng();
    let random_number = rng.gen::<f32>();

    if random_number < 0.01 {
        rand::thread_rng().choose(&vec).unwrap().to_string()
    } else {
        let mut score = -1;
        let mut chosen_action = String::new();
        for action in vec.iter(){
            let mut action_score = 0;
            match q_values.get(&Key::new(State::new(current_state), action.clone().to_string())){
                Some(reward) => action_score = *reward,
                None => action_score = 0,
            }
            if action_score > score {
                chosen_action = action.to_string();
                score = action_score
            }
        }
        chosen_action
    }
}