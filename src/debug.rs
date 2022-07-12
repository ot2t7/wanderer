use crate::token::TokenizedFunction;

pub fn debug_tokens(func: &TokenizedFunction) {
    for i in &func.instructions {
        let to_token: Vec<i64> = (&i.token_offsets)
            .clone()
            .iter_mut()
            .map(|v| *v + i.current_token)
            .collect();
        println!("{:<12}{:<12?}   {:?}", i.current_token, i.instruction.op_code, to_token);
    }
}