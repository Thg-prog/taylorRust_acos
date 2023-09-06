use std::io;

//acos(x)
fn arccosTeylor(x:f64, eps:f64) -> (f64, i32) {
	let mut result = 0.0;
	let mut term = x;
	let mut squaredX = x*x;
	let mut denominator = 1.0;
	let mut n = 1;
	while term.abs()>eps{
		result+=term/denominator;
		term *=- squaredX / ((2*n-1)as f64 * (2*n) as f64);
		n+=1;
		denominator+=2.0;
	}
	(std::f64::consts::PI/2.0 - result ,n-1)
}

// acos(x) = pi/2 - (x + x^3/3 + ..)
fn main(){
	let mut inp= String::new();
	println!("Введите x для функции acos(x):");
	io::stdin().read_line(&mut inp).expect("Не удалось считать строку");
	let x:f64 = inp.trim().parse().expect("НЕ удалось выполнить преобразование во float64");
	let eps = 1e-6;
	if x.abs() > 1.0 {
		println!("You can't enter x more then 1 and smaller then -1");
	}else {
		let (result, iterations) = arccosTeylor(x, eps);
		println!("arccos({}) = {} with {} iterations", x, result, iterations);
	}
}