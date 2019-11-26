use std::fmt;

enum StateAction {
    Keep,
    Move(String),
    End
}

impl Clone for StateAction {
    fn clone(&self) -> Self {
        match self {
            StateAction::Keep => StateAction::Keep,
            StateAction::Move(txt) => StateAction::Move(txt.clone()),
            StateAction::End => StateAction::End
        }
    }
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

impl Clone for InputAction {
    fn clone(&self) -> Self {
        match self {
            InputAction::Keep => InputAction::Keep,
            InputAction::Next => InputAction::Next
        }
    }
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

type RuleResult = (StateAction, InputAction, OutputAction);
type StateRule = (String, RuleResult);

struct State {
    name: String,
    rules: Vec<StateRule>,
    default_rule: StateRule
}

impl State {

    fn new(name: String) -> State {
        State {
            name: name,
            rules: Vec::new(),
            default_rule: (String::from(""), (StateAction::Keep, InputAction::Keep, OutputAction::None))
        }
    }

    fn add_rule(mut self, text: String, result: RuleResult) -> State {
        self.rules.push((text, result));
        self
    }

    fn set_default_rule(mut self, result: RuleResult) -> State {
        self.default_rule = (self.default_rule.0, result);
        self
    }

    fn clone_and_replace_output(&self, text: &str, rule_result: &RuleResult) -> RuleResult {
        let mut cloned = rule_result.clone();
        if let OutputAction::Keyword(txt) = cloned.2 {
            cloned.2 = OutputAction::Keyword(txt.replace("{TEXT}", &text));
        }
        if let OutputAction::Object(txt) = cloned.2 {
            cloned.2 = OutputAction::Object(txt.replace("{TEXT}", &text));
        }
        cloned.clone()
    }

    fn next_state(&self, text: &str) -> RuleResult {
        for rule in self.rules.iter() {
            if text == rule.0 {
                return self.clone_and_replace_output(&text, &rule.1);
            }
        }
        self.clone_and_replace_output(&text, &self.default_rule.1)
    }

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
            State::new(String::from("initial_state"))
                .add_rule(String::from("open"), (StateAction::Move(String::from("i_open")), InputAction::Next, OutputAction::Keyword(String::from("open"))))
                .set_default_rule((StateAction::Move(String::from("unknown_state")), InputAction::Keep, OutputAction::None)),
        
            State::new(String::from("unknown_state"))
                .set_default_rule((StateAction::End, InputAction::Keep, OutputAction::Error)),

            State::new(String::from("i_open"))
                .add_rule(String::from(""), (StateAction::Move(String::from("f_open")), InputAction::Keep, OutputAction::None))
                .set_default_rule((StateAction::Keep, InputAction::Next, OutputAction::Object(String::from("{TEXT}")))),

            State::new(String::from("f_open"))
                .set_default_rule((StateAction::End, InputAction::Keep, OutputAction::None))
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

            let applied_rule_result = current_state.next_state(&word);

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