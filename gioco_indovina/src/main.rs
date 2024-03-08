use std::io;

fn main() {
    println!("Indovina il numero!");

    println!("Prego, digita un tentativo.");

    let mut tentativo = String::new();

    io::stdin().read_line(&mut tentativo)
        .expect("Non si riesce a leggere la riga");

    println!("Hai digitato: {}", tentativo);
}