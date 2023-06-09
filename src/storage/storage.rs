use std::{env, fs};
use std::fs::File;
use std::path::Path;

use crate::bluetooth::Message;
use crate::bluetooth::utils::{hex_string_to_letter, hex_to_keyword, split_string};

#[derive(Debug, Clone)]
pub struct Storage {
    badge_storage_dir: String,
    badge_ext: String,
}

impl Storage {
    fn create_and_get_storage_dir() -> String {
        let working_dir: String = format!("{}{}", env::current_dir().unwrap().into_os_string().into_string().unwrap(), String::from("/bitBlinkData/"));
        fs::create_dir_all(&working_dir).unwrap();
        working_dir
    }
    pub fn save_message(&self, message: &mut Message) {
        let timestamp = chrono::Utc::now().format("%d-%m-%Y-%M-%S-%f").to_string();
        let target: String = self.get_full_badge_filename(&timestamp) + &*self.badge_ext;
        message.file_name = timestamp + &*self.badge_ext;
        let json = hex_string_to_json(message);
        File::create(&target).unwrap();
        fs::write(Path::new(&target), json).expect("Unable to write file")
    }

    pub fn get_all_messages(&self) -> Vec<Message> {
        let mut messages : Vec<Message> = vec![];
        let paths = fs::read_dir("./bitBlinkData").unwrap();
        for path in paths {
            let file_name : String = path.unwrap().file_name().into_string().unwrap();
            if file_name.contains(&self.badge_ext) {
                messages.push(self.load_badge(&file_name));
            }
        }
        messages
    }
    fn load_badge(&self, f_name: &String) -> Message {
        let target: String = self.get_full_badge_filename(&f_name);
        let message: Message = json_to_message(&fs::read_to_string(target).expect("Unable to read file"));
        message
    }
    pub fn delete_badge(&self, f_name: &String) {
        fs::remove_file(self.get_full_badge_filename(&f_name)).expect("File couldn't be deleted");
        println!("File deleted successfully!");
    }
    fn import_badge_to_app_dir(&self, path_to_file: &String) {   //import function of external badge files; not yet implemented in the front end
        let parts: Vec<&str> = path_to_file.split("/").collect();
        let f_name: &str = parts[&parts.len() - 1];
        fs::copy(path_to_file, self.get_full_badge_filename(&f_name.to_owned())).expect("Badge Import failed");
    }
    fn get_full_badge_filename(&self, f_name: &String) -> String {
        let filename = self.badge_storage_dir.clone() + f_name;
        filename
    }
}

fn json_to_message(json: &String) -> Message {
    let mut json_copy = json.clone();
    if json.contains("hex_strings") {
        json_copy = json.replace("hex_strings", "texts");
    }
    let mut message: Message = serde_json::from_str(&*json_copy).unwrap();
    for i in 0..message.texts.len() {
        let message_text = message.texts[i].clone();
        let subs: Vec<&str> = split_string(&message_text, 22);
        let mut hex_string: String = "".to_owned();
        for j in 0..subs.len() {
            let mut letter = hex_string_to_letter(subs[j]);
            if letter == "" {       //if hexstring is keyword
                letter = hex_to_keyword(subs[j]);
                if letter == "" && j < subs.len()-1 {   //if keyword-hexstring is 44 digits long
                    letter = hex_to_keyword((subs[j].to_owned() + subs[j+1]).as_str());
                }
                if letter == "" && j < subs.len()-2 {   //if keyword-hexstring is 66 digits long
                    letter = hex_to_keyword((subs[j].to_owned() + subs[j+1] + subs[j+2]).as_str());
                }
            }
            hex_string = hex_string + letter;
        }
        message.texts[i] = hex_string;
    }
    message
}

fn hex_string_to_json(message: &Message) -> String {
    let json: String = serde_json::to_string(&message).unwrap();
    json
}

pub fn build_storage() -> Storage {     // needs to be executed before the Storage struct can be used
    let main_dir: String = Storage::create_and_get_storage_dir();
    Storage {
        badge_storage_dir: main_dir,
        badge_ext: ".txt".to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bluetooth::message;
    use crate::bluetooth::message::{Animation, Message, Speed};

    fn give_example_message () -> Message {
        let file_name = String::from("");
        let texts = vec!(String::from("test"),String::from("abc"),String::from("123"));
        let inverted = vec!(false, true, true);
        let flash = vec!(false, true, true);
        let marquee = vec!(false, true, true);
        let speed = vec!(Speed::One, Speed::Eight, Speed::Four);
        let mode = vec!(Animation::Left, Animation::Laser, Animation::Curtain);

        Message{file_name, texts, inverted, flash, marquee, speed, mode}
    }

    fn initialize_empty_storage_for_test() -> Storage {
        let storage = build_storage();
        delete_all_message(&storage);
        storage
    }

    fn initialize_storage_for_test() -> Storage {
        let storage = build_storage();
        delete_all_message(&storage);
        let mut message = give_example_message();
        storage.save_message(&mut message);
        storage
    }

    fn delete_all_message(storage: &Storage) {
        for m in storage.get_all_messages()  {
            println!("{}", &m.file_name);
            storage.delete_badge(&m.file_name);
        }
    }

    #[test]
    fn save_message_test() {
        let storage = initialize_empty_storage_for_test();
        let mut message = give_example_message();
        storage.save_message(&mut message);
        let result = storage.get_all_messages();
        let message_result = storage.load_badge(&result[0].file_name);

        assert_eq!("test", message_result.texts[0]);
        assert_eq!("abc", message_result.texts[1]);
        assert_eq!("123", message_result.texts[2]);

        assert_eq!(false, message_result.inverted[0]);
        assert_eq!(true, message_result.inverted[1]);
        assert_eq!(true, message_result.inverted[2]);

        assert_eq!(false, message_result.flash[0]);
        assert_eq!(true, message_result.flash[1]);
        assert_eq!(true, message_result.flash[2]);

        assert_eq!(false, message_result.marquee[0]);
        assert_eq!(true, message_result.marquee[1]);
        assert_eq!(true, message_result.marquee[2]);

        assert_eq!(Speed::One, message_result.speed[0]);
        assert_eq!(Speed::Eight, message_result.speed[1]);
        assert_eq!(Speed::Four, message_result.speed[2]);

        assert_eq!(Animation::Left, message_result.mode[0]);
        assert_eq!(Animation::Laser, message_result.mode[1]);
        assert_eq!(Animation::Curtain, message_result.mode[2]);

        delete_all_message(&storage);
    }

    #[test]
    fn delete_badge_test() {
        let storage = initialize_storage_for_test();
        let v1 = storage.get_all_messages();
        let name_of_deleted_file = &v1[0].file_name;

        storage.delete_badge(&v1[0].file_name);
        let v2 = storage.get_all_messages();

        assert_eq!(v1.len() - 1, v2.len());
        if(v2.len()>0) {
            assert_ne!(name_of_deleted_file, &v2[0].file_name);
        }
        delete_all_message(&storage);
    }

    #[test]
    fn get_all_messages_test() {
        let storage = initialize_storage_for_test();
        let result = storage.get_all_messages();

        assert_eq!(1, result.len());
        delete_all_message(&storage);
    }

    /*
    #[test]
    fn build_single_message_from_first_text_vec_of_given_messages_test () {
        let file_name = String::from("Test");
        let texts = vec!(String::from("test"));
        let inverted = vec!(false);
        let flash = vec!(false);
        let marquee = vec!(false);
        let speed = vec!(Speed::One);
        let mode = vec!(Animation::Left);

        let message_result = Message{file_name, texts, inverted, flash, marquee, speed, mode};

        let message = give_example_message();
        let storage = initialize_storage_for_test();
        storage.build_single_message_from_first_text_vec_of_given_messages(&message);
        assert!(true);
    }
     */

}
