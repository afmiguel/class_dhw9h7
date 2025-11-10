use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let data = Arc::new(Mutex::new(0));
    let data_clone = Arc::clone(&data);

    // Esta thread vai dar panic! e envenenar o mutex
    let handle = thread::spawn(move || {
        let guard = data_clone.lock().unwrap();
        // panic!("A thread 1 falhou!"); // O lock é envenenado
        *guard + 1 
    }); // Espera a thread falhar

    match handle.join() {
        Ok(result) => println!("Thread 1 result: {}", result),
        Err(_) => println!("A thread 1 falhou."),
    }

    // A thread 'main' tenta travar o mutex envenenado
    let res = data.lock(); // Não usamos .unwrap()

    // 'res' será um Err(PoisonError(...))
    println!("Mutex está envenenado? {}", res.is_err()); // true
}
