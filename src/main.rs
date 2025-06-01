use core::fmt::Display;
use serde::__private::fmt::Formatter;
use std::fmt;
use reqwest::Client;
use serde_json::Value;
use std::error::Error;
use std::fs;

struct Package {
    name: String,
    version: String,
}

impl Display for Package {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        write!(f, "{} ({})", self.name, self.version)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Paketinformationen von Laravel115 fetchen
    let packages = fetch_packages_from_laravel115().await?;

    // Informationen im Terminal ausgeben
    for package in packages {
        println!("{}", package);
    }

    // HTTP-Anfrage durchführen
    let client = Client::new();
    let url = "http://laravel115.ddev.site/rust-data"; // Ersetzen Sie die URL durch die tatsächliche URL Ihrer Laravel-Anwendung

    let data = [("key1", "value1"), ("key2", "value2")]; // Die Daten, die Sie senden möchten

    let response = client
        .post(url)
        .form(&data)
        .send()
        .await?;

    if response.status().is_success() {
        println!("Anfrage erfolgreich gesendet!");
    } else {
        println!("Fehler beim Senden der Anfrage: {}", response.status());
    }

    Ok(())
}

// Hilfsfunktion, um Paketinformationen aus der composer.lock Datei von Laravel zu fetchen
async fn fetch_packages_from_laravel115() -> Result<Vec<Package>, Box<dyn Error>> {
    // Versuchen, die `composer.lock` Datei aus dem `laravel115` Ordner zu lesen
    let inhalt = fs::read_to_string("../laravel115/composer.lock")?;

    // Versuchen, den Inhalt der Datei als JSON zu parsen
    let data: Value = serde_json::from_str(&inhalt)?;

    let mut packages = Vec::new();

    // Prüfen, ob das JSON-Objekt einen Schlüssel "packages" enthält, der ein Array ist
    if let Some(pakete) = data["packages"].as_array() {
        // Durch jedes Paket im Array gehen
        for paket in pakete {
            // Den Namen und die Version des Pakets extrahieren
            let name = paket["name"].as_str().unwrap().to_string();
            let version = paket["version"].as_str().unwrap().to_string();
            // Paket erstellen und zur Ergebnisliste hinzufügen
            packages.push(Package { name, version });
        }
    }

    // Ergebnisliste zurückgeben
    Ok(packages)
}
