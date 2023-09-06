use std::io;
extern  crate num_complex;
use num_complex::Complex;

//acos(x)= pi/2 - arcsin(x)
fn arccosTeylor(x:f64, eps:f64) -> (f64, i32) {
	let mut result =0.0 ;
	let mut term = x;
	let mut squaredX = x*x;
	let mut n = 1;
	while term.abs()>eps{
		result+=term;
		n+=2;
		term *= - squaredX *(n-2)as f64 / (n as f64 * (n-1) as f64);
	}
	(std::f64::consts::PI/2.0 - result ,(n-1)/2)
}

fn arccosTeylorComplex(x: Complex<f64>, epsilon: f64) -> (Complex<f64>, usize) {
	let mut result = Complex::new(0.0, 0.0);
	let mut term = x;
	let mut squaredX = x * x;
	let mut denominator = Complex::new(1.0, 0.0);
	let mut n = 1;
	while term.norm() > epsilon {
		term = term * (-squaredX) / (Complex::new((2 * n - 1) as f64, 0.0) * Complex::new((2 * n) as f64, 0.0));
		result = result + term / denominator;
		n += 1;
		denominator = denominator + Complex::new(2.0, 0.0);
	}

	(Complex::new(std::f64::consts::PI / 2.0, 0.0) - result, n - 1)
}

fn main(){

	let mut inp= String::new();
	println!("Введите x для функции acos(x):");
	io::stdin().read_line(&mut inp).expect("Не удалось считать строку");
	let x:f64 = inp.trim().parse().expect("НЕ удалось выполнить преобразование во float64");
	let eps = 1e-6;
	if x.abs() > 1.0 {
		let (result, iterations) = arccosTeylorComplex(Complex::new(x,0.0),eps);
		println!("arccos({}) = {} with {} iterations", x, result, iterations);
	}else {
		let (result, iterations) = arccosTeylor(x, eps);
		println!("arccos({}) = {} with {} iterations", x, result, iterations);
	}
}