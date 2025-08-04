use esp_idf_hal::peripherals::Peripherals;
use esp_idf_sys as _;
use smart_leds::hsv::{hsv2rgb, Hsv};
use smart_leds::{SmartLedsWrite, RGB};
use ws2812_esp32_rmt_driver::Ws2812Esp32Rmt;

fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();
    let peripherals = Peripherals::take().unwrap();
    let led_pin = peripherals.pins.gpio4;
    let channel = peripherals.rmt.channel0;

    let mut ws2812 = Ws2812Esp32Rmt::new(channel, led_pin)?;
    const NUM_LEDS: usize = 100;

    let mut data = [RGB::default(); NUM_LEDS];
    let mut hue_offset: i32 = 0;
    let mut brightness: u8 = 254;
    let mut brightness_increase = 1;

    loop {
        for i in 0..NUM_LEDS {
            let hue = (((i as u32 * 255) / NUM_LEDS as u32 + hue_offset as u32) % 255) as u8;
            let hsv = Hsv {
                hue,
                sat: 255,
                val: brightness,
            };
            data[i] = hsv2rgb(hsv);
        }
        ws2812.write(data.iter().copied())?;

        hue_offset = hue_offset.wrapping_add(1);

        if brightness_increase == 0 {
            brightness = brightness - 1;
            if brightness == 0 {
                brightness_increase = 1
            }
        }
        if brightness_increase == 1 {
            brightness = brightness + 1;
            if brightness == 255 {
                brightness_increase = 0
            }
        }
        std::thread::sleep(std::time::Duration::from_millis(20));
    }
}
