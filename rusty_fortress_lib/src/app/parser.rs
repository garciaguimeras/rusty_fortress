use std::fmt;

#[derive(Clone, PartialEq)]
enum StateAction {
    Keep,
    Move(String),
    End
}

#[derive(Clone)]
enum InputAction {
    Keep,
    Next
}

#[derive(Clone, PartialEq)]
pub enum Keyword {
    Help,
    Quit, 
    Open,
    OpenWith,
    View, 
    Take,
    GoThrough
}

impl fmt::Display for Keyword {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Keyword::Help => write!(f, "Help"),
            Keyword::Quit => write!(f, "Quit"),
            Keyword::Open => write!(f, "Open"),
            Keyword::OpenWith => write!(f, "With"),
            Keyword::View => write!(f, "View"),
            Keyword::Take => write!(f, "Take"),
            Keyword::GoThrough => write!(f, "GoThrough")
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum OutputAction {
    None,
    Error,
    Keyword(Keyword),
    Object(String)
}

impl fmt::Display for OutputAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OutputAction::None => write!(f, "OutputAction::None"),
            OutputAction::Error => write!(f, "OutputAction::Error"),
            OutputAction::Keyword(k) => write!(f, "OutputAction::Keyword: {}", k),
            OutputAction::Object(txt) => write!(f, "OutputAction::Object: {}", txt)
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

    fn keyword_rule(name: &str, next_state: &str, keyword: Keyword) -> StateRule {
        Self::rule(name).set_move_state(next_state).set_next_input().set_keyword_output(keyword)
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

    fn set_keyword_output(mut self, keyword: Keyword) -> StateRule {
        self.result.2 = OutputAction::Keyword(keyword);
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
        if let OutputAction::Keyword(k) = cloned.2 {
            cloned.2 = OutputAction::Keyword(k);
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
                .add_rule(StateRule::keyword_rule("help", "default_intermediate_state", Keyword::Help))
                .add_rule(StateRule::keyword_rule("?", "default_intermediate_state", Keyword::Help))
                .add_rule(StateRule::keyword_rule("exit", "default_intermediate_state", Keyword::Quit))
                .add_rule(StateRule::keyword_rule("quit", "default_intermediate_state", Keyword::Quit))
                .add_rule(StateRule::keyword_rule("view", "default_intermediate_state", Keyword::View))
                .add_rule(StateRule::keyword_rule("take", "default_intermediate_state", Keyword::Take))
                .add_rule(StateRule::keyword_rule("open", "i_open", Keyword::Open))
                .add_rule(StateRule::rule("go").set_move_state("i_go").set_next_input())
                .set_default_rule(StateRule::default_rule().set_move_state("unknown_state")),
        
            State::build("unknown_state")
                .set_default_rule(StateRule::error_rule()),

            State::build("default_intermediate_state")
                .add_rule(StateRule::rule("").set_move_state("default_final_state"))
                .set_default_rule(StateRule::default_rule().set_next_input().set_object_output()),

            State::build("default_final_state")
                .set_default_rule(StateRule::end_rule()),

            State::build("i_open")
                .add_rule(StateRule::rule("").set_move_state("f_open"))
                .add_rule(StateRule::rule("with").set_move_state("i_openwith"))
                .set_default_rule(StateRule::default_rule().set_next_input().set_object_output()),

            State::build("i_openwith")
                .set_default_rule(StateRule::keyword_rule("with", "default_intermediate_state", Keyword::OpenWith)),

            State::build("f_open")
                .set_default_rule(StateRule::end_rule()),

            State::build("i_go")
                .add_rule(StateRule::keyword_rule("through", "default_intermediate_state", Keyword::GoThrough))
                .set_default_rule(StateRule::error_rule())
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