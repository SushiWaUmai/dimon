// Xorg
use std::ptr::*;
use x11::xlib::*;
use x11::xrandr::*;

fn main() {
    unsafe {
        let display = XOpenDisplay(null());
        let window = XDefaultRootWindow(display);

        // get cursor position
        let (mut x, mut y, mut win_x, mut win_y, mut mask) = (0, 0, 0, 0, 0);
        let mut root_win = XRootWindow(display, 0);
        let mut child_win = XRootWindow(display, 0);

        XQueryPointer(
            display,
            window,
            &mut root_win,
            &mut child_win,
            &mut x,
            &mut y,
            &mut win_x,
            &mut win_y,
            &mut mask,
        );

        let mut resources = *XRRGetScreenResources(display, window);

        for i in 0..resources.noutput {
            let output_info = *XRRGetOutputInfo(
                display,
                &mut resources,
                *resources.outputs.offset(i as isize),
            );

            if output_info.connection == RR_Connected as u16 {
                let crt_info = *XRRGetCrtcInfo(display, &mut resources, output_info.crtc);

                // check if cursor is on this output
                if crt_info.x <= x
                    && x <= crt_info.x + crt_info.width as i32
                    && crt_info.y <= y
                    && y <= crt_info.y + crt_info.height as i32
                {
                    println!(
                        "{}x{}+{}+{}",
                        crt_info.width, crt_info.height, crt_info.x, crt_info.y
                    );
                }
            }
        }
    }
}
