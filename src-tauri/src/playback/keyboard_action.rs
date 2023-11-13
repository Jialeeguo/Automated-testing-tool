pub mod keyboard {
    use enigo::Key;
    use std::collections::HashMap;

    //键盘按键哈希表查找
    pub fn get_key(content: &str) -> Option<Key> {
        let mut key_map: HashMap<String, Key> = HashMap::new();
        key_map.insert("Enter".to_string(), Key::Return);
        key_map.insert("Space".to_string(), Key::Space);
        key_map.insert("Backspace".to_string(), Key::Backspace);
        key_map.insert("Tab".to_string(), Key::Tab);
        key_map.insert("CapsLock".to_string(), Key::CapsLock);
        key_map.insert("Escape".to_string(), Key::Escape);
        key_map.insert("LShift".to_string(), Key::LShift);
        key_map.insert("Shift".to_string(), Key::Shift);
        key_map.insert("RShift".to_string(), Key::RShift);
        key_map.insert("LControl".to_string(), Key::LControl);
        key_map.insert("Control".to_string(), Key::Control);
        key_map.insert("RControl".to_string(), Key::RControl);
        // key_map.insert("LWin".to_string(), Key::LWin);
        // key_map.insert("RWin".to_string(), Key::RWin);
        key_map.insert("F1".to_string(), Key::F1);
        key_map.insert("F2".to_string(), Key::F2);
        key_map.insert("F3".to_string(), Key::F3);
        key_map.insert("F4".to_string(), Key::F4);
        key_map.insert("F5".to_string(), Key::F5);
        key_map.insert("F6".to_string(), Key::F6);
        key_map.insert("F7".to_string(), Key::F7);
        key_map.insert("F8".to_string(), Key::F8);
        key_map.insert("F9".to_string(), Key::F9);
        key_map.insert("F10".to_string(), Key::F10);
        key_map.insert("F11".to_string(), Key::F11);
        key_map.insert("F12".to_string(), Key::F12);
        // key_map.insert("PrintScreen".to_string(), Key::PrintScreen);
        // key_map.insert("ScrollLock".to_string(), Key::ScrollLock);
        key_map.insert("Pause".to_string(), Key::Pause);
        key_map.insert("Insert".to_string(), Key::Insert);
        key_map.insert("Home".to_string(), Key::Home);
        key_map.insert("PageUp".to_string(), Key::PageUp);
        key_map.insert("Delete".to_string(), Key::Delete);
        key_map.insert("End".to_string(), Key::End);
        key_map.insert("PageDown".to_string(), Key::PageDown);
        key_map.insert("Right".to_string(), Key::RightArrow);
        key_map.insert("Left".to_string(), Key::LeftArrow);
        key_map.insert("Down".to_string(), Key::DownArrow);
        key_map.insert("Up".to_string(), Key::UpArrow);
        key_map.insert("NumLock".to_string(), Key::Numlock);
        // key_map.insert("NumPad0".to_string(), Key::Num0);
        // key_map.insert("NumPad1".to_string(), Key::Num1);
        // key_map.insert("NumPad2".to_string(), Key::Num2);
        // key_map.insert("NumPad3".to_string(), Key::Num3);
        // key_map.insert("NumPad4".to_string(), Key::Num4);
        // key_map.insert("NumPad5".to_string(), Key::Num5);
        // key_map.insert("NumPad6".to_string(), Key::Num6);
        // key_map.insert("NumPad7".to_string(), Key::Num7);
        // key_map.insert("NumPad8".to_string(), Key::Num8);
        // key_map.insert("NumPad9".to_string(), Key::Num9);
        // key_map.insert("Multiply".to_string(), Key::Multiply);
        // key_map.insert("Add".to_string(), Key::Add);
        // key_map.insert("Subtract".to_string(), Key::Subtract);
        // key_map.insert("Decimal".to_string(), Key::Decimal);
        // key_map.insert("Divide".to_string(), Key::Divide);
        // key_map.insert("Semicolon".to_string(), Key::Semicolon);
        // key_map.insert("Equals".to_string(), Key::Equals);
        // key_map.insert("Comma".to_string(), Key::Comma);
        // key_map.insert("Minus".to_string(), Key::Minus);
        // key_map.insert("Period".to_string(), Key::Period);
        // key_map.insert("Slash".to_string(), Key::Slash);
        // key_map.insert("Grave".to_string(), Key::Grave);
        // key_map.insert("LeftBracket".to_string(), Key::LeftBracket);
        // key_map.insert("Backslash".to_string(), Key::Backslash);
        // key_map.insert("RightBracket".to_string(), Key::RightBracket);
        // key_map.insert("Quote".to_string(), Key::Quote);
        // 添加更多键映射...

        key_map.get(content).cloned()
    }
}
