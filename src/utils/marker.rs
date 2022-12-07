use rustc_hash::FxHashSet;
use std::collections::VecDeque;

#[inline]
pub fn find_marker(message: &str, m_size: usize) -> Option<u32> {
    let size = message.len(); //Assume all chars are alphabetic standard so this is actually nº of chars

    for c in 0..(size - m_size) {
        let curr_seg = &message[c..c + m_size];
        let mut uniq = FxHashSet::default();
        let all_state = curr_seg.chars().all(|e| uniq.insert(e));
        if all_state {
            return Some((c + m_size) as u32);
        }
    }
    None
}
#[inline]
pub fn find_marker_old(message: &str, m_size: usize) -> Option<u32> {
    let mut buffer = VecDeque::<char>::new();
    let chars = message.chars().collect::<Vec<char>>();

    for (i, c) in chars.iter().enumerate() {
        buffer.push_back(*c);
        if buffer.len() > m_size {
            buffer.pop_front();
            let mut u = FxHashSet::default();
            let all_state = buffer.iter().all(|e| u.insert(e));
            if all_state {
                return Some((i as u32) + 1);
            }
        }
    }
    None
}
