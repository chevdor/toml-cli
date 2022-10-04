#[cfg(test)]
mod cli_tests {

	#[cfg(test)]
	mod help {
		use assert_cmd::Command;

		#[test]
		fn it_shows_help() {
			let mut cmd = Command::cargo_bin(env!("CARGO_BIN_EXE_toml")).unwrap();
			let assert = cmd.arg("--help").assert();
			assert.success().code(0);
		}
	}

	#[cfg(test)]
	mod get {
		use assert_cmd::Command;

		#[test]
		fn it_gets_simple() {
			let mut cmd = Command::cargo_bin(env!("CARGO_BIN_EXE_toml")).unwrap();
			let assert = cmd.args(&["get", "Cargo.toml", "version"]).assert();
			assert.success().code(0);
		}

		#[test]
		#[ignore = "Requires a update of the toml_edit dependency at least"]
		fn it_gets_dotted() {
			let mut cmd = Command::cargo_bin(env!("CARGO_BIN_EXE_toml")).unwrap();
			let assert = cmd.args(&["get", "test/demo.toml", "version"]).assert();
			assert.success().code(0);
		}
	}
}
