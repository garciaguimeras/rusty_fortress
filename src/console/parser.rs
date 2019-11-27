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

pub enum OutputAction {
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

struct StateRule {
    input: String,
    result: RuleResult
}

impl StateRule {

    fn rule(name: &str) -> StateRule {
        StateRule { 
            input: name.to_string(), 
            result: (StateAction::Keep, InputAction::Keep, OutputAction::None) 
        }
    }

    fn default_rule() -> StateRule {
        Self::rule("")
    }

    fn keyword_rule(name: &str, next_state: &str) -> StateRule {
        Self::rule(name).set_move_state(next_state).set_next_input().set_keyword_output()
    }

    fn end_rule() -> StateRule {
        Self::default_rule().set_end_state()
    }

    fn error_rule() -> StateRule {
        Self::default_rule().set_end_state().set_error_output()
    }

    fn set_end_state(mut self) -> StateRule {
        self.result.0 = StateAction::End;
        self
    }

    fn set_move_state(mut self, text: &str) -> StateRule {
        self.result.0 = StateAction::Move(text.to_string());
        self
    }

    fn set_next_input(mut self) -> StateRule {
        self.result.1 = InputAction::Next;
        self
    }

    fn set_error_output(mut self) -> StateRule {
        self.result.2 = OutputAction::Error;
        self
    }

    fn set_keyword_output(mut self) -> StateRule {
        self.result.2 = OutputAction::Keyword("".to_string());
        self
    }

    fn set_object_output(mut self) -> StateRule {
        self.result.2 = OutputAction::Object("".to_string());
        self
    }

}

struct State {
    name: String,
    rules: Vec<StateRule>,
    default_rule: StateRule
}

impl State {

    fn build(name: &str) -> State {
        State {
            name: name.to_string(),
            rules: Vec::new(),
            default_rule: StateRule::default_rule()
        }
    }

    fn add_rule(mut self, rule: StateRule) -> State {
        self.rules.push(rule);
        self
    }

    fn set_default_rule(mut self, rule: StateRule) -> State {
        self.default_rule = rule;
        self
    }

    fn clone_and_replace_output(&self, text: &str, rule_result: &RuleResult) -> RuleResult {
        let mut cloned = rule_result.clone();
        if let OutputAction::Keyword(_) = cloned.2 {
            cloned.2 = OutputAction::Keyword(String::from(text));
        }
        if let OutputAction::Object(_) = cloned.2 {
            cloned.2 = OutputAction::Object(String::from(text));
        }
        cloned.clone()
    }

    fn next_state(&self, text: &str) -> RuleResult {
        for rule in self.rules.iter() {
            if text == rule.input {
                return self.clone_and_replace_output(&text, &rule.result);
            }
        }
        self.clone_and_replace_output(&text, &self.default_rule.result)
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

    pub fn build() -> StateMachine {
        let states = vec!(
            State::build("initial_state")
                .add_rule(StateRule::keyword_rule("help", "default_intermediate_state"))
                .add_rule(StateRule::keyword_rule("?", "default_intermediate_state"))
                .add_rule(StateRule::keyword_rule("exit", "default_intermediate_state"))
                .add_rule(StateRule::keyword_rule("quit", "default_intermediate_state"))
                .add_rule(StateRule::keyword_rule("open", "i_open"))
                .set_default_rule(StateRule::default_rule().set_move_state("unknown_state")),
        
            State::build("unknown_state")
                .set_default_rule(StateRule::error_rule()),

            State::build("default_intermediate_state")
                .add_rule(StateRule::rule("").set_move_state("final_intermediate_state"))
                .set_default_rule(StateRule::default_rule().set_next_input().set_object_output()),

            State::build("final_intermediate_state")
                .set_default_rule(StateRule::end_rule()),

            State::build("i_open")
                .add_rule(StateRule::rule("").set_move_state("f_open"))
                .add_rule(StateRule::rule("with").set_move_state("i_openwith"))
                .set_default_rule(StateRule::default_rule().set_next_input().set_object_output()),

            State::build("i_openwith")
                .set_default_rule(StateRule::keyword_rule("with", "default_intermediate_state")),

            State::build("f_open")
                .set_default_rule(StateRule::end_rule())
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

    pub fn parse_line(&self, text: &str) -> Vec<OutputAction> {
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
        //for output_state in output.iter() {
        //    println!("{}", output_state);
        //}
        output
    }
}