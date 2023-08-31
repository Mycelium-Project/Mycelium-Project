
ENCODED_ALPHABET = "162636465666768696:6;6<6=6>6?607172737475767778797:7142434445464748494:4;4<4=4>4?405152535455565758595:5"
ALPHABET = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"

def fix_hex(hex_str: str):
    new_str = ""
    for char in hex_str:
        orded = ord(char)
        if 58 <= orded <= 63:
            new_str += chr(orded + 39)
        else:
            new_str += char
    return new_str

def unfix_hex(hex_str: str):
    new_str = ""
    for char in hex_str:
        orded = ord(char)
        if 97 <= orded <= 102:
            new_str += chr(orded - 39)
        else:
            new_str += char
    return new_str

def swap_hex(hex_str: str):
    return hex_str[1] + hex_str[0]

def hex_to_int(hex_str: str):
    return int(swap_hex(fix_hex(hex_str)), 16)

def int_to_ascii(int_val: int):
    return chr(int_val)

def decode(encoded_str: str):
    decoded_str = ""
    for i in range(0, len(encoded_str), 2):
        hex_str = encoded_str[i:i+2]
        int_val = hex_to_int(hex_str)
        ascii_val = int_to_ascii(int_val)
        decoded_str += ascii_val
    return decoded_str

def encode(decoded_str: str):
    encoded_str = ""
    for char in decoded_str:
        int_val = ord(char)
        hex_str = hex(int_val)[2:]
        encoded_str += unfix_hex(swap_hex(hex_str))
    return encoded_str

assert decode(ENCODED_ALPHABET) == ALPHABET
assert encode(ALPHABET) == ENCODED_ALPHABET