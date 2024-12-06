use std::{collections::HashMap, fs};

fn main() {
    let data: String = convert_to_json(&read_file("./username.csv"));
    fs::write("./username.json", data).unwrap();
}

fn read_file(file_path: &str) -> Vec<Vec<String>> {
    let contents: String = fs::read_to_string(file_path).unwrap();
    let lines: Vec<String> = contents.lines().map(|line| line.to_string()).collect();
    return lines
        .iter()
        .map(|line| line.split(";").map(|s| s.to_string()).collect())
        .collect();
}

fn convert_to_json(data: &Vec<Vec<String>>) -> String {
    let mut json: HashMap<String, Vec<String>> = HashMap::new();
    let headers: Vec<String> = data[0].clone();
    for i in 0..data.len() as u32 {
        for j in 0..data[i as usize].len() as u32 {
            if i == 0 {
                json.insert(data[i as usize][j as usize].to_string(), Vec::new());
            } else if data[i as usize][j as usize].len() == 0 {
                break;
            } else {
                json.entry(headers[j as usize].to_string())
                    .and_modify(|e| e.push(data[i as usize][j as usize].to_string()));
            }
        }
    }
    let mut final_string: String = String::from("[\n");
    for (key, value) in json.iter() {
        if key == data[0][0].as_str() {
            final_string.push_str(&format!("\t{{\n\t\t\"{}\": {:?}\n\t}}\n", key, value));
            break;
        }
        final_string.push_str(&format!("\t{{\n\t\t\"{}\": {:?}\n\t}},\n", key, value));
    }
    return final_string + "]";
}
