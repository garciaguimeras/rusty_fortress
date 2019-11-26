use std::fmt;

enum StateAction {
    Keep,
    Move(String),
    End
}

impl PartialEq for StateAction {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (StateAction::Keep, StateAction::Keep) => true,
            (StateAction::End, StateAction::End) => true,
            (StateAction::Move(txt1), StateAction::Move(txt2)) => txt1 == txt2,
            _ => false
        }
    }
}

enum InputAction {
    Keep,
    Next
}

enum OutputAction {
    None,
    Error,
    Keyword(String),
    Object(String)
}

impl fmt::Display for OutputAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OutputAction::None => write!(f, "OutputAction::None"),
            OutputAction::Error => write!(f, "OutputAction::Error"),
            OutputAction::Keyword(txt) => write!(f, "OutputAction::Keyword: {}", txt),
            OutputAction::Object(txt) => write!(f, "OutputAction::Object: {}", txt)
        }
    }
}

impl Clone for OutputAction {
    fn clone(&self) -> Self {
        match self {
            OutputAction::None => OutputAction::None,
            OutputAction::Error => OutputAction::Error,
            OutputAction::Keyword(txt) => OutputAction::Keyword(txt.clone()),
            OutputAction::Object(txt) => OutputAction::Object(txt.clone())
        }
    }
}

type StateRule = Fn(&str) -> (StateAction, InputAction, OutputAction);

struct State {
    name: String,
    next_state: Box<StateRule>
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "State: {}", self.name)
    }
}

pub struct StateMachine {
    states: Vec<State>    
}

impl StateMachine {

    pub fn create() -> StateMachine {
        let states = vec!(
            State {
                name: String::from("initial_state"),
                next_state: Box::new(|txt| {
                    if txt == "open" {
                        return (StateAction::Move(String::from("i_open")), 
                                InputAction::Next, 
                                OutputAction::Keyword(String::from("open")));
                    }
                    (StateAction::Move(String::from("unknown_state")), 
                     InputAction::Keep,
                     OutputAction::None)
                })
            },
            State {
                name: String::from("unknown_state"),
                next_state: Box::new(|_| { (StateAction::End, InputAction::Keep, OutputAction::Error) })
            },
    
            State {
                name: String::from("i_open"),
                next_state: Box::new(|txt| {
                    if txt != "" {
                        return (StateAction::Keep, 
                                InputAction::Next, 
                                OutputAction::Object(String::from(txt)));
                    } 
                    (StateAction::Move(String::from("f_open")), InputAction::Keep, OutputAction::None)
                })
            },
            State {
                name: String::from("f_open"),
                next_state: Box::new(|_| { (StateAction::End, InputAction::Keep, OutputAction::None) })
            }
        );

        StateMachine {
            states: states
        }
    }

    fn find_by_name(&self, name: &str) -> Option<&State> {
        self.states.iter().find(|s| s.name == name)
    }

    fn mix_output_state(&self, output: Vec<OutputAction>) -> Vec<OutputAction> {
        let mut new_output: Vec<OutputAction> = Vec::new();
        let mut new_txt = String::from("");
        
        for output_state in output.iter() {
            if let OutputAction::Object(txt) = output_state {
                new_txt = if new_txt == "" { txt.to_string() } else { format!("{} {}", new_txt, txt) }
            }
            else {
                if new_txt != "" {
                    new_output.push(OutputAction::Object(new_txt));
                    new_txt = String::from("");
                }
                new_output.push(output_state.clone());
            }
        }
        if new_txt != "" {
            new_output.push(OutputAction::Object(new_txt));
        }

        new_output
    }

    pub fn parse_line(&self, text: &str) {
        let words: Vec<String> = text.trim().split(' ')                       
            .map(|w| w.to_lowercase())
            .collect();
        let mut words_iter = words.iter();
        let empty_str = String::from(""); 

        let mut running = true;
        let mut current_state = self.find_by_name("initial_state").unwrap();
        let mut word = words_iter.next().unwrap_or(&empty_str);
        let mut output: Vec<OutputAction> = Vec::new();
        
        while running {

            let applied_rule_result = (*(current_state.next_state))(&word);

            match applied_rule_result.2 {
                OutputAction::None => {},
                _ => output.push(applied_rule_result.2)
            }
            
            match applied_rule_result.0 {
                StateAction::Keep => {},
                StateAction::Move(n) => current_state = self.find_by_name(&n).unwrap(),
                StateAction::End => running = false
            }
            
            if running {
                match applied_rule_result.1 {
                    InputAction::Keep => {},
                    InputAction::Next => word = words_iter.next().unwrap_or(&empty_str)
                }
            }

        }

        output = self.mix_output_state(output);
        for output_state in output.iter() {
            println!("{}", output_state);
        }

    }
}