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

        // 添加更多键映射...

        key_map.get(content).cloned()
    }
}
