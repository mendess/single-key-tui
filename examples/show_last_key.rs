fn main() {
    let tui = single_key_tui::Tui::new(['q']).unwrap();
    println!("tui: {tui:#?}");

    let mut last_key = None;
    loop {
        println!("the last key was {last_key:?}");
        let Some(last) = tui.next_key().unwrap() else {
            return;
        };
        last_key = Some(last);
    }
}
