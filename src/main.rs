use std::io;
fn main() {
				println!("Informe o raio do círculo: ");
				let mut s = String::new();
				let _ = io::stdin().read_line(&mut s).expect("Ok");
				let raio: f64 = s.trim().parse().unwrap();
				calculaArea(raio)
}

fn calculaArea(raio: f64){
	let pi: f64 = 3.14;
	let area: f64 = pi * raio.powf(2.0);
	println!("A área do círuclo de raio {} é {:.2}cm", raio, area);
}