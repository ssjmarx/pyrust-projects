#[pyo3::pymodule]
mod rust {

use pyo3::prelude::*;

  #[pyfunction]
  fn say_hello(word: &str) -> String {
    format!("Hello from rust!  The secret word is {}.", word)
  }

  #[pyfunction]
  fn multilingual_greeting(language: &str) -> String {
      match language {
        "English" => String::from("Hello world!"),
        "Spanish" => String::from("¡Hola, mundo!"),
        "French" => String::from("Bonjour, le monde!"),
        "German" => String::from("Hallo, Welt!"),
        _ => String::from("Eraro, bonvolu provi denove")
      }
  }
}