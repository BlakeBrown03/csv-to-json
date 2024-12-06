use std::fs;

fn main() {
    println!("JSON: {:?}", convert_to_json(&read_file("./username.csv")));
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
    let mut json: String = "{".to_string();
    let length: u32 = data.len() as u32;
    for i in 0..length {
        for j in 0..data[i as usize].len() as u32 {
            if i == 0 {
                json.push_str(&format!("{}:", data[i as usize][j as usize]));
            } else {
                json.push_str(&format!("{} :", data[i as usize][j as usize]));
            }
        }
    }
    return json.to_string();
}
