use std::{sync::mpsc::channel, collections::HashMap};
use indicatif::{MultiProgress, ProgressStyle, ProgressBar};
use crate::pull::{pull_image, PullEvent};

#[tokio::test]
pub async fn can_pull_image() {
    let (tx, rx) = channel::<PullEvent>();

    std::thread::spawn(move || {
        let m = MultiProgress::new();
        let st = ProgressStyle::with_template("[{msg}] {bar:40.cyan/blue} {pos:>7}/{len:7}").unwrap().progress_chars("##-");
        let mut rxi = rx.iter();
        let mut pbs: HashMap<String, ProgressBar> = HashMap::new();

        while let Some(msg) = rxi.next() {
            match msg {
                PullEvent::Started => println!("Starting pull..."),

                PullEvent::Completed => {
                    println!("Pull completed!");
                    break;
                }

                PullEvent::Error(err) => {
                    assert!(false, "Error: {}", err);
                    break;
                }

                PullEvent::Status(id, status, cur, total, _) => {
                    if !pbs.contains_key(&id) {
                        let pb = m.add(ProgressBar::new(total));

                        pb.set_style(st.clone());
                        pb.set_message(id.clone());

                        pbs.insert(id.clone(), pb);
                    }

                    pbs.get(&id).unwrap().set_message(format!("{} ({})", id, status));
                    pbs.get(&id).unwrap().set_position(cur);
                }
            }
        }
    });

    pull_image("ubuntu:20.04", tx).await.unwrap();
}
