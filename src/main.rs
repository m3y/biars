use k8s_openapi::api::core::v1::Pod;
use kube::{
    api::{Api, ListParams, Meta},
    Client,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = Client::try_default().await?;
    let namespace = std::env::var("NAMESPACE").unwrap_or("default".into());

    let pods: Api<Pod> = Api::namespaced(client, &namespace);
    let lp = ListParams::default();
    for p in pods.list(&lp).await? {
        println!("{}", Meta::name(&p));
        if let Some(ns) = Meta::namespace(&p) {
            println!("{}", ns);
        }
        if let Some(spec) = &p.spec {
            if let Some(node_name) = &spec.node_name {
                println!("{}", node_name);
            }
        }
    }

    Ok(())
}
