use hidapi::HidApi;
use r2_tester::{dualsense::DualSenseDriver, interface::RGB};
use serialport;


fn main() {
    let hidapi = HidApi::new().unwrap();

    let dev = hidapi.open(1356, 3302).unwrap();

    let mut ds_driver = DualSenseDriver{device : dev, mode : r2_tester::interface::ControllerConnectionType::SERIAL, rgb : RGB::new()};

    let mut prev_rrpm = 0;
    let mut prev_lrpm = 0;

    let mut send_rrpm = 0;
    let mut send_lrpm = 0;

    let mut port = serialport::new("/dev/ttyUSB0", 115200).timeout(std::time::Duration::from_millis(100)).open().unwrap();


    loop {
        let input = ds_driver.task();

        let rotation = -1.0*input.sticks.left_y;
        let y = input.sticks.right_x;

        let left = y * 0.5 + rotation * 0.5;
        let right = y * -0.5 + rotation * 0.5;

        let left_rpm = (left * 10.0) as i32;
        let right_rpm = (right * 10.0) as i32;

        let vec_l = left_rpm - prev_lrpm;
        let vec_r = right_rpm - prev_rrpm;

        if vec_l > 0
        {
            send_lrpm += 1;
        }
        else if vec_l < 0{
            send_lrpm -= 1;
        }
        else {
            send_lrpm = left_rpm;
        }

        if vec_r > 0
        {
            send_rrpm += 1;
        }
        else if vec_r < 0{
            send_rrpm -= 1;
        }
        else {
            send_rrpm = right_rpm;
        }

        let msg = format!("3,{},{},20,20e", send_lrpm+20, send_rrpm+20);

            match port.write(msg.as_bytes()) {
                Ok(_size)=>{
                    println!("{}", msg)
                }
                Err(_e)=>{

                }
            }

        prev_lrpm = send_lrpm;
        prev_rrpm = send_rrpm;
    }
}
