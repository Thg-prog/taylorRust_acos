use std::io;
extern  crate num_complex;
use num_complex::Complex;

static POW_VEC: &'static [f64] = &[
0.0,
10.0,
100.0,
1000.0,
10000.0,
100000.0,
1000000.0,
10000000.0,
100000000.0,
];

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

fn expTeylor(x: f64, eps:f64) -> (f64, i32) {
    const E: f64 = std::f64::consts::E;

    let x2dop:f64 = x.floor();
    let mut c = 0;
	let mut res2dop:f64 = 1.0;
    for _ in 0..x2dop as i32 {
        res2dop *= E;
        c += 1;
    }
    let x2 = x - x2dop;
    let mut f:f64 = 1.0;
    let mut res2 = 1.0;
    let mut n = 0;

    while f.abs() > eps {
        f *= x2 / (n + 1) as f64;
        n += 1;
        res2 += f;
    }
    res2 *=res2dop;
    (res2, n-1)
}


fn cosTeylor(x:f64, eps:f64)->(f64, i32){
	let mut result:f64 = 1.0;
	let mut term:f64 = 1.0;
	let mut n:i32 = 1;
	while term.abs()>eps{
		term *= -x*x/ ((2*n)*(2*n-1)) as f64;
		term = round(term, 2);
		result+=term;
		n+=1;
		result = round(result, 6);
	}
	(result, n-1)
}

fn round(num: f64, precision: u8) -> f64 {
	let multiplier = POW_VEC[precision as usize];
	let tmp_value = (num * multiplier).round().abs() as u64;
	((tmp_value as f64) / multiplier) * num.signum()
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
		println!("cos({}) = {} with {} iterations {}", xForExp, result2, iterations2,round(xForcos, 1));
		let finallyRes= result*result2;
		println!("cos({})e^{} = {}", xForExp,xForExp, finallyRes);
	}else {
		let (result, iterations) = arccosTeylor(x, eps);
		println!("arccos({}) = {} with {} iterations", x, result, iterations);
		let (result1, iterations1) = expTeylor(xForExp,eps);
		println!("e^{} = {} with {} iterations", xForExp, result1, iterations1);
		let xForcos = xForExp%(2 as f64* std::f64::consts::PI);
		let (result2, iterations2) = cosTeylor(xForcos, eps);
		println!("cos({}) = {} with {} iterations", xForExp, result2, iterations2);
		let finallyRes= result*result1*result2;
		println!("acos({})cos({})e^{} = {}", xForcos,xForExp,xForExp, finallyRes);
	}
}