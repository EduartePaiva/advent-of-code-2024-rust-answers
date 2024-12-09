#[derive(Debug)]
struct ChunkFile {
    empty_space: u8,
    blocks: Vec<usize>,
}

pub fn question09(input: &str) -> i64 {
    // the id can be more than 1 digit wide
    let mut unpacked: Vec<ChunkFile> = input
        .as_bytes()
        .chunks(2)
        .enumerate()
        .map(|(id, chk)| {
            let num_blocks = chk[0] - b'0';
            let empty_space = if chk.len() < 2 { 0 } else { chk[1] - b'0' };
            let blocks: Vec<usize> = (0..num_blocks).map(|_| id).collect();
            ChunkFile {
                empty_space,
                blocks,
            }
        })
        .collect();
    let mut i = 0;
    while i < unpacked.len() {
        if unpacked[i].empty_space == 0 {
            i += 1;
            continue;
        }
        if unpacked.last().unwrap().blocks.len() == 0 {
            unpacked.pop();
            continue;
        }
        unpacked[i].empty_space -= 1;
        let lst_idx = unpacked.len() - 1;
        let last_id = unpacked[lst_idx].blocks.pop().unwrap();
        unpacked[i].blocks.push(last_id);
    }

    unpacked
        .into_iter()
        .map(|v| v.blocks.into_iter())
        .flatten()
        .enumerate()
        .map(|(pos, id)| pos as i64 * id as i64)
        .sum()

    // math the checksum
}

#[derive(Debug)]
struct Interval {
    id: usize,
    start: usize,
    end: usize,
}
pub fn question09_part_2(input: &str) -> i64 {
    let mut ending = 0;
    let mut intervals: Vec<Interval> = input
        .as_bytes()
        .chunks(2)
        .enumerate()
        .map(|(id, chk)| {
            let num_blocks = chk[0] - b'0';
            let empty_space = if chk.len() < 2 { 0 } else { chk[1] - b'0' };
            let start = ending;
            let end = ending + num_blocks as usize;
            ending += (num_blocks + empty_space) as usize;
            Interval { id, start, end }
        })
        .collect();

    let mut end = intervals.len() - 1;
    while end > 0 {
        let needed_space = intervals[end].end - intervals[end].start;
        let mut switched = false;
        for i in 0..end {
            if intervals[i + 1].start - intervals[i].end >= needed_space {
                let mut removed_int = intervals.remove(end);
                removed_int.start = intervals[i].end;
                removed_int.end = intervals[i].end + needed_space;
                intervals.insert(i + 1, removed_int);
                switched = true;
                break;
            }
        }
        if !switched {
            end -= 1;
        }
    }
    intervals
        .into_iter()
        .map(|int| {
            (int.start..int.end)
                .map(|pos| pos as i64 * int.id as i64)
                .sum::<i64>()
        })
        .sum()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(question09("2333133121414131402"), 1928);
    }
    #[test]
    fn test2() {
        assert_eq!(question09_part_2("2333133121414131402"), 2858);
    }
}
