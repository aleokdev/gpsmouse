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

fn gps_device(index: i32) -> anyhow::Result<Ftdi> {
    let mut dev = libftd2xx::Ftdi::with_index(index)?;
    dev.set_baud_rate(625000)?;
    dev.set_data_characteristics(
        libftd2xx::BitsPerWord::Bits8,
        libftd2xx::StopBits::Bits2,
        libftd2xx::Parity::No,
    )?;
    dev.set_flow_control_none()?;
    Ok(dev)
}

fn main() -> anyhow::Result<()> {
    libftd2xx::set_vid_pid(0x0403, 0xe8db)?;
    let num_devices = num_devices()?;
    println!("Number of devices: {}", num_devices);

    println!("listing devices:");
    for device in list_devices()? {
        println!("{:?}", device);
    }
    let mut device_a = gps_device(0)?;
    let mut device_b = gps_device(1)?; // I have no clue what the second device is for

    println!(
        "\ndevices used:\n{:?}; {:?}",
        device_a.device_info(),
        device_b.device_info()
    );
    println!("writing &PAAG,ID, PAAG,MODE,RATE, PAAG,MODE,START and PAAG,VAR,ACCRANGE");
    device_a.write_all("$PAAG,ID\r\n".as_bytes())?;
    device_a.write_all("$PAAG,MODE,RATE,1\r\n".as_bytes())?;
    device_a.write_all("$PAAG,MODE,START\r\n".as_bytes())?;
    device_a.write_all("$PAAG,VAR,ACCRANGE,2\r\n".as_bytes())?;
    device_a.set_timeouts(Duration::from_millis(1), Duration::from_millis(1))?;
    let mut buf = [0; 512];
    println!("reading back");
    loop {
        let rx_bytes = device_a.read(&mut buf)?;

        if rx_bytes > 0 {
            print!("{}", String::from_utf8_lossy(&buf[0..rx_bytes]))
        }
    }
}
