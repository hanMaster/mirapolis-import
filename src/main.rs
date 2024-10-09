use mtool::modules::person::save_list;
use mtool::{cli, update_person_director, update_person_job, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let matches = cli().get_matches();
    let filename = matches.get_one::<String>("file").unwrap();
    let source = matches.get_one::<String>("base").unwrap();
    let is_demo = source.eq("demo");
    if is_demo {
        println!("Тестовая база!");
    } else {
        println!("Прод база");
    }

    match matches.subcommand() {
        Some(("update", sub_matches)) => {
            let object = sub_matches.get_one::<String>("OBJECT").expect("OBJECT");
            match object.as_str() {
                "boss" => {
                    println!("Обновляем руководителя");
                    update_person_director(filename, is_demo).await?;
                    println!("Обновление закончено!");
                }
                "job" => {
                    println!("Обновляем должность");
                    update_person_job(filename, is_demo).await?;
                    println!("Обновление закончено!");
                }
                &_ => {}
            }
        }
        Some(("get", _)) => {
            println!("Скачать список сотрудников");
            save_list(is_demo).await?;
            println!("Скачивание закончено!");
        }
        _ => {}
    }

    Ok(())
}
