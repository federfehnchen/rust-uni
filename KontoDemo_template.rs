use std::sync::{Arc, Mutex};
use std::{thread};

struct Konto {
	kontostand: i32
}

fn einzahlen(amount:i32, konto:Arc<Mutex<Konto>>) {
	konto.lock().unwrap().kontostand += amount;
}

fn abheben(amount:i32, konto:Arc<Mutex<Konto>>) {
	konto.lock().unwrap().kontostand -= amount;
}

fn ueberweisungen(konto:Arc<Mutex<Konto>>) {
	// Es wird 100 mal 1 Euro eingezahlt
	for _ in 0..100 {
		konto.lock().unwrap().kontostand += 1;
	}

	// Es wird 100 mal 1 Euro wieder abgehoben
	for _ in 0..100 {
		konto.lock().unwrap().kontostand -= 1;
	}
}

fn main() {
	// Wir erstellen ein neues Konto ohne Guthaben
	let mein_konto = Konto{ kontostand: 0 };
	let konto_wrap = Arc::new(Mutex::new(mein_konto));
	println!("Kontostand vorher: {}", konto_wrap.lock().unwrap().kontostand);
	
	// Wir simulieren 50 mal jeweils 200 Ueberweisungsvorgänge
	let mut handlers = Vec::new();
	let n = 50;
	for _ in 0..n {
		let cloned = konto_wrap.clone();
		handlers.push(thread::spawn(move || {ueberweisungen(cloned)}));
	}

	for handle in handlers {
		handle.join().unwrap();
	}

	println!("Kontostand nachher: {}", konto_wrap.lock().unwrap().kontostand);
}