mod security;
mod sensors;
mod comms;
mod analysis;
mod satellite;

fn main() {
    // Güvenlik kontrolleri
    security::check_ufw_status();
    security::check_open_ports();

    // Sürekli veri toplama ve işleme döngüsü
    loop {
        // Sensör verilerini topla
        let sensor_data = sensors::collect_data();
        
        // Uydu verilerini al ve analiz et
        match satellite::fetch_satellite_data("coordinates") {
            Ok(satellite_data) => println!("Satellite data: {}", satellite_data),
            Err(e) => println!("Error fetching satellite data: {}", e),
        }

        // Dalga görüntülerini analiz et
        analysis::analyze_wave_images();

        // Verileri şifrele ve kaptan kabinine ilet
        let key = [0u8; 16];
        let iv = [0u8; 16];
        let encrypted_data = comms::encrypt_data(&key, &iv, &sensor_data);
        comms::send_data_to_cockpit();
        
        // CRC kontrolü
        let crc = comms::crc64_check(&sensor_data);
        println!("Received data: {:?}, CRC-64: {:x}", &sensor_data, crc);
    }
}
