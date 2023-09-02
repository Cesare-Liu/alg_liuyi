
//考虑实现乘法的trait的泛型
//算法未考虑溢出，或者在模乘法群下进行计算，可能需要取模计算
pub fn powering(g: i32, n: i32) -> i32 {
	let mut y = 1;
	if n == 0 {
		return 1;
	}
	let mut m;
	if n > 0 {
		m = n;
	}
	else {
		m = -n;
	}
	let mut z = g;
	while m > 0 {
		if m % 2 == 1 {
			y = z * y;
		}
		m = m / 2;
		z = z * z;
	}
	return y;
 }

 #[test]
 pub fn test_powering() {
	let base: i32 = 2;
	println!("{:?}", powering(2,10));
	assert_eq!(powering(2,10), base.pow(10));
 }