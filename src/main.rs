use std::io::{self, Write};
use std::process::{self};

mod commands;

/// Имя виртуальной файловой системы
const VFS_NAME: &str = "vfs";

fn main() {
    println!("Добро пожаловать в unix-shell-emulator");
    println!("Для выхода используется команда 'exit'\n");

    if let Err(e) = run_repl() {
        eprintln!("Критическая ошибка приложения: {}", e);
        process::exit(1);
    }
}

/// Основной цикл REPL (чтение строки ввода)
fn run_repl() -> io::Result<()> {
    loop {
        print!("{}> ", VFS_NAME);
        io::stdout().flush()?;

        let mut input = String::new();
        if io::stdin().read_line(&mut input)? == 0 {
            println!("\nВыход из программы.");
            break;
        }

        let input = input.trim();
        if !input.is_empty() {
            execute_line(input);
        }
    }
    Ok(())
}

/// Парсинг строки и маршрутизация команд
fn execute_line(input: &str) {
    let tokens = match parse_arguments(input) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Ошибка синтаксиса: {}", e);
            return;
        }
    };

    if let Some((command, args)) = tokens.split_first() {
        commands::handle(command, args);
    }
}

/// Разбивает строку на аргументы, учитывая двойные кавычки
fn parse_arguments(input: &str) -> Result<Vec<String>, &'static str> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '"' => {
                in_quotes = !in_quotes;
            }
            ' ' if !in_quotes => {
                if !current.is_empty() {
                    tokens.push(current);
                    current = String::new();
                }
            }
            _ => {
                current.push(c);
            }
        }
    }

    if in_quotes {
        return Err("Незакрытая кавычка");
    }

    if !current.is_empty() {
        tokens.push(current);
    }

    Ok(tokens)
}
