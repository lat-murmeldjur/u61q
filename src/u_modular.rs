pub fn modular_difference(begin: u32, end: u32, modulo: u32) -> u32 {
    if begin > end {
        return modulo + end - begin;
    } else {
        return end - begin;
    }
}

pub fn modular_offset(begin: u32, offset: u32, modulo: u32) -> u32 {
    if begin + offset >= modulo {
        return begin + offset - modulo;
    } else {
        return begin + offset;
    }
}

pub fn modular_difference_in_range(
    begin: u32,
    end: u32,
    modulo_begin: u32,
    modulo_end: u32,
) -> u32 {
    if begin > end {
        return modular_difference(
            begin - modulo_begin,
            end - modulo_begin,
            modulo_end - modulo_begin + 1,
        );
    } else {
        return end - begin;
    }
}

pub fn modular_offset_in_range(begin: u32, offset: u32, modulo_begin: u32, modulo_end: u32) -> u32 {
    if begin + offset > modulo_end {
        return modulo_begin
            + modular_offset(begin - modulo_begin, offset, modulo_end - modulo_begin + 1);
    } else {
        return begin + offset;
    }
}
