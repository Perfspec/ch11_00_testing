use ch11_00_testing;

mod common;

#[test]
fn greeting_contains_name() {
	common::setup();
	
	let result = ch11_00_testing::greeting("Carol");
	assert!(
		result.contains("Carol"),
		"Greeting did not contain name, value was `{}`",
		result
	);
}