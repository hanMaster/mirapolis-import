use clap::{arg, Command};

pub fn cli() -> Command {
    Command::new("mtool")
        .about("Утилита для работы с базой mirapolis")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("update")
                .about("Изменение параметра в базе")
                .arg(arg!(<OBJECT> "Объект для изменения: boss, job"))
                .arg_required_else_help(true),
        )
        .subcommand(Command::new("get").about("Скачать список сотрудников из базы"))
        .arg(
            arg!(-f --file <filename>)
                .num_args(0..=1)
                .default_value("full.csv")
                .default_missing_value("full.csv"),
        )
        .arg(
            arg!(-b --base <source>)
                .num_args(0..=1)
                .value_parser(["demo", "prod"])
                .default_value("demo")
                .default_missing_value("demo"),
        )
}
