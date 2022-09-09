use futures::StreamExt;
use k8s_openapi::api::core::v1::Pod;
use kube::{
    runtime::controller::{Action, Controller},
    Api, Client, ResourceExt,
};
use std::{sync::Arc, time::Duration};

#[derive(thiserror::Error, Debug)]
pub enum Error {}
pub type Result<T, E = Error> = std::result::Result<T, E>;

#[tokio::main]
async fn main() -> Result<(), kube::Error> {
    let client = Client::try_default().await?;
    let pods = Api::<Pod>::all(client);

    Controller::new(pods.clone(), Default::default())
        .run(reconcile, error_policy, Arc::new(()))
        .for_each(|_| futures::future::ready(()))
        .await;

    Ok(())
}

async fn reconcile(obj: Arc<Pod>, _: Arc<()>) -> Result<Action> {
    println!("reconcile request: {}", obj.name_any());
    Ok(Action::requeue(Duration::from_secs(3600)))
}

fn error_policy(_error: &Error, _ctx: Arc<()>) -> Action {
    Action::requeue(Duration::from_secs(5))
}

