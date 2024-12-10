use std::{collections::HashMap, fs, io};

fn main() {
    if fs::metadata("./your-json.json").is_ok() {
        fs::remove_file("./your-json.json").unwrap();
    }
    println!("Please enter the file path for the CSV file you want to convert to JSON");
    let mut file_path: String = String::new();
    io::stdin()
        .read_line(&mut file_path)
        .expect("Error reading file path");
    let data: String = convert_to_json(&read_file(&file_path.trim()));
    fs::write("./your-json.json", data).unwrap();
}

fn read_file(file_path: &str) -> Vec<Vec<String>> {
    let contents: String = fs::read_to_string(file_path).unwrap();
    let lines: Vec<String> = contents.lines().map(|line| line.to_string()).collect();
    return lines
        .iter()
        .map(|line| {
            let mut current = String::new();
            let mut result = Vec::new();
            let mut in_quotes: bool = false;
            for c in line.chars() {
                if c == '"' {
                    in_quotes = !in_quotes;
                    continue;
                }
                if c == ',' && !in_quotes {
                    result.push(current.clone());
                    current.clear();
                } else {
                    if c == '\u{00A0}' || c == '\u{00AD}' {
                        current.push(' ');
                    } else {
                        current.push(c);
                    }
                }
            }
            result.push(current);
            return result;
        })
        .collect();
}

fn convert_to_json(data: &Vec<Vec<String>>) -> String {
    let mut json: HashMap<String, HashMap<String, String>> = HashMap::new();
    let headers: Vec<String> = data[0].clone();
    // println!("{:?}", data);
    for i in 1..data.len() as u32 {
        let mut inside: HashMap<String, String> = HashMap::new();
        json.insert(data[i as usize][0].clone(), HashMap::new());
        for j in 1..headers.len() as u32 {
            inside.insert(
                headers[j as usize].clone(),
                data[i as usize][j as usize].clone(),
            );
            json.insert(data[i as usize][0].clone(), inside.clone());
        }
    }
    let mut final_string: String = String::from("[\n");
    for (key, value) in json.iter() {
        if key == json.keys().last().unwrap() {
            final_string.push_str(&format!("\t{{\n\t\t\"{}\": {:?}\n\t}}\n", key, value));
            continue;
        }
        final_string.push_str(&format!("\t{{\n\t\t\"{}\": {:?}\n\t}},\n", key, value));
    }
    return final_string + "]";
}
