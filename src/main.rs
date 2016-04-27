extern crate rand;
mod structs;

use rand::Rng;
use structs::{State, Key};
use std::collections::HashMap;
use std::process::{Command, Stdio};
use std::io::prelude::*;
/// Initializes parameters for Q-learning and runs the game infinitely.
/// 
/// Initializes the Q-value table, the gamma number, and the decision policy used to decide on actions.
///
/// Prints to console the number of times the game has been played, and the score of each iteration.
fn main() {

    // 1. Initialize structures and learning rates for learning and decision policy
    let mut q_values = HashMap::new();
    let gamma : f32 = 0.9;
    let mut iteration :i32 = 0;
    let decision_policy = 1;

    // Run game while printing how many times the game has been played
    loop{
        println!("Iteration number: {}", iteration);
        println!("{}", run_game(&mut q_values, gamma, decision_policy));
        iteration += 1;
    }
}

/// Runs the game and updates the Q-Value table as it plays.
///
/// This can be run on any game that generates output in a certain format (State, Reward, Actions). Stops when the game quits.
///
/// Utilizes child processes to run the game.
///
/// Terminates when the game quits (Once the game quits, program will run into an error when trying to parse the next state. The error is handled by returning the current reward)
fn run_game(q_values: &mut HashMap<Key, i32>, gamma: f32, decision_policy: i32) -> String {

    // 2. Run game (create the child process) and grab: state, reward, actions
    let mut child = Command::new("solution")
                    .stdin(Stdio::piped()).stdout(Stdio::piped())
                    .spawn().unwrap_or_else(|e| { panic!("failed to execute child: {}", e)});
    let mut buffer = std::io::BufReader::new(child.stdout.take().unwrap());
    let mut input = child.stdin.take().unwrap();

    // 2b. Taking in input from the game
    let mut states_initial = String::new();
    let mut rewards_initial = String::new();
    let mut actions_initial = String::new();
    buffer.read_line(&mut states_initial).expect("Failed to read line");
    buffer.read_line(&mut rewards_initial).expect("Failed to read line");
    buffer.read_line(&mut actions_initial).expect("Failed to read line");
    states_initial = states_initial.trim().to_string();
    rewards_initial = rewards_initial.trim().to_string();
    actions_initial = actions_initial.trim().to_string();

    // 2bi. Collecting Actions
    let splitted_actions_initial = actions_initial.split("|");
    let action_vec_initial = splitted_actions_initial.collect::<Vec<&str>>();

    // 2bii. Parsing state as a number
    // If you fail to get a state, that means the game is over and you return the final reward
    let states_initial: i32 = match states_initial.parse() {
        Ok(n) => n,
        _ => return rewards_initial,
    };

    // 3. Decide on next action
    // 3a. Create chosen_action: String, score: i32
    // 3b. Iterate through the vec action_vec
    let mut states = states_initial;
    let mut best_actions = find_best(&q_values, states_initial.clone(), &action_vec_initial);
    let mut previous_action = best_actions[0].clone();
    let mut previous_reward = 0;

    // BEGIN LOOP HERE ========================================================
    loop{

        // 4. Grab next state, reward, actions
        let prev_action = previous_action.clone();
        let prev_state = states.clone();

        // 4a. Send input to the game
        input.write((prev_action.clone() + "\n").as_bytes()).expect("Failed to write to child process");

        // 4b. Taking input from the next run
        let mut next_states = String::new();
        let mut next_reward = String::new();
        let mut next_actions = String::new();
        buffer.read_line(&mut next_states).expect("Failed to read line");
        buffer.read_line(&mut next_reward).expect("Failed to read line");
        buffer.read_line(&mut next_actions).expect("Failed to read line");
        next_states = next_states.trim().to_string();
        next_reward = next_reward.trim().to_string();
        next_actions = next_actions.trim().to_string();

        // 4bi. Collecting Actions
        let next_splitted_actions = next_actions.split("|");
        let next_action_vec = next_splitted_actions.collect::<Vec<&str>>();

        // 4bii. Collecting States
        let next_states: i32 = match next_states.parse() {
            Ok(n) => n,
            _ => {
                return previous_reward.to_string()
            },
        };

        // 4biii. Collecting rewards
        let next_reward: i32 = match next_reward.parse() {
            Ok(n) => n,
            _ => return previous_reward.to_string(),
        };
        states = next_states.clone();

        // 5. Find next action
        best_actions = find_best(&q_values,next_states.clone(), &next_action_vec);
        previous_action = best_actions[0].clone();

        // 6. Update previous state
        let max_reward:f32;
        match q_values.get(&Key::new(State::new(next_states), previous_action.clone().to_string())){
            Some(reward) => max_reward = *reward as f32,
            None => max_reward = 0 as f32,
        }
        let updated_value = next_reward as f32 + gamma * max_reward;
        q_values.insert(Key::new(State::new(prev_state), prev_action), updated_value as i32);
        previous_reward = next_reward;
        previous_action = decide(&next_action_vec, &best_actions, decision_policy);
    }
    // END LOOP HERE ========================================================

}

/// Depending on chance, outputs the actions with the highest reward, or decides on an action depending on the decision policy.
///
/// If the decision policy is 0, it randomly picks an action out of all the actions currently available.
///
/// If the decision policy is 1, it picks the second best action

fn decide(action_vec: &Vec<&str>, best_vec: &Vec<String>, policy: i32) -> String {
    let mut rng = rand::thread_rng();
    let random_number = rng.gen::<f32>();
    if random_number < 0.3 {
        if policy == 0 {
            rand::thread_rng().choose(&action_vec).unwrap().to_string()
        } else {
            best_vec[1].clone()
        }
    } else {
        best_vec[0].clone()
    }
}

/// Looks through actions available and the player's current state, and returns a vector containing two actions that provide the top two rewards
fn find_best(q_values: &HashMap<Key, i32>, current_state: i32, vec: &Vec<&str>) -> Vec<String> {
    let mut best_score = -10000;
    let mut second_score = -10000;
    let mut best_action = String::new();
    let mut second_action = String::new();
    for action in vec.iter(){
        let action_score;
        match q_values.get(&Key::new(State::new(current_state), action.clone().to_string())){
            Some(reward) => action_score = *reward,
            None => action_score = 0,
        }
        if action_score > best_score {
            second_score = best_score.clone();
            second_action = best_action.clone();
            best_action = action.to_string();
            best_score = action_score;
        } else if action_score >= second_score {
            second_score = action_score;
            second_action = action.to_string();
        }
    }
    vec![best_action, second_action]
}