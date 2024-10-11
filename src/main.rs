use tracing::info;
use tracing_subscriber::EnvFilter;

use mtool::{cli, Result, update_person_director, update_person_job};
use mtool::modules::person::save_list;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .without_time()
        .with_target(true)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let matches = cli().get_matches();
    let filename = matches.get_one::<String>("file").expect("expect filename");
    let outfile = matches
        .get_one::<String>("output")
        .expect("expect filename");
    let source = matches.get_one::<String>("base").expect("expect source");
    let is_demo = source.eq("demo");
    if is_demo {
        info!("Тестовая база!");
    } else {
        info!("Прод база");
    }

    match matches.subcommand() {
        Some(("update", sub_matches)) => {
            let object = sub_matches.get_one::<String>("OBJECT").expect("OBJECT");
            match object.as_str() {
                "boss" => {
                    info!("Обновляем руководителя");
                    update_person_director(filename, is_demo).await?;
                    info!("Обновление закончено!");
                }
                "job" => {
                    info!("Обновляем должность");
                    update_person_job(filename, is_demo).await?;
                    info!("Обновление закончено!");
                }
                _ => unreachable!(),
            }
        }
        Some(("get", _)) => {
            info!("Скачать список сотрудников");
            save_list(outfile, is_demo).await?;
            info!("Скачивание закончено!");
        }
        _ => unreachable!(),
    }

    Ok(())
}
