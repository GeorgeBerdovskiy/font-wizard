#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::Manager;
use window_vibrancy::{apply_vibrancy, NSVisualEffectMaterial};

use postgres::{Client, NoTls};
use sea_query::{ColumnDef, Iden, PostgresQueryBuilder, Table};

#[derive(Iden)]
enum FontBox {
    Table, Id, Decimal
}

fn initialize_postgres() -> Result<Client, postgres::Error> {
	let client_attempt = Client::connect("host=localhost user=font_wizard password=password dbname=font_wizard_database", NoTls);

	match client_attempt {
        Ok(mut valid_client) => {
			// Database schema
			let sql = [
				Table::drop()
					.table(FontBox::Table)
					.if_exists()
					.build(PostgresQueryBuilder),
				Table::create()
					.table(FontBox::Table)
					.if_not_exists()
					.col(
						ColumnDef::new(FontBox::Id)
							.integer()
							.not_null()
							.auto_increment()
							.primary_key(),
					)
					.col(ColumnDef::new(FontBox::Decimal).decimal())
					.build(PostgresQueryBuilder),
			].join("; ");

			let result = valid_client.batch_execute(&sql);
			println!("Create table 'font_box' - {:?}", result);

			return Ok(valid_client);
		},

        Err(error) => {
			return Err(error);
		}
    };
}

#[tauri::command]
fn collect_boxes() -> Vec<String> {
	// TODO - Replace with database queries
	let mut boxes = Vec::<String>::new();
	boxes.push("Modern Fonts".to_string());
	boxes.push("Classic Fonts".to_string());

    boxes
}

fn main() {
	// Set up PostgreSQL
	let client: Client;
	let result = initialize_postgres();

	match result {
		Ok(valid_client) => {
			client = valid_client;
			println!("SUCCESS - Connected to PostgreSQL!")
		}, 
		
		Err(error) => {
			println!("ERROR - Failed to connect to PostgreSQL.");
			panic!("{:?}", error);
		}
	}

    tauri::Builder::default()
		.setup(|app| {
       		let window = app.get_window("main").unwrap();

			#[cfg(target_os = "macos")]
			apply_vibrancy(&window, NSVisualEffectMaterial::HudWindow, None, None)
				.expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");

    		Ok(())
        })
		.invoke_handler(tauri::generate_handler![collect_boxes])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
