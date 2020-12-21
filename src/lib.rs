#![feature(test)]
#![feature(array_chunks)]
extern crate test;

use byteorder::ReadBytesExt;
use byteorder::LittleEndian;
use std::io::Cursor;
use std::io;

#[inline(never)]
pub fn sum_baseline_chunks_loop(data: &[u8]) -> u64 {
    let mut ret: u64 = 0;
    for arr in data.array_chunks::<8>() {
        ret += arr[0] as u64;
        ret += arr[1] as u64;
        ret += u16::from_le_bytes([arr[2], arr[3]]) as u64;
        ret += u32::from_le_bytes([arr[4], arr[5], arr[6], arr[7]]) as u64;
    }
    ret
}
#[inline(never)]
pub fn sum_baseline_naive_loop(data: &[u8]) -> u64 {
    let mut ret: u64 = 0;
    for i in 0..data.len()/8 {
        ret += data[i*8] as u64;
        ret += data[i*8+1] as u64;
        ret += u16::from_le_bytes([data[i*8+2 as usize], data[i*8+3 as usize]]) as u64;
        ret += u32::from_le_bytes([
            data[i*8+4 as usize],
            data[i*8+5 as usize],
            data[i*8+6 as usize],
            data[i*8+7 as usize],
        ]) as u64;
    }
    ret
}
#[inline(never)]
pub fn sum_baseline_naive_loop_v2(data: &[u8]) -> u64 {
    let mut ret: u64 = 0;
    for i in 0..data.len()/8 {
        let arr = &data[i*8..i*8+8];
        ret += arr[0] as u64;
        ret += arr[1] as u64;
        ret += u16::from_le_bytes([arr[2], arr[3]]) as u64;
        ret += u32::from_le_bytes([
            arr[4],
            arr[5],
            arr[6],
            arr[7],
        ]) as u64;
    }
    ret
}
#[inline(never)]
pub fn sum_byteorder_slice_impl(data: &mut &[u8]) -> io::Result<u64> {
    let mut ret = 0;
    ret += data.read_u8()? as u64;
    ret += data.read_u8()? as u64;
    ret += data.read_u16::<LittleEndian>()? as u64;
    ret += data.read_u32::<LittleEndian>()? as u64;
    Ok(ret)
}
#[inline(never)]
pub fn sum_byteorder_slice(data: &[u8]) -> io::Result<u64> {
    let mut data = data;
    let mut ret: u64 = 0;
    for _ in 0..data.len()/8 {
        ret += sum_byteorder_slice_impl(&mut data)?;
    }
    Ok(ret)
}

#[inline(never)]
pub fn sum_byteorder_slice_impl_v2(data: &mut &[u8]) -> io::Result<u64> {
    let mut ret = 0;
    if data.len() < 1 {
        return Err(io::Error::new(io::ErrorKind::Other, "failed to fill whole buffer"));
    }
    ret += data.read_u8()? as u64;
    if data.len() < 1 {
        return Err(io::Error::new(io::ErrorKind::Other, "failed to fill whole buffer"));
    }
    ret += data.read_u8()? as u64;
    if data.len() < 2 {
        return Err(io::Error::new(io::ErrorKind::Other, "failed to fill whole buffer"));
    }
    ret += data.read_u16::<LittleEndian>()? as u64;
    if data.len() < 4 {
        return Err(io::Error::new(io::ErrorKind::Other, "failed to fill whole buffer"));
    }
    ret += data.read_u32::<LittleEndian>()? as u64;
    Ok(ret)
}
#[inline(never)]
pub fn sum_byteorder_slice_v2(data: &[u8]) -> io::Result<u64> {
    let mut data = data;
    let mut ret: u64 = 0;
    for _ in 0..data.len()/8 {
        ret += sum_byteorder_slice_impl_v2(&mut data)?;
    }
    Ok(ret)
}


#[inline(never)]
pub fn sum_byteorder_cursor_impl(cursor: &mut Cursor<&[u8]>) -> io::Result<u64> {
    let mut ret = 0;
    ret += cursor.read_u8()? as u64;
    ret += cursor.read_u8()? as u64;
    ret += cursor.read_u16::<LittleEndian>()? as u64;
    ret += cursor.read_u32::<LittleEndian>()? as u64;
    Ok(ret)
}
#[inline(never)]
pub fn sum_byteorder_cursor(data: &[u8]) -> io::Result<u64> {
    let mut cursor = Cursor::new(data);
    let mut ret: u64 = 0;
    for _ in 0..data.len()/8 {
        ret += sum_byteorder_cursor_impl(&mut cursor)?;
    }
    Ok(ret)
}


#[inline(never)]
pub fn sum_byteorder_cursor_impl_v2(cursor: &mut Cursor<&[u8]>) -> io::Result<u64> {
    let mut ret = 0;
    {
        if cursor.position() as usize >= cursor.get_ref().len() {
            return Err(io::Error::new(io::ErrorKind::Other, "failed to fill whole buffer"));
        }
        let mut data = &cursor.get_ref()[cursor.position() as usize..cursor.position() as usize + 1];
        ret += data.read_u8()? as u64;
        cursor.set_position(cursor.position()+1);
    }
    {
        if cursor.position() as usize >= cursor.get_ref().len() {
            return Err(io::Error::new(io::ErrorKind::Other, "failed to fill whole buffer"));
        }
        let mut data = &cursor.get_ref()[cursor.position() as usize..cursor.position() as usize + 1];
        ret += data.read_u8()? as u64;
        cursor.set_position(cursor.position()+1);
    }
    {
        if cursor.position() as usize + 1 >= cursor.get_ref().len() {
            return Err(io::Error::new(io::ErrorKind::Other, "failed to fill whole buffer"));
        }
        let mut data = &cursor.get_ref()[cursor.position() as usize..cursor.position() as usize + 2];
        ret += data.read_u16::<LittleEndian>()? as u64;
        cursor.set_position(cursor.position()+2);
    }
    {
        if cursor.position() as usize + 3 >= cursor.get_ref().len() {
            return Err(io::Error::new(io::ErrorKind::Other, "failed to fill whole buffer"));
        }
        let mut data = &cursor.get_ref()[cursor.position() as usize..cursor.position() as usize + 4];
        ret += data.read_u32::<LittleEndian>()? as u64;
        cursor.set_position(cursor.position()+4);
    }
    Ok(ret)
}
#[inline(never)]
pub fn sum_byteorder_cursor_v2(data: &[u8]) -> io::Result<u64> {
    let mut cursor = Cursor::new(data);
    let mut ret: u64 = 0;
    for _ in 0..data.len()/8 {
        ret += sum_byteorder_cursor_impl_v2(&mut cursor)?;
    }
    Ok(ret)
}



#[inline(never)]
pub fn sum_byteorder_cursor_impl_v2_unsafe(cursor: &mut Cursor<&[u8]>) -> io::Result<u64> {
    let mut ret = 0;
    {
        if cursor.position() as usize >= cursor.get_ref().len() {
            return Err(io::Error::new(io::ErrorKind::Other, "failed to fill whole buffer"));
        }
        ret += unsafe { *cursor.get_ref().get_unchecked(cursor.position() as usize) } as u64;
        cursor.set_position(cursor.position()+1);
    }
    {
        if cursor.position() as usize >= cursor.get_ref().len() {
            return Err(io::Error::new(io::ErrorKind::Other, "failed to fill whole buffer"));
        }
        ret += unsafe { *cursor.get_ref().get_unchecked(cursor.position() as usize) } as u64;
        cursor.set_position(cursor.position()+1);
    }
    {
        if cursor.position() as usize + 1 >= cursor.get_ref().len() {
            return Err(io::Error::new(io::ErrorKind::Other, "failed to fill whole buffer"));
        }
        let mut data = unsafe { cursor.get_ref().get_unchecked(cursor.position() as usize..cursor.position() as usize + 2) };
        ret += data.read_u16::<LittleEndian>()? as u64;
        cursor.set_position(cursor.position()+2);
    }
    {
        if cursor.position() as usize + 3 >= cursor.get_ref().len() {
            return Err(io::Error::new(io::ErrorKind::Other, "failed to fill whole buffer"));
        }
        let mut data = unsafe {cursor.get_ref().get_unchecked(cursor.position() as usize..cursor.position() as usize + 4) };
        ret += data.read_u32::<LittleEndian>()? as u64;
        cursor.set_position(cursor.position()+4);
    }
    Ok(ret)
}
#[inline(never)]
pub fn sum_byteorder_cursor_v2_unsafe(data: &[u8]) -> io::Result<u64> {
    let mut cursor = Cursor::new(data);
    let mut ret: u64 = 0;
    for _ in 0..data.len()/8 {
        ret += sum_byteorder_cursor_impl_v2_unsafe(&mut cursor)?;
    }
    Ok(ret)
}


#[test]
fn test_funcs() {
    let data: Vec<u8> = vec![
        7u8,
        0u8,
        1u8, 2u8,
        0u8, 0u8, 0u8, 0u8,
        1u8,
        0u8,
        3u8, 4u8,
        0u8, 0u8, 0u8, 0u8
    ];
    assert!(sum_baseline_chunks_loop(&data) == 1548);
    assert!(sum_baseline_naive_loop(&data) == 1548);
    assert!(sum_baseline_naive_loop_v2(&data) == 1548);
    assert!(sum_byteorder_slice(&data).unwrap() == 1548);
    assert!(sum_byteorder_slice_v2(&data).unwrap() == 1548);
    assert!(sum_byteorder_cursor(&data).unwrap() == 1548);
    assert!(sum_byteorder_cursor_v2(&data).unwrap() == 1548);
    assert!(sum_byteorder_cursor_v2_unsafe(&data).unwrap() == 1548);

    let data: Vec<u8> = [1u8, 2u8, 3u8].iter().cycle().take(80000).cloned().collect();

    assert!(sum_baseline_chunks_loop(&data) == 336865294464);
    assert!(sum_baseline_naive_loop(&data) == 336865294464);
    assert!(sum_baseline_naive_loop_v2(&data) == 336865294464);
    assert!(sum_byteorder_slice(&data).unwrap() == 336865294464);
    assert!(sum_byteorder_slice_v2(&data).unwrap() == 336865294464);
    assert!(sum_byteorder_cursor(&data).unwrap() == 336865294464);
    assert!(sum_byteorder_cursor_v2(&data).unwrap() == 336865294464);
    assert!(sum_byteorder_cursor_v2_unsafe(&data).unwrap() == 336865294464);
}


#[cfg(test)]
mod tests {
    use test::{Bencher, black_box};
    use crate::*;
    
    #[bench]
    fn bench_baseline_chunks_loop(b: &mut Bencher) {
        let data: Vec<u8> = [1u8, 2u8, 3u8].iter().cycle().take(80000).cloned().collect();
        b.iter(|| black_box(sum_baseline_chunks_loop(black_box(&data))));
    }
    #[bench]
    fn bench_baseline_naive_loop(b: &mut Bencher) {
        let data: Vec<u8> = [1u8, 2u8, 3u8].iter().cycle().take(80000).cloned().collect();
        b.iter(|| black_box(sum_baseline_naive_loop(black_box(&data))));
    }
    #[bench]
    fn bench_baseline_naive_loop_v2(b: &mut Bencher) {
        let data: Vec<u8> = [1u8, 2u8, 3u8].iter().cycle().take(80000).cloned().collect();
        b.iter(|| black_box(sum_baseline_naive_loop_v2(black_box(&data))));
    }
    #[bench]
    fn bench_byteorder_slice(b: &mut Bencher) {
        let data: Vec<u8> = [1u8, 2u8, 3u8].iter().cycle().take(80000).cloned().collect();
        b.iter(|| black_box(sum_byteorder_slice(black_box(&data))));
    }
    #[bench]
    fn bench_byteorder_slice_v2(b: &mut Bencher) {
        let data: Vec<u8> = [1u8, 2u8, 3u8].iter().cycle().take(80000).cloned().collect();
        b.iter(|| black_box(sum_byteorder_slice_v2(black_box(&data))));
    }
    #[bench]
    fn bench_byteorder_cursor(b: &mut Bencher) {
        let data: Vec<u8> = [1u8, 2u8, 3u8].iter().cycle().take(80000).cloned().collect();
        b.iter(|| black_box(sum_byteorder_cursor(black_box(&data))));
    }
    #[bench]
    fn bench_byteorder_cursor_v2(b: &mut Bencher) {
        let data: Vec<u8> = [1u8, 2u8, 3u8].iter().cycle().take(80000).cloned().collect();
        b.iter(|| black_box(sum_byteorder_cursor_v2(black_box(&data))));
    }
    #[bench]
    fn bench_byteorder_cursor_v2_unsafe(b: &mut Bencher) {
        let data: Vec<u8> = [1u8, 2u8, 3u8].iter().cycle().take(80000).cloned().collect();
        b.iter(|| black_box(sum_byteorder_cursor_v2_unsafe(black_box(&data))));
    }

}
