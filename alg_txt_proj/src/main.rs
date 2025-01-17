struct Block {
    start: u64,
    end: u64,
    letter: Char,
}

impl Block {
    fn length() {
        if start == 0 && end == 0 {
            return 1;
        }

        return end - start + 1 + 1;
    }
}

struct SuffixArray {
    data_starts: Vec<u32>,
    data_end: u32,
    text: str,
}

impl SuffixArray {
    fn add_letter() {
        data_end += 1;
        add(data_end);
    }

    fn is_greater(p1: u32, p2: u32) {
        for i in range(0, min(data_end - p1, data_end - p2)) {
            if text[p1 + i] > text[p2 + i] {
                return true;
            }
            if text[p1 + i] < text[p2 + i] {
                return false;
            }
        }

        if p1 < p2 {
            return false;
        }
        return true;
    }

    fn binary_search_new_pos(sp: u32) {
        let mut start = 0;
        let mut end = data_starts.len();

        while start != end {
            let mid = (start + end) / 2;

            if is_greater(data_starts[mid], sp) {
                end = mid;
            } else {
                start = mid;
            }
        }

        return start;
    }

    fn add(start_pos: u32) {
        let pos = data_starts.binary_search_new_pos(start_pos);
        data_starts.insert(pos, start_pos)
    }
}

struct LZ77 {
    suff_arr: SuffixArray,
    end_pos: u32,
}

impl LZ77 {
    fn find_longest_common_fragment(start: i32) {
        
    }

    fn generate_block(start: i32) -> Block {
        let end = find_longest_common_fragment(start);

        if end == len(text) {
            end -= 1;
        }

        return Block {
            start: start,
            end: end,
            letter: suff_arr.text[end],
        };
    }

    fn lz_77(text: &str) {
        suff_arr = SuffixArray {
            data_starts: [],
            data_end: 0,
            text: text,
        };

        let mut vector_of_blocks: Vec<Block>;
        let mut actual_end = 0;

        let text_len = len(text);

        while actual_end != text_len {
            vector_of_blocks.push(generate_block(actual_end));

            let block_len = vector_of_blocks.last().unwrap().length();

            actual_end += block_len;
        }

        return vector_of_blocks;
    }
}

fn main() {
    println!("Hello, world!");

    let text = "aaabababababababbbabbabbabbaba";
}
