use std::collections::{HashMap, HashSet, hash_map::Entry};

use slint::{ComponentHandle, Model};
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
    let mut apps: Vec<AppData> = ui.global::<AppStates>().get_apps().iter().collect();

    let mut app_map: HashMap<i32, String> = HashMap::new();
    let mut app_type_map: HashSet<String> = HashSet::new();
    let mut i = 0;

    for app in app_db {
        if app_type_map.insert(app.app_name.clone()) {
            app_map.entry(app.slot).or_insert(app.app_name);
        }
    }

    for app in &mut apps {
        if app.app_type != AppTypes::None {
            continue;
        }
        let val = format!("{:?}", app.app_type);
        if app_type_map.insert(val.clone()) {
            loop {
                match app_map.entry(i) {
                    Entry::Vacant(entry) => {
                        entry.insert(val);
                        break;
                    }
                    Entry::Occupied(_) => {
                        i += 1;
                    }
                }
            }
        }
    }

    let app_limit = ui.global::<AppStates>().get_app_limit();
    let max_slot = app_map.keys().max().copied().unwrap_or(0);
    let total_pages = (max_slot / app_limit) + 1;
    let mut page_status = vec![false; total_pages as usize];

    for num in app_map.keys() {
        page_status[(num / app_limit) as usize] = true;
    }

    let mut empty_pages = 0;
    let mut apps: Vec<AppData> = Vec::new();

    if page_status.contains(&false) {
        for i in 0..total_pages {
            if page_status[i as usize] {
                if empty_pages > 0 {
                    for slot in (app_limit * i)..(app_limit * (i + 1)) {
                        if let Some((_, v)) = app_map.remove_entry(&slot) {
                            if let Some(app_type) = get_app_type(&v) {
                                apps.push(AppData {
                                    app_type,
                                    slot: slot - app_limit * empty_pages,
                                });
                            }
                        }
                    }
                } else {
                    for slot in (app_limit * i)..(app_limit * (i + 1)) {
                        if let Some((_, v)) = app_map.remove_entry(&slot) {
                            if let Some(app_type) = get_app_type(&v) {
                                apps.push(AppData { app_type, slot });
                            }
                        }
                    }
                }
            } else {
                empty_pages += 1;
            }
        }
    } else {
        for (slot, app_type) in app_map.iter() {
            //　ここから
        }
    }
}

fn get_app_type(str: &String) -> Option<AppTypes> {
    if *str == format!("{:?}", AppTypes::RoomTemp) {
        return Some(AppTypes::RoomTemp);
    }
    None
}
