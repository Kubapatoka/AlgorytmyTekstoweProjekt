#[derive(Debug, PartialEq)]
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

struct SuffixArray {
    data_starts: Vec<usize>,
    data_end: usize,
    text: Vec<char>,
}

impl SuffixArray {
    fn move_data_end(&mut self) {
        self.data_end += 1;
        println!("move data end to {}", self.data_end);
    }

    fn is_greater(&self, p1: usize, p2: usize) -> bool {
        for i in 0..std::cmp::min(self.text.len() - p1, self.text.len() - p2) {
            if self.text[p1 + i] > self.text[p2 + i] {
                return true;
            }
            if self.text[p1 + i] < self.text[p2 + i] {
                return false;
            }
        }

        if p1 < p2 {
            return false;
        }
        return true;
    }

    fn binary_search_new_pos(&self, sp: usize) -> usize {
        let mut start = 0;
        let mut end = self.data_starts.len();

        if end == 0 {
            return 0;
        }

        while end > start + 1 {
            let mid = (start + end) / 2;

            if self.is_greater(self.data_starts[mid], sp) {
                end = mid;
            } else {
                start = mid;
            }
        }

        if self.is_greater(sp, self.data_starts[start]) {
            return end;
        }

        return start;
    }

    fn add(&mut self, start_pos: usize) {
        println!("adding new start pos to data starts: {}", start_pos);
        let pos = self.binary_search_new_pos(start_pos);
        println!("pos in data: {}", pos);
        self.data_starts.insert(pos, start_pos);
        println!("added new start pos to data starts: {}", start_pos);
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

        while e > s + 1 {
            let mid = (e + s) / 2;

            let pos = self.data_starts[mid] + p_l;

            println!(
                "find new range begin s: {} e: {} mid: {} pos: {}",
                s, e, mid, pos
            );

            if pos > self.data_end {
                s = mid + 1;
                continue;
            }

            if self.text[pos] >= letter_to_find {
                e = mid;
                continue;
            }

            if self.text[pos] < letter_to_find {
                s = mid + 1;
            }
        }

        if self.text[self.data_starts[s] + p_l] == letter_to_find {
            return s;
        }

        if self.text[self.data_starts[e] + p_l] == letter_to_find {
            return e;
        }

        return self.data_starts.len() + 1;
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

        if s > self.data_starts.len() {
            return 0;
        }

        while e > s + 1 {
            let mid = (e + s) / 2;

            let pos = self.data_starts[mid] + p_l;

            println!(
                "find new range end s: {} e: {} mid: {} pos: {}",
                s, e, mid, pos
            );

            if pos > self.data_end {
                s = mid + 1;
                continue;
            }

            if self.text[pos] > letter_to_find {
                e = mid - 1;
                continue;
            }

            if self.text[pos] <= letter_to_find {
                s = mid;
            }
        }

        if self.text[self.data_starts[e] + p_l] == letter_to_find {
            return e;
        }

        if self.text[self.data_starts[s] + p_l] == letter_to_find {
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

        if start + p_l >= self.text.len() {
            // return incorrect range
            return (e_idx_sa + 1, e_idx_sa);
        }

        let letter_to_find = self.text[start + p_l];

        println!(
            "find in range start: {}, patt_len: {}, pocz: {}, kon: {}, letter: {}",
            start, pattern_len, s_idx_sa, e_idx_sa, letter_to_find
        );

        // Step 1: move beginning of range
        let new_start = self.find_new_range_begin(s_idx_sa, e_idx_sa, p_l, letter_to_find);
        let new_end = self.find_new_range_end(new_start, e_idx_sa, p_l, letter_to_find);

        return (new_start, new_end);
    }
}

struct LZ77 {
    suff_arr: SuffixArray,
}

impl LZ77 {
    fn find_longest_common_fragment(&mut self, start: usize) -> (usize, usize) {
        // text|pattern

        for pos in &self.suff_arr.data_starts {
            println!("data_starts: {}", pos);
        }

        if self.suff_arr.data_starts.len() == 0 {
            self.suff_arr.add(start);
            self.suff_arr.move_data_end();
            return (0, 0);
        }

        let mut s_range = 0;
        let mut e_range = self.suff_arr.data_starts.len() - 1;
        println!(
            "start find longest common fragment {}, starting range: {}-{}",
            start, s_range, e_range
        );

        for i in 0..(self.suff_arr.text.len() - start + 1) {
            let n_r = self.suff_arr.find_in_range(start, i + 1, s_range, e_range);
            println!("new range: {}-{}", n_r.0, n_r.1);

            if n_r.1 < n_r.0 {
                self.suff_arr.move_data_end();
                let pocz = self.suff_arr.data_starts[s_range];
                for j in 0..=i {
                    self.suff_arr.add(start + j);
                }
                if i == 0 {
                    return (0, 0);
                }
                return (pocz, i);
            }
            self.suff_arr.move_data_end();

            s_range = n_r.0;
            e_range = n_r.1;
        }

        return (0, 0);
    }

    fn generate_block(&mut self, start: usize) -> Block {
        let mut res = self.find_longest_common_fragment(start);

        if start + res.1 == self.suff_arr.text.len() {
            res.1 -= 1;
        }

        if res.1 == 0 {
            return Block {
                start: 0,
                end: 0,
                letter: self.suff_arr.text[start],
            };
        }

        return Block {
            start: res.0 + 1,
            end: res.0 + res.1,
            letter: self.suff_arr.text[start + res.1],
        };
    }

    fn lz_77(&mut self) -> Vec<Block> {
        let mut vector_of_blocks: Vec<Block> = vec![];
        let mut actual_end = 0;

        let text_len = self.suff_arr.text.len();

        while actual_end != text_len {
            vector_of_blocks.push(self.generate_block(actual_end));

            let block = vector_of_blocks.last().unwrap();
            let block_len = block.length();
            println!("");
            println!(
                "New block: {} {} {} {}",
                block.start, block.end, block.letter, block_len
            );
            println!("");

            actual_end += block_len;
        }

        return vector_of_blocks;
    }
}

fn main() {
    println!("Hello, world!");

    let text = "aaababc";

    //let text = "aaabababababababbbabbabbabbaba";
    println!("text len: {}", text.len());

    let mut compress = LZ77 {
        suff_arr: SuffixArray {
            data_starts: vec![],
            data_end: 0,
            text: text.to_string().chars().collect::<Vec<_>>(),
        },
    };

    let v = compress.lz_77();

    for b in v {
        println!("{}, {}, {}", b.start, b.end, b.letter);
    }
}

#[cfg(test)]
mod tests {
    use crate::Block;
    use crate::SuffixArray;
    use crate::LZ77;

    #[test]
    fn short_text_1() {
        let text = "a";
        let mut compress = LZ77 {
            suff_arr: SuffixArray {
                data_starts: vec![],
                data_end: 0,
                text: text.to_string().chars().collect::<Vec<_>>(),
            },
        };

        let v = compress.lz_77();
        assert_eq!(
            v,
            vec![Block {
                start: 0,
                end: 0,
                letter: 'a'
            }]
        );
    }

    #[test]
    fn short_text_2() {
        let text = "ab";
        let mut compress = LZ77 {
            suff_arr: SuffixArray {
                data_starts: vec![],
                data_end: 0,
                text: text.to_string().chars().collect::<Vec<_>>(),
            },
        };

        let v = compress.lz_77();
        assert_eq!(
            v,
            vec![
                Block {
                    start: 0,
                    end: 0,
                    letter: 'a'
                },
                Block {
                    start: 0,
                    end: 0,
                    letter: 'b'
                }
            ]
        );
    }

    #[test]
    fn short_text_3() {
        let text = "";
        let mut compress = LZ77 {
            suff_arr: SuffixArray {
                data_starts: vec![],
                data_end: 0,
                text: text.to_string().chars().collect::<Vec<_>>(),
            },
        };

        let v = compress.lz_77();
        assert_eq!(v, vec![]);
    }

    #[test]
    fn short_text_4() {
        let text = "aaababc";
        let mut compress = LZ77 {
            suff_arr: SuffixArray {
                data_starts: vec![],
                data_end: 0,
                text: text.to_string().chars().collect::<Vec<_>>(),
            },
        };

        let v = compress.lz_77();
        assert_eq!(
            v,
            vec![
                Block {
                    start: 0,
                    end: 0,
                    letter: 'a'
                },
                Block {
                    start: 1,
                    end: 2,
                    letter: 'b'
                },
                Block {
                    start: 3,
                    end: 4,
                    letter: 'c'
                }
            ]
        );
    }

    #[test]
    fn short_text_5() {
        let text = "banana";
        let mut compress = LZ77 {
            suff_arr: SuffixArray {
                data_starts: vec![],
                data_end: 0,
                text: text.to_string().chars().collect::<Vec<_>>(),
            },
        };

        let v = compress.lz_77();
        assert_eq!(
            v,
            vec![
                Block {
                    start: 0,
                    end: 0,
                    letter: 'b'
                },
                Block {
                    start: 0,
                    end: 0,
                    letter: 'a'
                },
                Block {
                    start: 0,
                    end: 0,
                    letter: 'n'
                },
                Block {
                    start: 2,
                    end: 3,
                    letter: 'a'
                }
            ]
        );
    }

    #[test]
    fn short_text_6() {
        let text = "bananas";
        let mut compress = LZ77 {
            suff_arr: SuffixArray {
                data_starts: vec![],
                data_end: 0,
                text: text.to_string().chars().collect::<Vec<_>>(),
            },
        };

        let v = compress.lz_77();
        assert_eq!(
            v,
            vec![
                Block {
                    start: 0,
                    end: 0,
                    letter: 'b'
                },
                Block {
                    start: 0,
                    end: 0,
                    letter: 'a'
                },
                Block {
                    start: 0,
                    end: 0,
                    letter: 'n'
                },
                Block {
                    start: 2,
                    end: 4,
                    letter: 's'
                }
            ]
        );
    }

    #[test]
    fn short_text_7() {
        let text = "missisipi";
        let mut compress = LZ77 {
            suff_arr: SuffixArray {
                data_starts: vec![],
                data_end: 0,
                text: text.to_string().chars().collect::<Vec<_>>(),
            },
        };

        let v = compress.lz_77();
        assert_eq!(
            v,
            vec![
                Block {
                    start: 0,
                    end: 0,
                    letter: 'm'
                },
                Block {
                    start: 0,
                    end: 0,
                    letter: 'i'
                },
                Block {
                    start: 0,
                    end: 0,
                    letter: 's'
                },
                Block {
                    start: 3,
                    end: 3,
                    letter: 'i'
                },
                Block {
                    start: 4,
                    end: 5,
                    letter: 'p'
                },
                Block {
                    start: 0,
                    end: 0,
                    letter: 'i'
                },
            ]
        );
    }
}
