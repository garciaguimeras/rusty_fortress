use super::parser;

pub enum SyntaxRule {
    Error,
    Action(parser::Keyword),
    ActionOnObject(parser::Keyword, String)
}

pub fn analyze_syntax(output: &Vec<parser::OutputAction>) -> Vec<SyntaxRule> {
    let mut rules: Vec<SyntaxRule> = Vec::new();

    let mut iter = output.iter();
    let mut pushed_keyword: Option<&parser::Keyword> = Option::None;
    let mut pushed_object: Option<&str> = Option::None;

    let mut action = iter.next().unwrap_or(&parser::OutputAction::None);
    while *action != parser::OutputAction::None {
        
        if let parser::OutputAction::Keyword(keyword) = action {
            match (pushed_keyword, pushed_object) {
                (Option::Some(k), Option::None) => rules.push(SyntaxRule::Action(k.clone())),
                (Option::Some(k), Option::Some(o)) => rules.push(SyntaxRule::ActionOnObject(k.clone(), o.to_string())),
                _ => {
                    pushed_object = Option::None;
                }
            }
            pushed_keyword = Option::Some(&keyword);
        }
        if let parser::OutputAction::Object(object) = action {
            pushed_object = Option::Some(&object);
        }
        if let parser::OutputAction::Error = action {
            return vec!(SyntaxRule::Error);
        }
        
        action = iter.next().unwrap_or(&parser::OutputAction::None);
    }

    match (pushed_keyword, pushed_object) {
        (Option::Some(k), Option::None) => rules.push(SyntaxRule::Action(k.clone())),
        (Option::Some(k), Option::Some(o)) => rules.push(SyntaxRule::ActionOnObject(k.clone(), o.to_string())),
        _ => {}
    }

    rules
}