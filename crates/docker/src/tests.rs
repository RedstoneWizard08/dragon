use futures_util::{pin_mut, StreamExt};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::collections::HashMap;

use crate::{
    container::{create::ContainerCreateOptions, ContainerHelper},
    image::{pull::PullEvent, ImageHelper},
};

async fn pull_test_inner() {
    let m = MultiProgress::new();
    let st = ProgressStyle::with_template("[{msg}] {bar:40.cyan/blue} {pos:>7}/{len:7}")
        .unwrap()
        .progress_chars("##-");
    let mut pbs: HashMap<String, ProgressBar> = HashMap::new();

    let stream = ImageHelper::new().unwrap().pull("ubuntu:20.04");

    pin_mut!(stream);

    while let Some(msg) = stream.next().await {
        match msg {
            PullEvent::Error(err) => {
                assert!(false, "Error: {}", err);
                break;
            }

            PullEvent::Status {
                id,
                status,
                current,
                total,
                progress: _,
            } => {
                if !pbs.contains_key(&id) {
                    let pb = m.add(ProgressBar::new(total));

                    pb.set_style(st.clone());
                    pb.set_message(id.clone());

                    pbs.insert(id.clone(), pb);
                }

                pbs.get(&id)
                    .unwrap()
                    .set_message(format!("{} ({})", id, status));

                pbs.get(&id).unwrap().set_position(current);
            }
        }
    }
}

#[tokio::test]
pub async fn can_pull_image() {
    pull_test_inner().await;
}

#[tokio::test]
pub async fn can_create_and_run_container() {
    pull_test_inner().await;

    ContainerHelper::new()
        .unwrap()
        .run(ContainerCreateOptions {
            image: String::from("ubuntu:20.04"),
            command: Some(vec![
                String::from("echo"),
                String::from("Hello from inside Docker!"),
            ]),
            rm: true,
            attach: true,
            ..Default::default()
        })
        .await
        .unwrap();
}
