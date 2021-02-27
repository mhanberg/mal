pub type MalForm = String;
pub type MalList = Vec<MalForm>;
pub struct MalSymbol {
    name: String,
}
pub type MalNumber = i32;
pub enum MalType {
    MalList,
    MalForm,
    MalSymbol,
}
