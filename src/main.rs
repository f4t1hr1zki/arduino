#![no_std]
#![no_main]

use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    // Mengambil kontrol periferal Arduino Uno
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    // Konfigurasi pin digital sesuai dengan jalur kabel SimulIDE Anda
    let mut led_blue = pins.d9.into_output();
    let mut led_green = pins.d10.into_output();
    let mut led_red = pins.d11.into_output();

    loop {
        // 1. Nyalakan Warna Biru saja
        led_blue.set_high();
        led_green.set_low();
        led_red.set_low();
        arduino_hal::delay_ms(500);

        // 2. Nyalakan Warna Hijau saja
        led_blue.set_low();
        led_green.set_high();
        led_red.set_low();
        arduino_hal::delay_ms(500);

        // 3. Nyalakan Warna Merah saja
        led_blue.set_low();
        led_green.set_low();
        led_red.set_high();
        arduino_hal::delay_ms(500);
    }
}
