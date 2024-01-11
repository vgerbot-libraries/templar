mod tokenizer;

use tokenizer::entities::*;

fn tokenize<'a>(text: &'a str) -> Vec<Element<'a>> {
    // 创建一些tokens
    let plain_text_token = Token {
        start_index: 0,
        end_index: 5,
        row: 0,
        col: 0,
        text: "color",
    };
    
    // 创建plaintext
    let plaintext = PlainText { token: plain_text_token };
    
    let at_rule_token = Token {
        start_index: 6,
        end_index: 7,
        row: 0,
        col: 0,
        text: "@",
    };
    let at_rule_name_token = Token {
        start_index: 7,
        end_index: 10,
        row: 0,
        col: 0,
        text: "var"
    };
    // 创建instruction
    let instruction = Instruction {
        at_rule: at_rule_token,
        name: at_rule_name_token,
        expression: Vec::new(),
    };
    
    
    let var_name_token = Token {
        start_index: 10,
        end_index: 13,
        row: 0,
        col: 0,
        text: "hello"
    };
    
    let eq_token = Token {
        start_index: 10,
        end_index: 13,
        row: 0,
        col: 0,
        text: "="
    };
    
    let expression_token = Token {
        start_index: 15,
        end_index: 20,
        row: 0,
        col: 0,
        text: "1+1"
    };
    
    // 创建assignment
    let assignment = Assignment {
        var_name: var_name_token,
        eq: eq_token,
        expression: expression_token,
    };
    
    let mut vec = Vec::new();
    vec.push(
        Element::PlainText(plaintext)
    );
    vec.push(
        Element::Instruction(instruction)
    );
    vec.push(
        Element::Assignment(assignment)
    );
    vec
}

fn main() {
    let result = tokenize("");

    // 打印block
    println!("{:#?}", result);
}
