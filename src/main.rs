use std::sync::{Arc, RwLock};

fn main() {
    let config = Arc::new(RwLock::new("Versão 1".to_string()));

    // --- 5 Threads Leitoras (rodam em paralelo!) ---
    for i in 0..5 {
        let config_clone = Arc::clone(&config);
        std::thread::spawn(move || {
            // Pede um lock de LEITURA (compartilhado)
            let cfg = config_clone.read().unwrap();
            println!("Leitor {}: Config é '{}'", i, *cfg);
        });
    }

    // --- 1 Thread Escritora (espera todos os leitores) ---
    let config_clone_writer = Arc::clone(&config);
    std::thread::spawn(move || {
        // Pede um lock de ESCRITA (exclusivo)
        let mut cfg = config_clone_writer.write().unwrap();
        *cfg = "Versão 2".to_string();
        println!("ESCRITOR: Config atualizada!");
    });

    std::thread::sleep(std::time::Duration::from_secs(1));
}

