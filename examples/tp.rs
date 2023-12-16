#![no_std]
#![no_main]

use esp32_trackpoint_example::trackpoint::TrackPoint;
use esp_backtrace as _;
use esp_println::println;
use hal::{clock::ClockControl, peripherals::Peripherals, prelude::*, Delay, IO};

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();

    let clocks = ClockControl::max(system.clock_control).freeze();
    let delay = Delay::new(&clocks);

    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let p_rst = io.pins.gpio5.into_push_pull_output();
    let p_clk = io.pins.gpio6.into_open_drain_output();
    let p_data = io.pins.gpio7.into_open_drain_output();

    let mut trackpoint = TrackPoint::new(p_clk, p_data, p_rst, delay);
    trackpoint.reset();
    let (mut pre_btns, mut pre_x, mut pre_y) = (0, 0, 0);
    println!("start remote mode");

    loop {
        let tp_data = trackpoint.query_data_report();
        let cur_btns = tp_data.state % 8; // btn1: 1, btn2: 2, btn3: 4
        if cur_btns != pre_btns || tp_data.x != pre_x || tp_data.y != pre_y {
            pre_btns = cur_btns;
            pre_x = tp_data.x;
            pre_y = tp_data.y;
            println!("button: {}, x: {}, y: {}", pre_btns, pre_x, pre_y);
        }
    }
}
