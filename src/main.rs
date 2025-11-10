use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // 1. O contador é protegido por Arc e Mutex.
    let counter = // CRIAR O ARC MUTEX para um i32!!!!
    let mut handles = vec![];

    for _ in 0..10 {
        // 2. Clonamos o Arc (operação barata)
        let counter_clone = Arc::clone(&counter);

        let handle = thread::spawn(move || {
            // INCREMENTAR O counter AQUI!!!!
        });
        handles.push(handle);
    }
    // 6. Espera todas as threads terminarem
    for handle in handles {
        handle.join().unwrap();
    }

    // 7. Trava o mutex na thread 'main' para ler
    //    o valor final.
    println!("Resultado: {}", *counter.lock().unwrap());
}
