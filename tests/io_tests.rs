use std::fs;
use tempfile::TempDir;

use jing::compiler::Compiler;
use jing::lexer::Lexer;
use jing::parser::Parser;
use jing::vm::VM;

/// Test file I/O builtin functions
#[test]
fn test_file_io_functions() {
    // Initialize builtins once
    jing::init();

    // Create a temporary directory for testing
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let test_file = temp_dir.path().join("test.txt");
    // Convert Windows backslashes to forward slashes for string literals
    let test_file_str = test_file.to_str().expect("Invalid path").replace('\\', "/");

    // Test 1: write_file and verify it creates the file
    let content = "Hello, World!\nThis is a test file.";
    let code1 = format!(r#"write_file("{}", "{}");"#, test_file_str, content);
    run_jing_code(&code1).expect("Failed to write file");

    // Verify file was created and has correct content
    assert!(test_file.exists(), "File should exist after write_file");
    let actual_content = fs::read_to_string(&test_file).expect("Failed to read file");
    assert_eq!(actual_content, content);

    // Test 2: file_exists should return true for existing file
    let code2 = format!(
        r#"let exists = file_exists("{}"); if (exists) {{ write_file("{}_exists", "true"); }}"#,
        test_file_str, test_file_str
    );
    run_jing_code(&code2).expect("Failed to check file existence");

    let exists_file = temp_dir.path().join("test.txt_exists");
    assert!(
        exists_file.exists(),
        "file_exists should detect existing file"
    );

    // Test 3: read_file should return correct content
    let read_test_file = temp_dir.path().join("read_test.txt");
    let read_test_str = read_test_file
        .to_str()
        .expect("Invalid path")
        .replace('\\', "/");
    let code3 = format!(
        r#"
        let content = read_file("{}");
        write_file("{}", content);
    "#,
        test_file_str, read_test_str
    );
    run_jing_code(&code3).expect("Failed to read and write file");

    let read_content = fs::read_to_string(&read_test_file).expect("Failed to read result file");
    assert_eq!(read_content, content);

    // Test 4: file_exists should return false for non-existent file
    let non_existent = temp_dir.path().join("nonexistent.txt");
    let non_existent_str = non_existent
        .to_str()
        .expect("Invalid path")
        .replace('\\', "/");
    let result_file = temp_dir.path().join("exists_result.txt");
    let result_file_str = result_file
        .to_str()
        .expect("Invalid path")
        .replace('\\', "/");

    let code4 = format!(
        r#"
        let exists = file_exists("{}");
        if (exists) {{
            write_file("{}", "true");
        }} else {{
            write_file("{}", "false");
        }}
    "#,
        non_existent_str, result_file_str, result_file_str
    );
    run_jing_code(&code4).expect("Failed to test non-existent file");

    let exists_result = fs::read_to_string(&result_file).expect("Failed to read result");
    assert_eq!(exists_result, "false");
}

/// Helper function to run Jing code
fn run_jing_code(code: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut lexer = Lexer::new(code);
    let tokens = lexer.tokenize()?;

    let mut parser = Parser::new(tokens);
    let statements = parser.parse()?;

    let mut compiler = Compiler::new();
    let chunk = compiler.compile(statements)?;

    let mut vm = VM::new();
    vm.interpret(chunk)?;

    Ok(())
}

/// Test error handling for invalid file operations
#[test]
fn test_file_io_error_handling() {
    // Initialize builtins once
    jing::init();

    // Test reading from non-existent file (should error)
    let code = r#"read_file("/invalid/path/that/should/not/exist.txt");"#;
    let result = run_jing_code(code);
    assert!(
        result.is_err(),
        "Should error when reading non-existent file"
    );
}
