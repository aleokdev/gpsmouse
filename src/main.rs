use std::io::Cursor;
use std::io::Read;
use std::io::Write;
use std::ops::Deref;
use std::ops::DerefMut;
use std::str::from_utf8;
use std::time::Duration;

use libftd2xx::list_devices;
use libftd2xx::num_devices;
use libftd2xx::Ftdi;
use libftd2xx::FtdiCommon;

fn main() {
    libftd2xx::set_vid_pid(0x0403, 0xe8db).unwrap();
    let num_devices = num_devices().unwrap();
    println!("Number of devices: {}", num_devices);

    println!("listing devices:");
    for device in list_devices().unwrap() {
        println!("{:?}", device);
    }
    let mut device_a = libftd2xx::Ftdi::with_index(0).unwrap();
    device_a.set_baud_rate(625000).unwrap();
    device_a
        .set_data_characteristics(
            libftd2xx::BitsPerWord::Bits8,
            libftd2xx::StopBits::Bits2,
            libftd2xx::Parity::No,
        )
        .unwrap();
    device_a.set_flow_control_none().unwrap();
    let mut device_b = libftd2xx::Ftdi::with_index(1).unwrap();
    device_b.set_baud_rate(625000).unwrap();
    device_b
        .set_data_characteristics(
            libftd2xx::BitsPerWord::Bits8,
            libftd2xx::StopBits::Bits2,
            libftd2xx::Parity::No,
        )
        .unwrap();
    device_b.set_flow_control_none().unwrap();
    println!("\ndevices used:\n{:?}", device_a.device_info());
    println!("{:?}", device_b.device_info());
    println!("writing &PAAG,ID");
    device_a.write_all("$PAAG,ID\r\n".as_bytes()).unwrap();
    device_a
        .write_all("$PAAG,MODE,RATE,1\r\n".as_bytes())
        .unwrap();
    device_a
        .write_all("$PAAG,MODE,START\r\n".as_bytes())
        .unwrap();
    device_a
        .write_all("$PAAG,VAR,ACCRANGE,2\r\n".as_bytes())
        .unwrap();
    device_a
        .set_timeouts(Duration::from_millis(1), Duration::from_millis(1))
        .unwrap();
    let mut msg_until_now = String::new();
    let mut buf = [0; 512];
    println!("reading back");
    loop {
        let rx_bytes = device_a.queue_status().unwrap();
        if rx_bytes > 0 {
            let rx_bytes = device_a.read(&mut buf).unwrap();
            print!("{}", String::from_utf8_lossy(&buf[0..rx_bytes]));
        }
    }
}
