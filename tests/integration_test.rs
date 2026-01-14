use mini_grep::{Config, grep};
use std::env;
use std::io::Write;
use std::sync::Mutex;
use tempfile::NamedTempFile;

// Mutex para garantir a segurança ao manipular variáveis de ambiente em testes paralelos.
static ENV_LOCK: Mutex<()> = Mutex::new(());

#[test]
fn integration_test_case_sensitive() {
    // Arrange
    let mut file = NamedTempFile::new().unwrap();
    let contents = "linha um\nlinha dois com query\nlinha tres\nQUERY: outra linha";
    file.write_all(contents.as_bytes()).unwrap();
    file.flush().unwrap(); 

    let file_path = file.path().to_str().unwrap().to_string();
    let config = Config::build(&file_path, "query").unwrap();

    // Cria um vetor de bytes na memória para atuar como nosso "writer".
    let mut writer = Vec::new();

    // Act
    grep(&config, &mut writer);

    let output = String::from_utf8(writer).unwrap();

    // Assert
    let expected_output = "[LINHA 2]: linha dois com query\n";
    assert_eq!(output, expected_output);
}

#[test]
fn integration_test_case_insensitive() {
    let _lock = ENV_LOCK.lock().unwrap();
    // Arrange 
    let mut file = NamedTempFile::new().unwrap();
    let contents = "linha um\nlinha dois com Query\nlinha tres\nUPPERCASE: outra linha";
    file.write_all(contents.as_bytes()).unwrap();
    file.flush().unwrap();

    let file_path = file.path().to_str().unwrap().to_string();

    unsafe {
        env::set_var("IGNORE_CASE", "1");
    }
    let config = Config::build(&file_path, "query").unwrap();
    unsafe {
        env::remove_var("IGNORE_CASE");
    }

    let mut writer = Vec::new();

    // Act
    grep(&config, &mut writer);
    let output = String::from_utf8(writer).unwrap();

    // Assert
    let expected_output = "[LINHA 2]: linha dois com Query\n";
    assert_eq!(output, expected_output);
}

#[test]
fn integration_test_no_results() {
    // Arrange
    let mut file = NamedTempFile::new().unwrap();
    let contents = "linha um\nlinha dois\nlinha tres";
    file.write_all(contents.as_bytes()).unwrap();
    file.flush().unwrap();

    let file_path = file.path().to_str().unwrap().to_string();
    let config = Config::build(&file_path, "nonexistent_query").unwrap();

    let mut writer = Vec::new();

    // Act
    grep(&config, &mut writer);
    let output = String::from_utf8(writer).unwrap();

    // Assert 
    let expected_output = "";
    assert_eq!(output, expected_output);
}
