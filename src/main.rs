use std::io;
extern  crate num_complex;
use num_complex::Complex;

//acos(x)= pi/2 - arcsin(x)
fn arccosTeylor(x:f64, eps:f64) -> (f64, i32) {
	let mut result:f64 =0.0 ;
	let mut term:f64 = x;
	let mut squaredX:f64 = x*x;
	let mut n:i32 = 1;
	while term.abs()>eps{
		result+=term;
		n+=2;
		term *= squaredX *(n-2)as f64 / (n as f64 * (n-1) as f64);
	}
	(std::f64::consts::PI/2.0 - result ,(n-1)/2)
}

fn expTeylor(x:f64, eps:f64)->(f64,i32) {
	let mut result:f64 = 1.0;
	let mut term: f64 = 1.0;
	let mut n:i32 = 1;
	while term.abs() >eps{
		term*=x/n as f64;
		result+=term;
		n+=1;
	}
	(result, n-1)
}

fn cosTeylor(x:f64, eps:f64)->(f64, i32){
	let mut result:f64 = 1.0;
	let mut term:f64 = 1.0;
	let mut n:i32 = 1;
	while term.abs()>eps{
		term *= -x*x/ ((2*n)*(2*n-1)) as f64;
		result+=term;
		n+=1;
	}
	(result, n-1)
}

fn main(){
	println!("Программа для рассчета выражения cos(y)(e^y)*arccos(x)\\n в случае x>1 будет выполнен рассчет только первой части");
	let mut inp= String::new();
	println!("Введите x для функции acos(x):");
	io::stdin().read_line(&mut inp).expect("Не удалось считать строку");
	let x:f64 = inp.trim().parse().expect("НЕ удалось выполнить преобразование во float64");
	println!("Введите y для функции e^y и cos(y):");
	let mut inp2= String::new();
	io::stdin().read_line(&mut inp2).expect("Не удалось считать строку");
	let xForExp:f64 = inp2.trim().parse().expect("НЕ удалось выполнить преобразование во float64");
	let eps = 1e-6;
	if x.abs() > 1.0 {
		let (result, iterations) = expTeylor(xForExp,eps);
		let xForcos = xForExp%(2 as f64* std::f64::consts::PI);
		let (result2, iterations2) = cosTeylor(xForcos, eps);
		println!("e^{} = {} with {} iterations", xForExp, result, iterations);
		println!("cos({}) = {} with {} iterations", xForExp, result2, iterations2);
		let finallyRes= result*result2;
		println!("cos({})e^{} = {}", xForExp,xForExp, finallyRes);
	}else {
		let (result, iterations) = arccosTeylor(x, eps);
		println!("arccos({}) = {} with {} iterations", x, result, iterations);
	}
}