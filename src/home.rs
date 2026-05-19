use std::{collections::{HashMap, HashSet}, rc::Rc};

use slint::{ComponentHandle, Model, VecModel};
use tokio::sync::{mpsc, oneshot};

use crate::{
    db::DbRequest,
    entities::app_slot,
    ui::{AppData, AppStates, AppTypes, Main},
};

pub async fn setup_home(tx: mpsc::Sender<DbRequest>, ui: &Main) {
let (tx_oneshot, rx_oneshot) = oneshot::channel::<Vec<app_slot::Model>>();
    tx.send(DbRequest::GetApps(tx_oneshot)).await.unwrap();

    let app_db = rx_oneshot.await.unwrap();
    let ui_apps = ui.global::<AppStates>().get_apps();

    let mut app_map: HashMap<i32, String> = HashMap::new();
    let mut app_type_map: HashSet<String> = HashSet::new();

    for app in app_db {
        if app_type_map.insert(app.app_name.clone()) {
            app_map.insert(app.slot, app.app_name);
        }
    }

    let mut next_slot = 0;
    for app in ui_apps.iter() {
        if app.app_type == AppTypes::None {
            continue;
        }
        let val = format!("{:?}", app.app_type);
        
        if app_type_map.insert(val.clone()) {
            while app_map.contains_key(&next_slot) {
                next_slot += 1;
            }
            app_map.insert(next_slot, val);
            next_slot += 1;
        }
    }

    let app_limit = ui.global::<AppStates>().get_app_limit();
    let max_slot = app_map.keys().max().copied().unwrap_or(0);
    let total_pages = (max_slot / app_limit) + 1;
    
    let mut page_status = vec![false; total_pages as usize];
    for &slot in app_map.keys() {
        page_status[(slot / app_limit) as usize] = true;
    }

    let mut final_apps: Vec<AppData> = Vec::new();
    let mut empty_pages = 0;

    for page_idx in 0..total_pages {
        if page_status[page_idx as usize] {
            let start_slot = app_limit * page_idx;
            let end_slot = app_limit * (page_idx + 1);
            
            for slot in start_slot..end_slot {
                if let Some(v) = app_map.remove(&slot) {
                    if let Some(app_type) = get_app_type(&v) {
                        let new_slot = slot - (app_limit * empty_pages);
                        final_apps.push(AppData { app_type, slot: new_slot });
                    }
                }
            }
        } else {
            empty_pages += 1;
        }
    }

    let active_pages = page_status.iter().filter(|&&x| x).count();
    let page_num = if active_pages > 0 { active_pages - 1 } else { 0 };

    ui.global::<AppStates>().set_page_num(page_num as i32);
    ui.global::<AppStates>().set_apps(Rc::new(VecModel::from(final_apps)).into());

}

fn get_app_type(str: &String) -> Option<AppTypes> {
    if *str == format!("{:?}", AppTypes::RoomTemp) {
        return Some(AppTypes::RoomTemp);
    } else if *str == format!("{:?}", AppTypes::Other) {
        return Some(AppTypes::Other);
    }
    None
}
