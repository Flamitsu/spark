/*
 * Copyright (C) 2026 Flamitsu
 *
 * This file is part of Ignix.
 *
 * Ignix is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, version 3.
 *
 * Ignix is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with Ignix.  If not, see <https://www.gnu.org/licenses/>.
 */

/*
 * Pre-calculates the binary table in compilation time. So if you need to
 * calculate 92 bytes, it will do 92 iterations, instead of 736 like if it is calculated in
 * execution.
 */
const fn generate_crc32_table() -> [u32;256]{
    let mut table = [0u32;256];
    let mut index = 0;
    while index < 256{
        let mut crc = index as u32;
        let mut current_byte = 0;
        while current_byte < 8{
            // If the Least Significant Byte is 1, shift and XOR it to the polynomial number
            if crc & 1 != 0{
                crc = (crc >> 1) ^ 0xEDB88320;
            // If the Least Significant Byte is 0, shift it to the right
            } else {
                crc >>= 1;
            }
            current_byte += 1;
        }
        table[index] = crc;
        index += 1;
    }
    table
}
const CRC32_TABLE: [u32; 256] = generate_crc32_table();
pub fn calculate_crc32(data: &[u8]) -> u32{
    let mut crc: u32 = 0xFFFFFFFF;
    for &byte in data{
        // Does XOR operation and AND operation over 0xFF
        let index = ((crc ^ byte as u32) & 0xFF) as usize;
        // Shifts to the 8 to the right and XOR it with the CRC32 pre calculated index.
        crc = (crc >> 8) ^ CRC32_TABLE[index];
    }
    // Returns the NOT value of the final crc (0x04C11DB7 polynomial)
    !crc
}


#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_calculate_crc32(){
        let input = b"123456789";
        let crc32_result = 0xCBF43926;
        let crc32 = calculate_crc32(input);
        assert_eq!(crc32,crc32_result, "Function does not match the waited value.");
    }
    #[test]
    fn test_calculate_crc32_invalid(){
        let input = b"123456789";
        let crc32_result = 0x12345678;
        let crc32 = calculate_crc32(input);
        assert_ne!(crc32,crc32_result, "Function does not match the waited value.")
    }
}
