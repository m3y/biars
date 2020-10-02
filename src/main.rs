use k8s_openapi::api::core::v1::Pod;
use kube::{
    api::{Api, ListParams, Meta},
    Client,
};

#[derive(Debug)]
struct PodInfo {
    name: String,
    namespace: String,
    node_name: Option<String>,
}

impl PodInfo {
    fn new(pod: &Pod) -> PodInfo {
        PodInfo {
            name: Meta::name(pod),
            namespace: Meta::namespace(pod).unwrap_or("default".to_string()),
            node_name: pod.spec.as_ref().map(|ps| ps.node_name.clone()).flatten(),
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = Client::try_default().await?;

    let pods: Api<Pod> = Api::all(client);
    let lp = ListParams::default();

    let pod_list = pods
        .list(&lp)
        .await?
        .items
        .iter()
        .map(|p| PodInfo::new(&p))
        .collect::<Vec<PodInfo>>();
    println!("{:#?}", &pod_list);

    Ok(())
}
