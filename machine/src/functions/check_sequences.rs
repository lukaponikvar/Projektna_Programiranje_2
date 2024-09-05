use crate::{communication::expected::expected, structs::sequences::SequenceSyntax};

///Funkcija preveri ali so vsa morebiti gnezdena zaporedja v najini lasti ali ne.
pub fn check_sequences(syn: &SequenceSyntax) -> bool {
    let mut result = expected(&syn);
    for seq in &syn.sequences {
        result = result && check_sequences(&seq);
        if result == false {
            break;
        }
    }
    result
}
