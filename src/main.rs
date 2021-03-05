use matrix::Matrix;

fn main() {
	let a = Matrix {arr:vec![vec![2.0,2.0],vec![2.0,2.0]]};
	let b = Matrix {arr:vec![vec![2.0], vec![2.0]]};
	let c = a.mul(&b);
	
	print!("{:?}",c);
}