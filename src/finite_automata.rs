use std::collections::{HashMap, HashSet};
use std::{fs, io};
use std::fmt::Display;

type State = String;
type Alphabet = char;

#[derive(Debug)]
pub struct FiniteAutomaton {
    states: Vec<State>,
    initial_state: State,
    final_states: HashSet<State>,
    alphabet: Vec<Alphabet>,
    transition_function: HashMap<(State, Alphabet), State>,
}

impl FiniteAutomaton {
    pub fn new(finite_automaton_file_path: &str) -> io::Result<FiniteAutomaton> {
        let file_contents = fs::read_to_string(finite_automaton_file_path)?;
        let mut lines = file_contents.lines();
        let states: Vec<State> = lines.next().unwrap().split_whitespace()
            .map(|state| String::from(state)).collect();
        let initial_state = String::from(
            lines.next().unwrap().split_whitespace().next().unwrap());
        let final_states = lines.next().unwrap().split_whitespace()
            .map(|state| String::from(state)).collect();
        let alphabet = lines.next().unwrap().split_whitespace()
            .map(|state| state.chars().next().unwrap()).collect();
        lines.next().unwrap();

        let mut transition_function = HashMap::new();
        for line in lines {
            let transition_function_element_parts: Vec<&str> = line.split_whitespace().collect();
            let (state, symbol, result_state) = (
                String::from(transition_function_element_parts[0]),
                transition_function_element_parts[1].chars().next().unwrap(),
                String::from(transition_function_element_parts[2])
            );
            transition_function.insert((state, symbol), result_state);
        }

        Ok(FiniteAutomaton {
            states,
            initial_state,
            final_states,
            alphabet,
            transition_function,
        })
    }

    pub fn verify(&self, input_sequence: &str) -> bool {
        let mut current_state = &self.initial_state;
        for char in input_sequence.chars() {
            let transition = (current_state.clone(), char);
            if let Some(next_state) = self.transition_function.get(&transition) {
                current_state = next_state
            }
            else {
                return false
            }
        }
        self.final_states.contains(current_state)
    }
}

fn print_vec_as_math_set<T: Display>(iter: &mut dyn Iterator<Item=T>) {
    print!("{{");
    if let Some(next_item) = iter.next() {
        print!("{}", next_item);
        for next_item in iter {
            print!(", {}", next_item)
        }
    }
    print!("}}")
}

pub fn read_and_print_finite_automaton(path_to_fa_file: &str) {
    let finite_automaton = match FiniteAutomaton::new(path_to_fa_file) {
        Ok(res) => res,
        Err(err) => {
            eprintln!("Could not read FA file: {}", err);
            return
        }
    };

    loop {
        let mut user_input = String::new();
        if let Err(err) = io::stdin().read_line(&mut user_input) {
            eprintln!("Error: {}", err);
            continue
        }
        let mut user_input_iterator = user_input.split_whitespace();
        let user_input = match user_input_iterator.next() {
            None => continue,
            Some(val) => val
        };

        match user_input {
            "states" => {
                println!("States: ");
                print_vec_as_math_set(&mut finite_automaton.states.iter());
                println!()
            },
            "initial" => println!("Initial state: {}", finite_automaton.initial_state),
            "final" => {
                println!("Final states: ");
                print_vec_as_math_set(&mut finite_automaton.final_states.iter());
                println!()
            },
            "alphabet" => {
                println!("Alphabet: ");
                print_vec_as_math_set(&mut finite_automaton.alphabet.iter());
                println!()
            },
            "transitions" => {
                for transition in finite_automaton.transition_function.iter() {
                    println!("({}, {}) -> {}", transition.0.0, transition.0.1, transition.1);
                }
            },
            "check" => {
                match user_input_iterator.next() {
                    None => {
                        eprintln!("Please provide an input sequence to check.");
                        continue;
                    }
                    Some(input_sequence) => {
                        println!("The sequence is{}accepted by the automaton.",
                                 if finite_automaton.verify(input_sequence) {" "} else {" not "})
                    }
                }
            }
            "exit" => return,
            _ => println!("Unsupported menu option.")
        }
    }
}