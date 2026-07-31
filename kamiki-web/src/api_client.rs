use crate::data::models::{FlowData, InterfaceInfo, PacketEvent, SystemStats};
use gloo_net::http::Request;

pub async fn get_interfaces() -> Result<Vec<InterfaceInfo>, String> {
    Request::get("/api/interfaces")
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json()
        .await
        .map_err(|e| e.to_string())
}

pub async fn start_capture(interface: String) -> Result<String, String> {
    Request::post("/api/start")
        .body(interface)
        .map_err(|e| e.to_string())?
        .send()
        .await
        .map_err(|e| e.to_string())?
        .text()
        .await
        .map_err(|e| e.to_string())
}

pub async fn stop_capture() -> Result<(), String> {
    Request::post("/api/stop")
        .send()
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

pub async fn poll_packets(limit: usize) -> Result<Vec<PacketEvent>, String> {
    Request::get(&format!("/api/poll?limit={}", limit))
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json()
        .await
        .map_err(|e| e.to_string())
}

pub async fn get_flows() -> Result<Vec<FlowData>, String> {
    Request::get("/api/flows")
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json()
        .await
        .map_err(|e| e.to_string())
}

pub async fn get_stats() -> Result<SystemStats, String> {
    Request::get("/api/stats")
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json()
        .await
        .map_err(|e| e.to_string())
}
