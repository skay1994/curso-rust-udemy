enum Pagamento {
    Dinheiro(f32),
    CreditoCartao(bool, f32),
    _DebitoCartao(bool, f32),
}

fn main() {
    let pessoa_pagamento:Pagamento = Pagamento::CreditoCartao(false, 100f32);

    match pessoa_pagamento {
        Pagamento::Dinheiro(f) => println!("A pessoa pagou em Dinheiro! {}", f),
        Pagamento::CreditoCartao(true, f) => println!("A pessoa pagou em Cartão de Credito! {}", f),
        Pagamento::CreditoCartao(false, f) => println!("O pagamento em Cartão de credito não funciono, o valor {} não foi pago", f),
        _ => {}
    }
}