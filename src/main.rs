/*
By: <Asen Doiron>
Date: 2026-03-26
Program Details: <The purpose of this program is to allow the user to type in two numbers and perform addition, subtraction, multiplication, and division on those numbers and give the user the answer.>
*/

mod modules;

use crate::modules::grid::draw_grid;
use crate::modules::label::Label;
use crate::modules::text_button::TextButton;
use crate::modules::text_input::TextInput;
use macroquad::prelude::*;

/// Set up window settings before the app runs
fn window_conf() -> Conf {
    Conf {
        window_title: "basiccalculator".to_string(),
        window_width: 1024,
        window_height: 768,
        fullscreen: false,
        high_dpi: true,
        window_resizable: true,
        sample_count: 4, // MSAA: makes shapes look smoother
        ..Default::default()
    }
}

fn add(num1: f64, num2: f64) -> f64 {
    num1 + num2
}

fn subtract(num1: f64, num2: f64) -> f64 {
    num1 - num2
}

fn multiply(num1: f64, num2: f64) -> f64 {
    num1 * num2
}

fn divide(num1: f64, num2: f64) -> f64 {
    num1 / num2
}

#[macroquad::main(window_conf)]
async fn main() {
    let btn_exit = TextButton::new(650.0, 100.0, 50.0, 50.0, "X", RED, ORANGE, 30);

    let mut txt_num1 = TextInput::new(250.0, 225.0, 150.0, 40.0, 25.0);

    let mut txt_num2 = TextInput::new(550.0, 225.0, 150.0, 40.0, 25.0);

    let mut lbl_answer = Label::new("=", 553.0, 328.0, 30);

    let lbl_help = Label::new("Type a number in \neach box and click\none of the operators", 220.0, 325.0, 25);

    let btn_add = TextButton::new(450.0, 125.0, 50.0, 50.0, "+", BLUE, PURPLE, 30);

    let btn_subtract = TextButton::new(450.0, 200.0, 50.0, 50.0, "-", BLUE, PURPLE, 30);

    let btn_multiply = TextButton::new(450.0, 275.0, 50.0, 50.0, "X", BLUE, PURPLE, 30);

    let btn_divide = TextButton::new(450.0, 350.0, 50.0, 50.0, "/", BLUE, PURPLE, 30);

    loop {
        clear_background(WHITE);
        draw_grid(50.0, BLACK);

        if btn_exit.click() {
            break;
        }

        if btn_add.click() {
            let _num1: f64 = txt_num1.get_text().trim().parse().unwrap_or(0.0);
            let _num2: f64 = txt_num2.get_text().trim().parse().unwrap_or(0.0);
            let num1 = txt_num1.get_text().parse::<f64>();
            let num2 = txt_num2.get_text().parse::<f64>();
            if let (Ok(num1), Ok(num2)) = (num1, num2) {
               
                lbl_answer.set_text(&format!("= {}", add(num1, num2)));
            } else {
                lbl_answer.set_text("Invalid");
            }
        }
        

        if btn_subtract.click() {
            let _num1: f64 = txt_num1.get_text().trim().parse().unwrap_or(0.0);
            let _num2: f64 = txt_num2.get_text().trim().parse().unwrap_or(0.0);
            let num1 = txt_num1.get_text().parse::<f64>();
            let num2 = txt_num2.get_text().parse::<f64>();
            if let (Ok(num1), Ok(num2)) = (num1, num2) {
               
                lbl_answer.set_text(&format!("= {}", subtract(num1, num2)));
            } else {
                lbl_answer.set_text("Invalid");
            }
        }

        if btn_multiply.click() {
            let _num1: f64 = txt_num1.get_text().trim().parse().unwrap_or(0.0);
            let _num2: f64 = txt_num2.get_text().trim().parse().unwrap_or(0.0);
            let num1 = txt_num1.get_text().parse::<f64>();
            let num2 = txt_num2.get_text().parse::<f64>();
            if let (Ok(num1), Ok(num2)) = (num1, num2) {
               
                lbl_answer.set_text(&format!("= {}", multiply(num1, num2)));
            } else {
                lbl_answer.set_text("Invalid");
            }
        }

        

        if btn_divide.click() {
            let _num1: f64 = txt_num1.get_text().trim().parse().unwrap_or(0.0);
            let _num2: f64 = txt_num2.get_text().trim().parse().unwrap_or(0.0);
            let num1 = txt_num1.get_text().parse::<f64>();
            let num2 = txt_num2.get_text().parse::<f64>();
            if let (Ok(num1), Ok(num2)) = (num1, num2) {
               
                lbl_answer.set_text(&format!("= {}", divide(num1, num2)));
            } else {
                lbl_answer.set_text("Invalid");
            }
        }

        txt_num1.draw();
        txt_num2.draw();
        lbl_answer.with_border(BLACK, 3.0).with_fixed_size(153.0, 53.0).draw();
        lbl_help.draw();
        next_frame().await;
    }}
    
