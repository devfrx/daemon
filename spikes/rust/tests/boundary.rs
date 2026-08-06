use kernel_spike::{build_prompt, summarize, Instruction, Untrusted};

#[test]
fn t3_ereditarieta_una_trasformazione_su_non_fidato_resta_non_fidata() {
    let web = Untrusted::new("x".repeat(200));
    let ridotto: Untrusted = summarize(&web);
    assert_eq!(ridotto.as_str().len(), 50);
}

#[test]
fn t2_la_conversione_esiste_ed_e_esplicita() {
    let web = Untrusted::new("contenuto arbitrario".to_string());
    let promosso: Instruction = web.promote_to_instruction("motivo registrato");
    assert_eq!(promosso.as_str(), "contenuto arbitrario");
}

#[test]
fn il_prompt_si_costruisce_solo_da_istruzioni() {
    let sistema = Instruction::new("sei un assistente".to_string());
    let utente = Instruction::new("ciao".to_string());
    assert_eq!(build_prompt(&sistema, &utente), "sei un assistente\nciao");
}
