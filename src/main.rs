#![no_std]
#![no_main]

use panic_halt as _;
use max7219::MAX7219;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    // 1. PIN UNTUK LED RGB
    let mut led_blue = pins.d5.into_output();
    let mut led_green = pins.d6.into_output();
    let mut led_red = pins.d7.into_output();

    // 2. PIN UNTUK SPEAKER TOKEN LISTRIK
    let mut speaker = pins.d8.into_output();

    // 3. PIN UNTUK SERVO MOTOR (Ubah jadi Output Biasa untuk Kontrol Manual)
    let mut motor = pins.d10.into_output();
    
    // 4. PIN UNTUK MAX7219 LED MATRIX
    let din = pins.d11.into_output(); 
    let cs = pins.d12.into_output();  
    let sck = pins.d13.into_output(); 

    // Inisialisasi Driver MAX7219
    let mut display = MAX7219::from_pins(1, din, cs, sck).unwrap();
    display.power_on().unwrap(); 
    display.set_intensity(0, 0x0F).unwrap(); 
    
    loop {
        // ==========================================
        // 1. ANIMASI LED RGB (Biru -> Hijau)
        // ==========================================
        led_blue.set_high();
        led_green.set_low();
        led_red.set_low();
        arduino_hal::delay_ms(400);

        led_blue.set_low();
        led_green.set_high();
        led_red.set_low();
        arduino_hal::delay_ms(400);

        // ==========================================
        // 2. LED MERAH + BEEP + SERVO BERGERAK KE 180°
        // ==========================================
        led_green.set_low();
        led_red.set_high();
        
        // Kirim pulsa manual $2.4\text{ ms}$ ( HIGH) berkali-kali agar servo SimulIDE berputar ke 180°
        for _ in 0..30 {
            motor.set_high();
            arduino_hal::delay_us(2400); // Pulsa sudut 180 derajat
            motor.set_low();
            arduino_hal::delay_us(17600); // Sisa jeda periode PWM (Total 20ms)
        }
        
        // Efek suara beep token listrik
        for _ in 0..300 {
            speaker.set_high();
            arduino_hal::delay_us(250);
            speaker.set_low();
            arduino_hal::delay_us(250);
        }
        led_red.set_low();
        arduino_hal::delay_ms(300);

        // ==========================================
        // 3. ANIMASI MAX7219 LED MATRIX
        // ==========================================
        for row in 0..8 {
            display.write_raw_byte(0, row, 0b11111111).unwrap();
            arduino_hal::delay_ms(60); 
            display.write_raw_byte(0, row, 0b00000000).unwrap();
        }

        // ==========================================
        // 4. SERVO KEMBALI KE 0° SEBELUM LOOP DIULANG
        // ==========================================
        // Kirim pulsa manual $0.6\text{ ms}$ ( HIGH) berkali-kali agar servo kembali ke 0°
        for _ in 0..30 {
            motor.set_high();
            arduino_hal::delay_us(600); // Pulsa sudut 0 derajat
            motor.set_low();
            arduino_hal::delay_us(19400); // Sisa jeda periode PWM (Total 20ms)
        }
        
        arduino_hal::delay_ms(400); 
    }
}
