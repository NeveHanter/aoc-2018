use crate::consts::COMMON_FILESYSTEM_BLOCK_SIZE;

use std::cmp::min;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Seek, SeekFrom};

fn part_one() {
    let file = File::open("input/day01.txt").expect("Failed to open the file");

    let mut reader = BufReader::new(file);

    let mut buffer = String::new();
    let mut counter: i32 = 0;
    loop {
        reader
            .read_line(&mut buffer)
            .expect("Failed reading the line from the file");

        {
            let buffer = buffer.trim();

            if buffer.is_empty() {
                break;
            }

            counter += buffer
                .parse::<i32>()
                .expect(format!("Failed parsing integer from {} buffer", buffer).as_str());
        }

        buffer.clear();
    }

    println!("01_1: Counted {}.", counter);
}

fn part_two() {
    let file = File::open("input/day01.txt").expect("Failed to open the file");

    let buffer_size = min(
        file.metadata()
            .expect("Failed getting the file metadata")
            .len(),
        COMMON_FILESYSTEM_BLOCK_SIZE,
    ) as usize;

    let mut reader = BufReader::with_capacity(buffer_size, file);

    let mut buffer = String::new();
    let mut counter: i32 = 0;
    let mut frequencies = HashMap::new();
    loop {
        reader
            .read_line(&mut buffer)
            .expect("Failed reading the line from the file");

        {
            let buffer = buffer.trim();

            if buffer.is_empty() {
                reader
                    .seek(SeekFrom::Start(0))
                    .expect("Failed seeking the buffer.");

                continue;
            }

            counter += buffer
                .parse::<i32>()
                .expect(format!("Failed parsing integer from {} buffer", buffer).as_str());

            let entry_value = frequencies.entry(counter).or_insert(0);

            if *entry_value == 1 {
                println!("01_2: {} counter value appears twice first.", counter);

                return;
            }

            *entry_value += 1;
        }

        buffer.clear();
    }
}

pub fn run() {
    part_one();
    part_two();
}
