#![no_std]
#![no_main]

use esp_alloc as _;
use esp_backtrace as _;
use esp_hal::{delay::Delay, prelude::*, rng::Rng, time, timer::timg::TimerGroup};
use esp_wifi::{
    init,
    wifi::{
        utils::create_network_interface, AccessPointConfiguration, Configuration, WifiApDevice,
    },
    wifi_interface::WifiStack,
    EspWifiInitFor,
};
use smoltcp::iface::SocketStorage;

#[entry]
fn main() -> ! {
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let delay = Delay::new();
    let timg0 = TimerGroup::new(peripherals.TIMG0);

    let init = init(
        EspWifiInitFor::Wifi,
        timg0.timer0,
        Rng::new(peripherals.RNG),
        peripherals.RADIO_CLK,
    )
    .unwrap();

    let mut wifi = peripherals.WIFI;
    let mut socket_set_entries: [SocketStorage; 3] = Default::default();
    let (iface, device, mut controller, sockets) =
        create_network_interface(&init, &mut wifi, WifiApDevice, &mut socket_set_entries).unwrap();
    let now = || time::now().duration_since_epoch().to_millis();
    let mut wifi_stack = WifiStack::new(iface, device, sockets, now);

    let client_config = Configuration::AccessPoint(AccessPointConfiguration {
        ssid: "test".try_into().unwrap(),
        ..Default::default()
    });

    esp_println::logger::init_logger_from_env();

    loop {
        log::info!("Hello world!");
        delay.delay(500.millis());
    }
}
