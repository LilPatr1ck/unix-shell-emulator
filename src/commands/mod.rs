pub mod ls;
pub mod cd;
pub mod exit;

/// Распределяет выполнение команд по отдельным файлам-модулям
pub fn handle(command: &str, args: &[String]) {
    match command {
        "ls" => ls::run(args),
        "cd" => cd::run(args),
        "exit" => exit::run(),
        _ => eprintln!("Ошибка: команда '{}' не найдена", command),
    }
}