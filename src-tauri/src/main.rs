// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use sqlx::{ migrate::MigrateDatabase, Sqlite };
use tauri::{ Manager, Window };
#[path = "./service/db.rs"]
mod db;
#[async_std::main]
async fn main() {
    let db_url = String::from("sqlite://app.db");
    if !Sqlite::database_exists(&db_url).await.unwrap_or(false) {
        Sqlite::create_database(&db_url).await.unwrap();
        match db::create_tables(&db_url).await {
            Ok(_) => println!("Database created successfully"),
            Err(e) => panic!("{}", e),
        }
    }

    tauri::Builder
        ::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_notification::init())
        .invoke_handler(
            tauri::generate_handler![
                db::check_entity,
                db::load_entity,
                db::login,
                db::search_products,
                db::save_transaction,
                db::synchronize,
                db::get_transactions,
                db::search_transactions,
                db::refund_transaction,
                db::save_customer,
                db::get_customers,
                db::force_update
            ]
        )
        .setup(|app| {
            let splashscreen_window = app.get_webview_window("splashscreen").unwrap();
            let main_window = app.get_webview_window("main").unwrap();
            // we perform the initialization code on a new task so the app doesn't freeze
            tauri::async_runtime::spawn(async move {
                // initialize your app here instead of sleeping :)
                println!("Initializing...");
                std::thread::sleep(std::time::Duration::from_secs(3));
                println!("Done initializing.");

                // After it's done, close the splashscreen and display the main window
                splashscreen_window.close().unwrap();
                main_window.show().unwrap();
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
#[tauri::command]
async fn close_splashscreen(window: Window) {
    // Close splashscreen
    window
        .get_webview_window("splashscreen")
        .expect("no window labeled 'splashscreen' found")
        .close()
        .unwrap();
    // Show main window
    window.get_webview_window("main").expect("no window labeled 'main' found").show().unwrap();
}
