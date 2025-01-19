struct Block {
    start: usize,
    end: usize,
    letter: char,
}

impl Block {
    fn length(&self) -> usize {
        if self.start == 0 && self.end == 0 {
            return 1;
        }

        return self.end - self.start + 1 + 1;
    }
}

struct SuffixArray  {
    data_starts: Vec<usize>,
    data_end: usize,
    text: String,
}

impl SuffixArray {
    fn add_letter(&mut self) {
        self.data_end += 1;
        self.add(self.data_end);
    }

    fn is_greater(&self, p1: usize, p2: usize) -> bool {
        for i in 0..std::cmp::min(self.data_end - p1, self.data_end - p2) {
            if self.text.chars().nth(p1 + i).unwrap() > self.text.chars().nth(p2 + i).unwrap() {
                return true;
            }
            if self.text.chars().nth(p1 + i).unwrap() < self.text.chars().nth(p2 + i).unwrap() {
                return false;
            }
        }

        if p1 < p2 {
            return false;
        }
        return true;
    }

    // fragment p1..data_end begins p2..p2+p2_len
    fn begins(&self, p1: usize, p2: usize, p2_len: usize) -> bool {
        for i in 0..std::cmp::min(self.data_end - p1, p2_len) {
            if self.text.chars().nth(p1 + i).unwrap() != self.text.chars().nth(p2 + i).unwrap() {
                return false;
            }
        }
        return true;
    }

    fn binary_search_new_pos(&self, sp: usize) -> usize {
        let mut start = 0;
        let mut end = self.data_starts.len();

        while start != end {
            let mid = (start + end) / 2;

            if self.is_greater(self.data_starts[mid], sp) {
                end = mid;
            } else {
                start = mid;
            }
        }

        return start;
    }

    fn add(&mut self, start_pos: usize) {
        let pos = self.binary_search_new_pos(start_pos);
        self.data_starts.insert(pos, start_pos)
    }

    fn find_new_range_begin(
        &self,
        s_idx_sa: usize,
        e_idx_sa: usize,
        p_l: usize,
        letter_to_find: char,
    ) -> usize {
        let mut s = s_idx_sa;
        let mut e = e_idx_sa;

        while e - s < 1 {
            let mid = e + s / 2;

            let pos = self.data_starts[mid] + p_l;

            if pos > self.data_end {
                s = mid + 1;
                continue;
            }

            if self.text.chars().nth(pos).unwrap() >= letter_to_find {
                e = mid;
                continue;
            }

            if self.text.chars().nth(pos).unwrap() < letter_to_find {
                s = mid + 1;
            }
        }

        if self.text.chars().nth(self.data_starts[s] + p_l).unwrap() == letter_to_find {
            return s;
        }

        if self.text.chars().nth(self.data_starts[e] + p_l).unwrap() == letter_to_find {
            return e;
        }

        return 0;
    }

    fn find_new_range_end(
        &self,
        s_idx_sa: usize,
        e_idx_sa: usize,
        p_l: usize,
        letter_to_find: char,
    ) -> usize {
        let mut s = s_idx_sa;
        let mut e = e_idx_sa;

        while e - s < 1 {
            let mid = e + s / 2;

            let pos = self.data_starts[mid] + p_l;

            if pos > self.data_end {
                s = mid + 1;
                continue;
            }

            if self.text.chars().nth(pos).unwrap() > letter_to_find {
                e = mid - 1;
                continue;
            }

            if self.text.chars().nth(pos).unwrap() <= letter_to_find {
                s = mid;
            }
        }

        if self.text.chars().nth(self.data_starts[e] + p_l).unwrap() == letter_to_find {
            return e;
        }

        if self.text.chars().nth(self.data_starts[s] + p_l).unwrap() == letter_to_find {
            return s;
        }

        return 0;
    }

    fn find_in_range(
        &self,
        start: usize,
        pattern_len: usize,
        s_idx_sa: usize,
        e_idx_sa: usize,
    ) -> (usize, usize) {
        // given range meet the following condition:
        // All suffixes in this range have prefix text[start..pattern_len-1]
        // So we need to check only last letter
        let p_l = pattern_len - 1;

        let letter_to_find = self.text.chars().nth(start + p_l).unwrap();

        // Step 1: move beginning of range
        let new_start = self.find_new_range_begin(s_idx_sa, e_idx_sa, p_l, letter_to_find);
        let new_end = self.find_new_range_end(new_start, e_idx_sa, p_l, letter_to_find);

        return (new_start, new_end);
    }
}

struct LZ77  {
    suff_arr: SuffixArray ,
}

impl LZ77{
    fn find_longest_common_fragment(&self, start: usize) -> usize {
        // text|pattern

        let mut s_range = 0;
        let mut e_range = self.suff_arr.data_starts.len();

        for i in 0..(self.suff_arr.text.len() - start + 1) {
            let n_r = self.suff_arr.find_in_range(start, 1, s_range, e_range);

            if n_r.1 - n_r.0 < 1 {
                return i;
            }

            s_range = n_r.0;
            e_range = n_r.1;
        }

        return 0;
    }

    fn generate_block(&self, start: usize) -> Block {
        let mut end = self.find_longest_common_fragment(start);

        if end == self.suff_arr.text.len() {
            end -= 1;
        }

        return Block {
            start: start,
            end: end,
            letter: self.suff_arr.text.chars().nth(end).unwrap(),
        };
    }

    fn lz_77(&self) -> Vec<Block> {
        let mut vector_of_blocks: Vec<Block> = vec![];
        let mut actual_end = 0;

        let text_len = self.suff_arr.text.len();

        while actual_end != text_len {
            vector_of_blocks.push(self.generate_block(actual_end));

            let block_len = vector_of_blocks.last().unwrap().length();

            actual_end += block_len;
        }

        return vector_of_blocks;
    }
}

fn main() {
    println!("Hello, world!");

    let text = "aaabababababababbbabbabbabbaba";

    let compress = LZ77 {
        suff_arr: SuffixArray {
            data_starts: vec![],
            data_end: 0,
            text: text.to_string(),
        },
    };

    let v = compress.lz_77();

    for b in v{
        println!("{}, {}, {}", b.start, b.end, b.letter);
    }
}
