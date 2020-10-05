use k8s_openapi::api::core::v1::Pod;
use kube::{
    api::{Api, ListParams, Meta},
    Client,
};
use std::collections::HashSet;

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

    fn contains(&self, target: &String) -> bool {
        match &self.node_name {
            Some(node) => node == target,
            None => false,
        }
    }
}

#[derive(Debug)]
struct NodeInfo<'a> {
    name: String,
    pods: Vec<&'a PodInfo>,
}

impl NodeInfo<'_> {
    fn new(name: String, pod_info: &'static Vec<PodInfo>) -> NodeInfo {
        NodeInfo {
            name: name.clone(),
            pods: pod_info
                .iter()
                .filter(|p| p.contains(&name))
                .collect::<Vec<_>>(),
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
        .collect::<Vec<_>>();

    let node_names = pod_list
        .iter()
        .filter(|p| p.node_name.as_ref().is_some())
        .map(|p| p.node_name.clone().unwrap())
        .collect::<HashSet<_>>();
    println!("{:#?}", &node_names);

    let nodes = node_names
        .iter()
        .map(|n| NodeInfo::new(n.to_string(), pod_list.clone()))
        .collect::<Vec<_>>();

    println!("{:#?}", &nodes);

    Ok(())
}
