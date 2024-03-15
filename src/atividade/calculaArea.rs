pub fn calculaArea(raio: f64){
	let pi: f64 = 3.14;
	let area: f64 = pi * raio.powf(2.0);
	println!("A área do círuclo de base {} é {:.2}", raio, area);
}