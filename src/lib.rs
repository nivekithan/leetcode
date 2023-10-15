#![allow(dead_code)]

struct Solution {}

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let num_rows = num_rows as usize;

        let mut output: Vec<Vec<i32>> = Vec::new();

        for row_length in 1..(num_rows + 1) {
            if row_length == 1 {
                output.push(vec![1]);
                continue;
            }

            let last_vector = output.last().unwrap();

            let mut new_vector: Vec<i32> = Vec::new();
            for i in 0..row_length {
                if i == 0 {
                    new_vector.push(1);
                    continue;
                }

                if i == row_length - 1 {
                    new_vector.push(1);
                    continue;
                }

                let value = last_vector[i - 1] + last_vector[i];
                new_vector.push(value);
            }
            output.push(new_vector);
        }

        return output;
    }
}
