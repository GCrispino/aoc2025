use std::error::Error;
use std::fmt::Display;
use std::fs;

pub mod types;

fn get_input_path(rel_path: String) -> Result<String, Box<dyn Error>> {
    let cur_dir_blah = std::env::current_dir()?;
    let cur_dir_opt = cur_dir_blah.to_str();

    let cur_dir_res: Result<&str, Box<dyn Error>> = match cur_dir_opt {
        Some(s) => Ok(s),
        None => Err("".into()),
    };

    let cur_dir = cur_dir_res?;

    Ok(format!("{}/src/days/{}", cur_dir, rel_path))
}

pub fn read_from_file_to_string(rel_path: String) -> Result<String, Box<dyn Error>> {

    let input_path = get_input_path(rel_path)?;

    let res = fs::read_to_string(input_path)?.trim().to_string();

    Ok(res)
}

pub fn read_from_file_to_string_no_trim(rel_path: String) -> Result<String, Box<dyn Error>> {
    let input_path = get_input_path(rel_path)?;

    let res = fs::read_to_string(input_path)?.to_string();

    Ok(res)
}

pub fn read_from_file_to_number_tuples(
    rel_path: String,
) -> Result<Vec<(i32, i32)>, Box<dyn Error>> {
    let file_contents = read_from_file_to_string(rel_path)?;
    let spl = file_contents.split("\n");
    let n = spl.clone().count();
    let parsed: Result<Vec<(i32, i32)>, Box<dyn Error>> = spl
        .take(n)
        .map(|s| {
            let v: Vec<&str> = s.split_whitespace().collect();

            let x = v[0].parse::<i32>()?;
            let y = v[1].parse::<i32>()?;

            Ok((x, y))
        })
        .collect();

    parsed
}

pub fn read_from_file_to_string_list(rel_path: String) -> Result<Vec<String>, Box<dyn Error>> {
    let file_contents = read_from_file_to_string(rel_path)?;
    let spl = file_contents.split("\n");
    let n = spl.clone().count();
    let parsed: Vec<String> = spl
        .take(n)
        .map(|s| {
            let v: String = s.split_whitespace().map(|x| x.to_string()).collect();
            v
        })
        .collect();

    Ok(parsed)
}

pub fn read_from_file_to_number_matrix(rel_path: String) -> Result<Vec<Vec<i32>>, Box<dyn Error>> {
    let file_contents = read_from_file_to_string(rel_path)?;
    let spl = file_contents.split("\n");
    let n = spl.clone().count();
    let parsed: Result<Vec<Vec<i32>>, Box<dyn Error>> = spl
        .take(n - 1)
        .map(|s| {
            let v: Result<Vec<i32>, Box<dyn Error>> = s
                .split_whitespace()
                .map(|x| Ok(x.parse::<i32>()?))
                .collect();

            v
        })
        .collect();

    parsed
}

pub fn transpose_matrix<T: Copy>(input: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    let mut result: Vec<Vec<T>> = vec!();

    let n_rows = input.len();
    let n_cols = input[0].len();

    for j in 0..n_cols {
        let mut column: Vec<T> = vec!();

        for i in 0..n_rows {
            column.push(input[i][j]);
        }

        result.push(column);
    }

    result
}

pub fn print_matrix<T: Display>(input: &Vec<Vec<T>>) {
    let n_rows = input.len();
    let n_cols = input[0].len();

    for i in 0..n_rows {
        for j in 0..n_cols {
            print!("{:}", input[i][j]);
        }

        println!("");
    }

}
