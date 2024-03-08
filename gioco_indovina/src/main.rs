extern crate rand;

use std::io;
use rand::Rng;

fn main() {
    println!("Indovina il numero!");

    let numero_segreto = rand::thread_rng().gen_range(1, 101);

    println!("Il numero segreto è: {}", numero_segreto);

    println!("Prego, digita un tentativo.");

    let mut tentativo = String::new();

    io::stdin().read_line(&mut tentativo)
        .expect("Non si riesce a leggere la riga");

    println!("Hai digitato: {}", tentativo);
}