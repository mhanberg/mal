use crate::types::*;

pub fn pr_str(data: MalType) -> String {
    match data {
        MalNumber => data.to_string,
        MalSymbol{name} => name,
        MalList => {
            data.iter()
                .map(|d| pr_str(d))
                .join(" ")
                .collect();
        }
    }
}
