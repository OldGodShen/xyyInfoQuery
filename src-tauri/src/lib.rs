#![allow(non_snake_case)]
use std::sync::Arc;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tauri::async_runtime::Mutex;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![cardNoQuery])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn cardNoQuery(card_no: &str) -> Result<String, String> {
    // 将字符串解析为 u32 类型
    let current_card_no: u32 = match card_no.parse() {
        Ok(num) => num,
        Err(_) => {
            return Ok(format!("你是说你的卡号不止10位¿")); // 返回一个错误消息
        }
    };

    let client = Client::builder()
        .build()
        .expect("Failed to build client");
    let client = Arc::new(Mutex::new(client));
    let mut collected_data = String::new(); // 初始化一个字符串，用于收集处理后的数据
    
    match request_data(&client, current_card_no).await {
        Ok(api_response) => {
            if let Some(user_info) = api_response.result {
                if api_response.return_code == 203 || api_response.return_code == 200 {
                    // Handle two different response data structures
                    if let Some(org_info) = user_info.org {
                        if let Some(user_details) = user_info.user {
                            // 调用 process_user_info 并捕获可能的错误
                            if let Ok(data) = process_user_info(&org_info, &user_details) {
                                collected_data = data; // 如果成功，将结果赋值给 collected_data
                            } else {
                                return Ok(format!("卡号{}并不存在",current_card_no)); // 返回一个错误消息
                            }
                        }
                    } else if let Some(reserve_order_resps) = api_response.reserveOrderResps {
                        for resp in reserve_order_resps {
                            if let Some(org_info) = resp.org {
                                if let Some(user_details) = resp.user {
                                    // 调用 process_user_info 并捕获可能的错误
                                    if let Ok(data) = process_user_info(&org_info, &user_details) {
                                        collected_data = data; // 如果成功，将结果赋值给 collected_data
                                    } else {
                                        return Ok(format!("卡号{}并不存在",current_card_no)); // 返回一个错误消息
                                    }
                                }
                            }
                        }
                    }
                }
            }
            if collected_data.is_empty() {
                return Ok(format!("卡号{}并不存在",current_card_no)); // 返回一个错误消息，当没有收集到数据时
            }
        }
        Err(e) => return Ok(format!("服务器响应失败, {:?}",e)), // 返回一个错误消息
    }
    Ok(collected_data) // 返回收集到的处理后的数据
}

fn process_user_info(
    org_info: &OrgInfo,
    user_details: &UserDetails,
) -> Result<String, String> {
    if let (Some(full_name), Some(user_name), Some(card_no)) = (
        org_info.fullName.clone(),
        user_details.userName.clone(),
        user_details.cardNo.clone(),
    ) {
        let collected_data = CollectedData {
            full_name,
            user_name,
            card_no,
        };
        // Serialize data to JSON
        serde_json::to_string(&collected_data).map_err(|e| e.to_string()) // 返回处理后的数据
    } else {
        Err("Missing information".to_string()) // 如果有信息缺失，返回错误
    }
}

async fn request_data(
    client: &Mutex<Client>,
    card_no: u32,
) -> Result<ApiResponse, Box<dyn std::error::Error>> {
    let url = "https://xyy.shbizhen.com/aiapi/device/searchReserveOrder";
    let request_body = serde_json::to_string(&json!({ "cardNo": card_no }))?;

    let client = client.lock().await;
    let res = client
        .post(url)
        .header("content-type", "application/json")
        .body(request_body)
        .send()
        .await?;

    let api_response: ApiResponse = res.json().await?;
    Ok(api_response)
}

#[derive(Debug, Serialize)]
struct CollectedData {
    full_name: String,
    user_name: String,
    card_no: String,
}

#[derive(Debug, Deserialize)]
struct ApiResponse {
    return_code: u32,
    result: Option<UserInfo>,
    reserveOrderResps: Option<Vec<ReserveOrderResp>>,
}

#[derive(Debug, Deserialize)]
struct ReserveOrderResp {
    org: Option<OrgInfo>,
    user: Option<UserDetails>,
}

#[derive(Debug, Deserialize)]
struct UserInfo {
    org: Option<OrgInfo>,
    user: Option<UserDetails>,
}

#[derive(Debug, Deserialize)]
struct OrgInfo {
    fullName: Option<String>,
}

#[derive(Debug, Deserialize)]
struct UserDetails {
    userName: Option<String>,
    cardNo: Option<String>,
}
