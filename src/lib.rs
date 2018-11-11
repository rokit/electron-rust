#[no_mangle]
pub extern fn fibonacci(x: i32) -> i32 {
		if x <= 2 {
				return 1;
		}
		return fibonacci(x - 1) + fibonacci(x - 2);
}

#[no_mangle]
pub extern fn two_plus_two() -> i32 {
		2+2
}

#[no_mangle]
pub extern fn different_types(x: i32, y: i32) -> f32 {
		x as f32 + 1.023 + y as f32
}