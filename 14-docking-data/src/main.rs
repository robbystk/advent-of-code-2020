use std::collections::HashMap;

use std::slice::Iter;

const SET_LSB: u64 = 0b0000_0000_0000_0000_0000_0000_0000_0000_0001;
const RESET_LSB: u64 = 0b1111_1111_1111_1111_1111_1111_1111_1111_1110;

enum MaskBit {
    Transparent,    // 0
    Set,            // 1
    Floating        // X
}

struct Mask {
    mask: Vec<MaskBit>,
}

impl Mask {
    fn new() -> Mask {
        Mask { mask: Vec::new() }
    }

    fn update(self: &mut Mask, s: &str) {
        self.mask.clear();

        for c in s.chars() {
            match c {
                'X' => self.mask.push(MaskBit::Floating),
                '0' => self.mask.push(MaskBit::Transparent),
                '1' => self.mask.push(MaskBit::Set),
                _ => panic!("Invalid mask character `{}`", c)
            }
        }
    }

    fn decode_address(self: &Mask, address: u64, address_list: &mut AddressList) {
        use MaskBit::*;

        let mut bit_mask: u64 = 1 << 35;

        for mask_bit in &self.mask {
            let bit = address & bit_mask != 0;
            bit_mask >>= 1;
            match mask_bit {
                Transparent => address_list.push(bit),
                Set => address_list.push(true),
                Floating => address_list.bifurcate()
            }
        }
    }
}

#[derive(Debug)]
struct AddressList {
    address_list: Vec<u64>
}

impl AddressList {
    fn new() -> AddressList {
        AddressList { address_list: vec![0] }
    }

    fn push(self: &mut AddressList, bit: bool) {
        for addr in self.address_list.iter_mut() {
            *addr <<= 1;
            if bit { *addr |= SET_LSB }
        }
    }

    fn bifurcate(self: &mut AddressList) {
        let mut new_addresses = Vec::with_capacity(self.address_list.len());
        for addr in self.address_list.iter_mut() {
            *addr <<= 1;
            new_addresses.push(*addr | SET_LSB);
        }
        self.address_list.extend(new_addresses);
    }

    fn iter(self: &AddressList) -> Iter<u64> {
        self.address_list.iter()
    }
}

fn main() {
    let input_filename = &std::env::args().collect::<Vec<String>>()[1];
    let input = std::fs::read_to_string(input_filename).unwrap();

    let mut memory = HashMap::<u64, u64>::new();

    let mut mask = Mask::new();

    for line in input.lines() {
        let parts = line.split('=').collect::<Vec<&str>>();

        let directive = parts[0].trim();
        let value = parts[1].trim();

        // println!("{}, {}", directive, value);

        if directive.starts_with("mask") {
            mask.update(value);
        } else if directive.starts_with("mem[") {
            let address_str = directive[4..].strip_suffix(']').unwrap();
            let value: u64 = value.parse().unwrap();
            let address: u64 = address_str.parse().unwrap();

            let mut address_list = AddressList::new();

            mask.decode_address(address, &mut address_list);
            // println!("{:?}", address_list);

            for address in address_list.iter() {
                memory.insert(*address, value);
            }

        } else {
            panic!("Can't parse line: `{}`", line);
        }
        // println!("{:#038b}\n{:#038b}", zero_mask, one_mask);
    }

    let memory_sum: u64 = memory.values().sum();

    println!("{:?}", memory_sum);
}
