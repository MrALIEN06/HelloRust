extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Indovina il numero!");

    let numero_segreto = rand::thread_rng().gen_range(1, 101);

    println!("Il numero segreto è: {}", numero_segreto);

    loop {
        println!("Prego, digita un tentativo.");

        let mut tentativo = String::new();

        io::stdin().read_line(&mut tentativo)
            .expect("Non si riesce a leggere la riga");

        let tentativo: u32 = match tentativo.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Hai digitato: {}", tentativo);

        match tentativo.cmp(&numero_segreto) {
            Ordering::Less    => println!("Troppo piccolo!"),
            Ordering::Greater => println!("Troppo grande!"),
            Ordering::Equal   => {
                println!("Hai vinto!");
                break;
            }
        }
    }
}