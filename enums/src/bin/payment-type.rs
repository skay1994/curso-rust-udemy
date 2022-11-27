enum Pagamento {
    Dinheiro,
    CreditoCartao,
    DebitoCartao,
}

fn main() {
    let pessoa_pagamento:Pagamento = Pagamento::DebitoCartao;

    match pessoa_pagamento {
        Pagamento::Dinheiro => println!("A pessoa pagou em Dinheiro!"),
        Pagamento::CreditoCartao => println!("A pessoa pagou em Cartão de Credito!"),
        _ => {}
    }
}