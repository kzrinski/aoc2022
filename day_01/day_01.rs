use std::fs::File;
use std::path::Path;
use std::io::{prelude::*, self};

//get rid of compiler warning about main not being used
#[allow(dead_code)]
fn main() {
    //create path to text file
    let path = Path::new("day_01.txt");

    if let Ok(lines) = read_lines(path){
        let mut num_elves: i32 = 0;
        for line in lines 
        {
            if let Ok(read) = line 
            {
                if read.eq("")
                {
                    num_elves += 1;
                }
            }
        }

        let mut calories_per_elf: Vec<Vec <i32>> = Vec::new();

        if let Ok(lines) = read_lines(path){
            let mut list_of_cals: Vec<i32> = Vec::new();
            for line in lines
            {

                if let Ok(read) = line
                {
                    if read.eq("")
                    {
                        calories_per_elf.push(list_of_cals);
                        list_of_cals = Vec::new();
                    }
                    else {
                        let num = read.parse::<i32>().unwrap();
                        list_of_cals.push( num );
                    }
                }
            }

            println!("Number of elves: {}\n",num_elves);

            let mut count: i32 = 1;
            let mut index: i32 = 0;

            let mut max_cal: i32 = 0;
            let mut max_cal_elf_id = 0;

            for item in calories_per_elf
            {
                let mut sum_of_cals: i32 = 0;
                for i in &item[0 .. item.len()]{
                    sum_of_cals += i;
                }

                if sum_of_cals > max_cal 
                {
                    max_cal = sum_of_cals;
                    max_cal_elf_id = count;
                }

                print!("Elf #{} has {:?} for a total of: {}\n", count,item,sum_of_cals);

                count += 1;
                index += 1;
            }

            print!("The elf with the most calories is #{} with {}",max_cal_elf_id,max_cal);
        }
    }
    
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}