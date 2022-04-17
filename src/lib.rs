use std::process::Command;

// constants
const TRUNCATE_TEXT_SIZE: usize = 50;
const MARGIN: usize = 3;
const LEFT_MARGIN: usize = 1;
const ELLIPSES_TEXT: &str = "...";

fn get_name_value(env: &str) -> (&str, &str) {
    let e: Vec<&str> = env.split('=').collect();
    let name = e[0];
    let mut value: &str = "";

    if e.len() == 2 {
        value = e[1];
    }

    (name, value)
}

fn get_max_size(env_list: Vec<&str>) -> (usize, usize) {
    let mut name_max_size: usize = 0;
    let mut value_max_size: usize = 0;

    for env in env_list {
        let (name, value) = get_name_value(env);

        if name_max_size < name.len() {
            name_max_size = name.len();
        }

        if value_max_size < value.len() {
            value_max_size = value.len();
        }
    }

    if name_max_size > TRUNCATE_TEXT_SIZE {
        name_max_size = TRUNCATE_TEXT_SIZE;
    }

    if value_max_size > TRUNCATE_TEXT_SIZE {
        value_max_size = TRUNCATE_TEXT_SIZE;
    }

    (name_max_size, value_max_size)
}

fn get_chars(w: usize, c: char) -> String {
    let mut str: String = "".to_string();

    for _ in 0..w {
        str = str + &String::from(c);
    }

    str
}

pub fn pretty_env() {
    let output = Command::new("sh")
        .arg("-c")
        .arg("env")
        .output()
        .expect("failed to execute process");
    let env_str = String::from_utf8(output.stdout).unwrap();
    let env_list: Vec<&str> = env_str.split('\n').collect();

    let (name_max_size, value_max_size) = get_max_size(env_list.clone());
    let table_length = name_max_size + value_max_size + MARGIN;

    let mut display_text = get_chars(table_length, '-');

    for env in env_list {
        let mut name_space_length: usize = 0;
        let mut value_space_length: usize = 0;

        let (name, value) = get_name_value(env);

        if name.is_empty() {
            break;
        }

        if name_max_size > name.len() {
            name_space_length = name_max_size - name.len();
        }

        if value_max_size > value.len() {
            value_space_length = value_max_size - value.len();
        }

        display_text += "\n";
        display_text = display_text + &get_chars(1, '|');
        display_text = display_text + &get_chars(LEFT_MARGIN, ' ');

        if name.len() > (name_max_size - ELLIPSES_TEXT.len()) {
            display_text += &name[..(name_max_size - (ELLIPSES_TEXT.len() + 1))];
            display_text += ELLIPSES_TEXT;
        } else {
            display_text += name;
        }

        if name_space_length > 0 {
            display_text = display_text + &get_chars(name_space_length - LEFT_MARGIN, ' ');
        }

        display_text = display_text + &get_chars(1, '|');
        display_text = display_text + &get_chars(LEFT_MARGIN, ' ');

        if value.len() > (value_max_size - ELLIPSES_TEXT.len()) {
            display_text += &value[..(value_max_size - (ELLIPSES_TEXT.len() + 1))];
            display_text += ELLIPSES_TEXT;
        } else {
            display_text += value;
        }

        if value_space_length > 0 {
            display_text = display_text + &get_chars(value_space_length - LEFT_MARGIN, ' ');
        }

        display_text = display_text + &get_chars(1, '|');
        display_text = display_text + &'\n'.to_string();
        display_text = display_text + &get_chars(table_length, '-');
    }

    println!("{}", display_text);
}
