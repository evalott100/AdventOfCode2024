use std::collections::VecDeque;
use std::fmt;
use std::fs::read_to_string;
use std::time::Instant;

type FileBlocks = VecDeque<Option<usize>>;

pub fn add_block(blocks: &mut FileBlocks, id: Option<usize>) {
    blocks.push_back(id)
}

struct DisplayFileBlocks(FileBlocks);

impl fmt::Display for DisplayFileBlocks {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut fmt_string = String::new();
        for file in self.0.iter() {
            let mut id_char = '.';
            if file.is_some() {
                id_char = file.unwrap().to_string().chars().next().unwrap();
            }
            fmt_string.push(id_char);
        }
        write!(f, "{}", fmt_string)
    }
}

fn load_input(path: &str) -> FileBlocks {
    let mut file_blocks = FileBlocks::new();
    let raw_input: Vec<char> = read_to_string(path)
        .expect("Failed to read file")
        .chars()
        .collect();

    // I tried a fancy iterator here but it was annoying.
    for i in 0..(raw_input.len() / 2) {
        let index_skip_two = i * 2;
        let file_length = raw_input
            .get(index_skip_two)
            .unwrap_or(&' ')
            .to_digit(10)
            .unwrap_or(0);
        let space_length = raw_input
            .get(index_skip_two + 1)
            .unwrap_or(&' ')
            .to_digit(10)
            .unwrap_or(0);

        for _ in 0..file_length {
            file_blocks.push_back(Some(i));
        }
        for _ in 0..space_length {
            file_blocks.push_back(None);
        }
    }

    file_blocks
}

fn get_back_some_index(blocks: &FileBlocks) -> usize {
    for (index, id) in blocks.iter().enumerate().rev() {
        if id.is_some() {
            return index;
        }
    }
    panic!("No back index")
}

fn get_front_space(blocks: &FileBlocks) -> usize {
    for (index, id) in blocks.iter().enumerate() {
        if id.is_none() {
            return index;
        }
    }
    panic!("No free space")
}

fn checksum(file_blocks: FileBlocks) -> u64 {
    let mut sum: u64 = 0;
    for (index, id) in file_blocks.iter().enumerate() {
        if id.is_some() {
            sum += (index * id.unwrap()) as u64;
        }
    }
    sum
}

fn move_blocks_with_fragmentation(mut file_blocks: FileBlocks) -> FileBlocks {
    let mut front_space_index = get_front_space(&file_blocks);
    let mut back_some_index = get_back_some_index(&file_blocks);

    while front_space_index < back_some_index {
        file_blocks.swap(front_space_index, back_some_index);
        front_space_index = get_front_space(&file_blocks);
        back_some_index = get_back_some_index(&file_blocks);
    }

    file_blocks
}

#[derive(Copy, Clone)]
struct Chunk {
    pub id: Option<usize>,
    pub length: usize,
    pub active: bool,
}
impl fmt::Debug for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut fmt_string = String::new();
        let mut id_char = '.';
        if self.id.is_some() {
            id_char = self.id.unwrap().to_string().chars().next().unwrap();
        }
        for _ in 0..self.length {
            fmt_string.push(id_char);
        }
        write!(f, "[{}]", fmt_string)
    }
}

fn get_contiguous_chunks(file_blocks: &FileBlocks) -> VecDeque<Chunk> {
    let mut result = VecDeque::new();
    let mut chunk = Chunk {
        id: file_blocks[0],
        length: 0,
        active: true,
    };

    for file_block in file_blocks.iter() {
        if *file_block == chunk.id {
            chunk.length += 1;
        } else {
            result.push_back(chunk);
            chunk = Chunk {
                id: *file_block,
                length: 1,
                active: true,
            };
        }
    }
    result.push_back(chunk);
    result
}

fn get_back_chunk(chunks: &VecDeque<Chunk>) -> Option<usize> {
    for (index, chunk) in chunks.iter().enumerate().rev() {
        if chunk.id.is_some() && chunk.active {
            return Some(index);
        }
    }
    None
}

fn get_front_space_chunk_which_fits(
    chunks: &VecDeque<Chunk>,
    minimum_length: usize,
) -> Option<usize> {
    for (index, chunk) in chunks.iter().enumerate() {
        if chunk.id.is_none() && chunk.length >= minimum_length {
            return Some(index);
        }
    }
    None
}

fn move_blocks_contiguous(file_blocks: FileBlocks) -> FileBlocks {
    let mut chunks: VecDeque<Chunk> = get_contiguous_chunks(&file_blocks);

    loop {
        let maybe_back_chunk_index = get_back_chunk(&chunks);
        if maybe_back_chunk_index.is_none() {
            break;
        }
        let back_chunk_index = maybe_back_chunk_index.unwrap();
        let maybe_front_chunk_index =
            get_front_space_chunk_which_fits(&chunks, chunks[back_chunk_index].length);
        if maybe_front_chunk_index.is_none() {
            chunks[back_chunk_index].active = false;
            continue;
        }
        let mut front_chunk_index = maybe_front_chunk_index.unwrap();

        if front_chunk_index >= back_chunk_index {
            chunks[back_chunk_index].active = false;
            continue;
        }

        let mut front_chunk = chunks[front_chunk_index];
        let mut back_chunk = chunks[back_chunk_index];

        chunks[back_chunk_index] = Chunk {
            id: None,
            length: back_chunk.length,
            active: false,
        };

        back_chunk.active = false;

        chunks.insert(front_chunk_index, back_chunk);
        front_chunk_index += 1;
        front_chunk.length -= back_chunk.length;

        if front_chunk.length == 0 {
            chunks.remove(front_chunk_index);
        } else {
            chunks[front_chunk_index] = front_chunk
        }
    }

    let mut sorted_file_blocks: FileBlocks = VecDeque::new();

    for chunk in chunks.iter() {
        for _ in 0..chunk.length {
            sorted_file_blocks.push_back(chunk.id);
        }
    }
    sorted_file_blocks
}

fn solution_1(file_blocks: FileBlocks) -> u64 {
    checksum(move_blocks_with_fragmentation(file_blocks))
}
fn solution_2(file_blocks: FileBlocks) -> u64 {
    checksum(move_blocks_contiguous(file_blocks))
}

fn main() {
    let input_start = Instant::now();
    let file_blocks: FileBlocks = load_input("input.dat");
    println!("input took {:?}", input_start.elapsed());

    let solution_1_start = Instant::now();
    let output_1 = solution_1(file_blocks.clone());
    println!(
        "solution_1: {:?}, took {:?}",
        output_1,
        solution_1_start.elapsed()
    );

    let solution_2_start = Instant::now();
    let output_2 = solution_2(file_blocks.clone());
    println!(
        "solution_2: {:?}, took {:?}",
        output_2,
        solution_2_start.elapsed()
    );
}
