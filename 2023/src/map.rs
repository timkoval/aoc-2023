pub fn solve() -> u32 {
    let input = include_bytes!("../inputs/map.txt");
    let split = input.iter().position(|&c| c == b'\n').unwrap();

    let mut map = [0u32; 0b11001_11001_11001 + 1];
    let encode = |n: &[u8]| {
        ((n[0] - b'A') as u32) << 10 | ((n[1] - b'A') as u32) << 5 | (n[2] - b'A') as u32
    };
    input[split + 2..input.len() - 1]
        .split(|&c| c == b'\n')
        .for_each(|node| {
            map[encode(&node[0..3]) as usize] = encode(&node[7..10]) | encode(&node[12..15]) << 16;
        });

    let answer = input[0..split]
        .iter()
        .cycle()
        .scan(encode(b"AAA"), |node, step| {
            *node = if step == &b'L' {
                map[*node as usize] & u16::MAX as u32
            } else {
                map[*node as usize] >> 16
            };
            Some(*node & 0b11111 == (b'Z' - b'A') as u32)
        })
        .position(|node| node)
        .unwrap()
        + 1;
    answer as u32
}

pub fn solve_part2() -> usize {
    let input = include_bytes!("../inputs/map.txt");
    let split = input.iter().position(|&c| c == b'\n').unwrap();

    let mut map = [0u32; 0b11001_11001_11001 + 1];
    let mut starts: Vec<_> = Vec::with_capacity(6);
    let encode = |n: &[u8]| {
        ((n[0] - b'A') as u32) << 10 | ((n[1] - b'A') as u32) << 5 | (n[2] - b'A') as u32
    };
    input[split + 2..input.len() - 1]
        .split(|&c| c == b'\n')
        .for_each(|node| {
            map[encode(&node[0..3]) as usize] = encode(&node[7..10]) | encode(&node[12..15]) << 16;
            if node[2] == b'A' {
                starts.push(encode(&node[0..3]));
            }
        });

    let answer = starts
        .into_iter()
        .map(|node| {
            input[0..split]
                .iter()
                .cycle()
                .scan(node, |node, step| {
                    *node = if step == &b'L' {
                        map[*node as usize] & u16::MAX as u32
                    } else {
                        map[*node as usize] >> 16
                    };
                    Some(*node & 0b11111 == (b'Z' - b'A') as u32)
                })
                .position(|node| node)
                .unwrap()
                + 1
        })
        .fold(1, num_integer::lcm);
    answer
}
