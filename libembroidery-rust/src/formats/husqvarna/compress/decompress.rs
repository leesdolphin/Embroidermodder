extern "C" {
    fn free(__ptr: *mut ::std::os::raw::c_void);
    fn malloc(__size: usize) -> *mut ::std::os::raw::c_void;
    fn memcpy(
        __dest: *mut ::std::os::raw::c_void,
        __src: *const ::std::os::raw::c_void,
        __n: usize,
    ) -> *mut ::std::os::raw::c_void;
    fn memset(
        __s: *mut ::std::os::raw::c_void,
        __c: i32,
        __n: usize,
    ) -> *mut ::std::os::raw::c_void;
}
use formats::husqvarna::compress::data::_163;
 use formats::husqvarna::compress::data::_164;
 use formats::husqvarna::compress::data::_165;
 use formats::husqvarna::compress::data::_166;
 use formats::husqvarna::compress::data::_167;
 use formats::husqvarna::compress::data::_168;
 use formats::husqvarna::compress::data::_169;
 use formats::husqvarna::compress::data::_170;
 use formats::husqvarna::compress::data::_171;
 use formats::husqvarna::compress::data::_172;
 use formats::husqvarna::compress::data::_173;
 use formats::husqvarna::compress::data::_174;
 use formats::husqvarna::compress::data::_175;
 use formats::husqvarna::compress::data::_176;
 use formats::husqvarna::compress::data::_177;
 use formats::husqvarna::compress::data::_178;
 use formats::husqvarna::compress::data::_179;
 use formats::husqvarna::compress::data::_180;
 use formats::husqvarna::compress::data::_181;
 use formats::husqvarna::compress::data::_182;
 use formats::husqvarna::compress::data::_183;
 use formats::husqvarna::compress::data::_184;
 use formats::husqvarna::compress::data::_185;
 use formats::husqvarna::compress::data::_186;
 use formats::husqvarna::compress::data::_187;
 use formats::husqvarna::compress::data::_188;
 use formats::husqvarna::compress::data::_189;
 use formats::husqvarna::compress::data::_190;
 use formats::husqvarna::compress::data::_191;
 use formats::husqvarna::compress::data::_192;
 use formats::husqvarna::compress::data::_193;
 use formats::husqvarna::compress::data::_194;
 use formats::husqvarna::compress::data::_531;
 use formats::husqvarna::compress::data::_533;
 use formats::husqvarna::compress::data::inputArray;
 use formats::husqvarna::compress::data::inputLength;
 use formats::husqvarna::compress::data::inputPosition;
 use formats::husqvarna::compress::data::inputSize_534;
 use formats::husqvarna::compress::data::mStatus;
 use formats::husqvarna::compress::data::outputArray;
 use formats::husqvarna::compress::data::outputPosition;



struct ExpandData {
    mut mStatus: i32,
    mut currentIndex: i32,
    mut remainingBytes: isize,
    mut inputSize: i32,
    //
    // #[no_mangle]
    // pub static mut inputBufferSize: i32 = 0i32;
    //
    // #[no_mangle]
    // pub static mut inputLength: i32 = 0i32;
    //
    // #[no_mangle]
    // pub static mut outputArray: *mut u8 = 0 as (*mut u8);
    //
    // #[no_mangle]
    // pub static mut inputArray: *mut u8 = 0 as (*mut u8);
    //
    // #[no_mangle]
    // pub static mut inputBuffer: *mut u8 = 0 as (*mut u8);
    //
    // #[no_mangle]
    // pub static mut currentPosition: i32 = 0i32;
    //
    // #[no_mangle]
    // pub static mut inputPosition: i32 = 0i32;
    //
    // #[no_mangle]
    // pub static mut outputPosition: i32 = 0i32;
}



#[no_mangle]
pub unsafe extern "C" fn husExpand(
    mut input: *mut u8,
    mut output: *mut u8,
    mut compressedSize: i32,
    mut _269: i32,
) {
    currentPosition = 0i32;
    outputPosition = 0i32;
    currentIndex = 0i32;
    inputBufferSize = bufferSize as (i32);
    mStatus = 0i32;
    outputArray = output;
    inputArray = input;
    inputSize = bufferSize as (i32);
    remainingBytes = compressedSize as (isize);
    if _269 > _137 as (i32) || _269 < _138 as (i32) {
        mStatus = -1i32;
        _175 = 2i16;
    } else {
        _175 = (1i32 << _269) as (i16);
    }
    _172 = 0i16;
    _243 = 0i16;
    _246 = 0i16;
    _244 = 0u16;
    _245 = 0u8;
    _176 = (_175 as (i32) - 1i32) as (i16);
    _166 = malloc(::std::mem::size_of::<u8>().wrapping_mul((_175 as (i32) + 2i32) as (usize)))
        as (*mut u8);
    if !_166.is_null() {
        memset(
            _166 as (*mut ::std::os::raw::c_void),
            0i32,
            ((_175 as (i32) + 2i32) as (usize)).wrapping_mul(::std::mem::size_of::<u8>()),
        );
    }
    _240 = malloc(::std::mem::size_of::<u16>().wrapping_mul(_148)) as (*mut u16);
    if !_240.is_null() {
        memset(
            _240 as (*mut ::std::os::raw::c_void),
            0i32,
            _148.wrapping_mul(::std::mem::size_of::<u16>()),
        );
    }
    _241 = malloc(::std::mem::size_of::<u16>().wrapping_mul(_149)) as (*mut u16);
    if !_241.is_null() {
        memset(
            _241 as (*mut ::std::os::raw::c_void),
            0i32,
            _149.wrapping_mul(::std::mem::size_of::<u16>()),
        );
    }
    _189 = malloc(
        ::std::mem::size_of::<u16>().wrapping_mul(2usize.wrapping_mul(_141).wrapping_sub(1usize)),
    ) as (*mut u16);
    if !_189.is_null() {
        memset(
            _189 as (*mut ::std::os::raw::c_void),
            0i32,
            2usize
                .wrapping_mul(_141)
                .wrapping_sub(1usize)
                .wrapping_mul(::std::mem::size_of::<u16>()),
        );
    }
    _190 = malloc(
        ::std::mem::size_of::<u16>().wrapping_mul(2usize.wrapping_mul(_141).wrapping_sub(1usize)),
    ) as (*mut u16);
    if !_190.is_null() {
        memset(
            _190 as (*mut ::std::os::raw::c_void),
            0i32,
            2usize
                .wrapping_mul(_141)
                .wrapping_sub(1usize)
                .wrapping_mul(::std::mem::size_of::<u16>()),
        );
    }
    _180 = malloc(::std::mem::size_of::<u8>().wrapping_mul(_141)) as (*mut u8);
    if !_180.is_null() {
        memset(
            _180 as (*mut ::std::os::raw::c_void),
            0i32,
            _141.wrapping_mul(::std::mem::size_of::<u8>()),
        );
    }
    _181 = malloc(::std::mem::size_of::<u8>().wrapping_mul(_152 as (usize))) as (*mut u8);
    if !_181.is_null() {
        memset(
            _181 as (*mut ::std::os::raw::c_void),
            0i32,
            (_152 as (usize)).wrapping_mul(::std::mem::size_of::<u8>()),
        );
    }
    if _166 == 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8)
        || _189 == 0i32 as (*mut ::std::os::raw::c_void) as (*mut u16)
        || _190 == 0i32 as (*mut ::std::os::raw::c_void) as (*mut u16)
        || _180 == 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8)
        || _181 == 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8)
        || _240 == 0i32 as (*mut ::std::os::raw::c_void) as (*mut u16)
        || _241 == 0i32 as (*mut ::std::os::raw::c_void) as (*mut u16)
        || inputBuffer == 0i32 as (*mut ::std::os::raw::c_void) as (*mut u8)
    {
        mStatus = -1i32;
    }
    husExpand_expand();
    husExpand_cleanup();
}

#[no_mangle]
pub unsafe extern "C" fn husExpand_cleanup() {
    free(_166 as (*mut ::std::os::raw::c_void));
    free(_189 as (*mut ::std::os::raw::c_void));
    free(_190 as (*mut ::std::os::raw::c_void));
    free(_180 as (*mut ::std::os::raw::c_void));
    free(_181 as (*mut ::std::os::raw::c_void));
    free(_240 as (*mut ::std::os::raw::c_void));
    free(_241 as (*mut ::std::os::raw::c_void));
}

#[no_mangle]
pub unsafe extern "C" fn husExpand_253(mut _254: i16, mut _220: i16, mut _221: i16) {
    let mut _226: i16;
    let mut _203: i16;
    let mut _219: i16;
    let mut _283: u16;
    _219 = husExpand_252(_220 as (i32)) as (i16);
    if _219 as (i32) == 0i32 {
        _203 = husExpand_252(_220 as (i32)) as (i16);
        _226 = 0i16;
        'loop15: loop {
            if !(_226 as (i32) < _254 as (i32)) {
                break;
            }
            *_181.offset(_226 as (isize)) = 0u8;
            _226 = (_226 as (i32) + 1) as (i16);
        }
        _226 = 0i16;
        'loop17: loop {
            if !(_226 as (i32) < 256i32) {
                break;
            }
            *_241.offset(_226 as (isize)) = _203 as (u16);
            _226 = (_226 as (i32) + 1) as (i16);
        }
    } else {
        _226 = 0i16;
        'loop2: loop {
            if !(_226 as (i32) < _219 as (i32)) {
                break;
            }
            _203 = (_182 as (i32) >> 13i32) as (i16);
            if _203 as (i32) == 7i32 {
                _283 = (1u32 << 12i32) as (u16);
                'loop8: loop {
                    if _283 as (i32) & _182 as (i32) == 0 {
                        break;
                    }
                    _283 = (_283 as (i32) >> 1i32) as (u16);
                    _203 = (_203 as (i32) + 1) as (i16);
                }
            }
            husExpand_256(if _203 as (i32) < 7i32 {
                3i32
            } else {
                _203 as (i32) - 3i32
            });
            *_181.offset({
                let _old = _226;
                _226 = (_226 as (i32) + 1) as (i16);
                _old
            } as (isize)) = _203 as (u8);
            if !(_226 as (i32) == _221 as (i32)) {
                continue;
            }
            _203 = husExpand_252(2i32) as (i16);
            'loop11: loop {
                if !({
                    _203 = (_203 as (i32) - 1) as (i16);
                    _203
                } as (i32) >= 0i32)
                {
                    break;
                }
                *_181.offset({
                    let _old = _226;
                    _226 = (_226 as (i32) + 1) as (i16);
                    _old
                } as (isize)) = 0u8;
            }
        }
        'loop3: loop {
            if !(_226 as (i32) < _254 as (i32)) {
                break;
            }
            *_181.offset({
                let _old = _226;
                _226 = (_226 as (i32) + 1) as (i16);
                _old
            } as (isize)) = 0u8;
        }
        husExpand_258(_254 as (i32), _181, 8i32, _241, _149 as (u16));
    }
}

#[no_mangle]
pub unsafe extern "C" fn husExpand_expand() -> i32 {
    let mut _currentBlock;
    let mut _200: i16 = 0i16;
    let mut _278: *mut u8 = _166;
    let mut _279: i16 = _175;
    let mut _280: i16 = _176;
    _243 = 0i16;
    husExpand_251();
    'loop1: loop {
        if !(_243 as (i32) < 5i32) {
            break;
        }
        let mut _203: i16;
        if {
            _203 = husExpand_249() as (i16);
            _203
        } as (usize) <= byte_MAX
        {
            *_278.offset(_200 as (isize)) = _203 as (u8);
            if !({
                _200 = (_200 as (i32) + 1) as (i16);
                _200
            } as (i32) >= _279 as (i32))
            {
                continue;
            }
            _200 = 0i16;
            memcpy(
                &mut *outputArray.offset(outputPosition as (isize)) as (*mut u8)
                    as (*mut ::std::os::raw::c_void),
                _278 as (*const ::std::os::raw::c_void),
                _279 as (usize),
            );
            outputPosition = outputPosition + _279 as (i32);
        } else {
            let mut _226: i16;
            let mut _276: i16 = (_203 as (usize))
                .wrapping_sub(byte_MAX.wrapping_add(1usize).wrapping_sub(_135 as (usize)))
                as (i16);
            if _276 as (usize) == _144 {
                break;
            }
            _226 = (_200 as (i32) - husExpand_250() as (i32) - 1i32 & _280 as (i32)) as (i16);
            if _226 as (usize) < (_279 as (usize)).wrapping_sub(_140).wrapping_sub(1usize)
                && (_200 as (usize) < (_279 as (usize)).wrapping_sub(_140).wrapping_sub(1usize))
            {
                _currentBlock = 9;
            } else {
                _currentBlock = 5;
            }
            'loop5: loop {
                if _currentBlock == 5 {
                    if !({
                        _276 = (_276 as (i32) - 1) as (i16);
                        _276
                    } as (i32) >= 0i32)
                    {
                        break;
                    }
                    *_278.offset(_200 as (isize)) = *_278.offset(_226 as (isize));
                    if {
                        _200 = (_200 as (i32) + 1) as (i16);
                        _200
                    } as (i32) >= _279 as (i32)
                    {
                        _200 = 0i16;
                        memcpy(
                            &mut *outputArray.offset(outputPosition as (isize)) as (*mut u8)
                                as (*mut ::std::os::raw::c_void),
                            _278 as (*const ::std::os::raw::c_void),
                            _279 as (usize),
                        );
                        outputPosition = outputPosition + _279 as (i32);
                    }
                    _226 = (_226 as (i32) + 1i32 & _280 as (i32)) as (i16);
                    _currentBlock = 5;
                } else {
                    if !({
                        _276 = (_276 as (i32) - 1) as (i16);
                        _276
                    } as (i32) >= 0i32)
                    {
                        break;
                    }
                    *_278.offset({
                        let _old = _200;
                        _200 = (_200 as (i32) + 1) as (i16);
                        _old
                    } as (isize)) = *_278.offset({
                        let _old = _226;
                        _226 = (_226 as (i32) + 1) as (i16);
                        _old
                    } as (isize));
                    _currentBlock = 9;
                }
            }
        }
    }
    if _200 as (i32) != 0i32 {
        memcpy(
            &mut *outputArray.offset(outputPosition as (isize)) as (*mut u8)
                as (*mut ::std::os::raw::c_void),
            _278 as (*const ::std::os::raw::c_void),
            _200 as (usize),
        );
        outputPosition = outputPosition + _200 as (i32);
    }
    0i32
}

#[no_mangle]
pub unsafe extern "C" fn husExpand_249() -> u16 {
    let mut _276: u16;
    let mut _283: u16;
    if _244 as (i32) == 0i32 {
        _244 = husExpand_252(16i32);
        husExpand_253(_145, _147, 3i16);
        husExpand_255();
        husExpand_253(_142, _540, -1i16);
        if mStatus < 0i32 {
            return 0u16;
        }
    }
    _244 = (_244 as (i32) - 1) as (u16);
    _276 = *_240.offset((_182 as (i32) >> 4i32) as (isize));
    if _276 as (usize) >= _141 {
        _283 = (1u32 << 3i32) as (u16);
        'loop4: loop {
            if _182 as (i32) & _283 as (i32) != 0 {
                _276 = *_190.offset(_276 as (isize));
            } else {
                _276 = *_189.offset(_276 as (isize));
            }
            _283 = (_283 as (i32) >> 1i32) as (u16);
            if !(_276 as (usize) >= _141) {
                break;
            }
        }
    }
    husExpand_256(*_180.offset(_276 as (isize)) as (i32));
    _276
}

#[no_mangle]
pub unsafe extern "C" fn husExpand_250() -> u16 {
    let mut _276: u16;
    let mut _283: u16;
    _276 = *_241.offset((_182 as (i32) >> 8i32) as (isize));
    if _276 as (i32) >= _142 as (i32) {
        _283 = (1u32 << 7i32) as (u16);
        'loop2: loop {
            if _182 as (i32) & _283 as (i32) != 0 {
                _276 = *_190.offset(_276 as (isize));
            } else {
                _276 = *_189.offset(_276 as (isize));
            }
            _283 = (_283 as (i32) >> 1i32) as (u16);
            if !(_276 as (i32) >= _142 as (i32)) {
                break;
            }
        }
    }
    husExpand_256(*_181.offset(_276 as (isize)) as (i32));
    if _276 as (i32) != 0i32 {
        _276 = (_276 as (i32) - 1) as (u16);
        _276 = (1u32 << _276 as (i32)).wrapping_add(husExpand_252(_276 as (i32)) as (u32)) as (i16)
            as (u16);
    }
    _276
}

#[no_mangle]
pub unsafe extern "C" fn husExpand_251() {
    _244 = 0u16;
    husExpand_257();
}

#[no_mangle]
pub unsafe extern "C" fn husExpand_252(mut _219: i32) -> u16 {
    let mut _284: u16 = (_182 as (i32) >> 16i32 - _219) as (u16);
    husExpand_256(_219);
    _284
}

#[no_mangle]
pub unsafe extern "C" fn husExpand_255() {
    let mut _226: i16;
    let mut _203: i16;
    let mut _219: i16 = husExpand_252(_143 as (i32)) as (i16);
    if _219 as (i32) == 0i32 {
        _203 = husExpand_252(_143 as (i32)) as (i16);
        _226 = 0i16;
        'loop23: loop {
            if !(_226 as (usize) < _141) {
                break;
            }
            *_180.offset(_226 as (isize)) = 0u8;
            _226 = (_226 as (i32) + 1) as (i16);
        }
        _226 = 0i16;
        'loop25: loop {
            if !(_226 as (usize) < _148) {
                break;
            }
            *_240.offset(_226 as (isize)) = _203 as (u16);
            _226 = (_226 as (i32) + 1) as (i16);
        }
    } else {
        _226 = 0i16;
        'loop2: loop {
            if !(_226 as (i32) < _219 as (i32)) {
                break;
            }
            _203 = *_241.offset((_182 as (i32) >> 8i32) as (isize)) as (i16);
            if _203 as (i32) >= _145 as (i32) {
                let mut _283: u16 = (1i32 << 7i32) as (u16);
                'loop8: loop {
                    if _182 as (i32) & _283 as (i32) != 0i32 {
                        _203 = *_190.offset(_203 as (isize)) as (i16);
                    } else {
                        _203 = *_189.offset(_203 as (isize)) as (i16);
                    }
                    _283 = (_283 as (i32) >> 1i32) as (u16);
                    if !(_203 as (i32) >= _145 as (i32)) {
                        break;
                    }
                }
            }
            husExpand_256(*_181.offset(_203 as (isize)) as (i32));
            if _203 as (i32) <= 2i32 {
                if _203 as (i32) == 0i32 {
                    _203 = 1i16;
                } else if _203 as (i32) == 1i32 {
                    _203 = (husExpand_252(4i32) as (i32) + 3i32) as (i16);
                } else {
                    _203 = (husExpand_252(_143 as (i32)) as (i32) + 20i32) as (i16);
                }
                'loop20: loop {
                    if !({
                        _203 = (_203 as (i32) - 1) as (i16);
                        _203
                    } as (i32) >= 0i32)
                    {
                        break;
                    }
                    *_180.offset({
                        let _old = _226;
                        _226 = (_226 as (i32) + 1) as (i16);
                        _old
                    } as (isize)) = 0u8;
                }
            } else {
                *_180.offset({
                    let _old = _226;
                    _226 = (_226 as (i32) + 1) as (i16);
                    _old
                } as (isize)) = (_203 as (i32) - 2i32) as (u8);
            }
        }
        'loop3: loop {
            if !(_226 as (usize) < _141) {
                break;
            }
            *_180.offset({
                let _old = _226;
                _226 = (_226 as (i32) + 1) as (i16);
                _old
            } as (isize)) = 0u8;
        }
        husExpand_258(_141 as (i32), _180, 12i32, _240, _148 as (u16));
    }
}

#[no_mangle]
pub unsafe extern "C" fn husExpand_256(mut _219: i32) {
    'loop0: loop {
        if !(_219 > _172 as (i32)) {
            break;
        }
        _219 = _219 - _172 as (i32);
        _182 =
            ((_182 as (i32) << _172 as (i32)) + (_245 as (i32) >> 8i32 - _172 as (i32))) as (u16);
        if _246 as (i32) <= 0i32 {
            currentIndex = 0i32;
            if remainingBytes >= 0isize && (remainingBytes as (usize) < bufferSize) {
                inputBuffer = &mut *inputArray.offset(currentPosition as (isize)) as (*mut u8);
                currentPosition = (currentPosition as (isize) + remainingBytes) as (i32);
                _246 = remainingBytes as (i16);
                remainingBytes = remainingBytes - _246 as (isize);
                inputBufferSize = _246 as (i32);
            } else {
                inputBuffer = &mut *inputArray.offset(currentPosition as (isize)) as (*mut u8);
                currentPosition = (currentPosition as (usize)).wrapping_add(bufferSize) as (i32);
                _246 = bufferSize as (i16);
                inputBufferSize = _246 as (i32);
            }
            if _246 as (i32) <= 0i32 {
                _243 = (_243 as (i32) + 1) as (i16);
            }
        }
        _245 = *inputBuffer.offset({
            let _old = currentIndex;
            currentIndex = currentIndex + 1;
            _old
        } as (isize));
        _246 = (_246 as (i32) - 1) as (i16);
        _172 = 8i16;
    }
    _172 = (_172 as (i32) - _219) as (i16);
    _182 = ((_182 as (i32) << _219) + (_245 as (i32) >> 8i32 - _219)) as (u16);
    _245 = (_245 as (i32) << _219) as (u8);
}

#[no_mangle]
pub unsafe extern "C" fn husExpand_257() {
    _182 = 0u16;
    _245 = 0u8;
    _172 = 0i16;
    _246 = 0i16;
    husExpand_256(16i32);
}

#[no_mangle]
pub unsafe extern "C" fn husExpand_258(
    mut _259: i32,
    mut _260: *mut u8,
    mut _261: i32,
    mut _262: *mut u16,
    mut _263: u16,
) {
    let mut _currentBlock;
    let mut _277: [u16; 17];
    let mut _287: [u16; 17];
    let mut _288: [u16; 18];
    let mut _204: *mut u16;
    let mut _226: u32;
    let mut _289: u32;
    let mut _209: u32;
    let mut _290: u32;
    let mut _291: u32;
    let mut _292: u32;
    let mut _293: u32;
    let mut _283: u32;
    _288[0usize] = 0u16;
    _277[0usize] = 0u16;
    _287[0usize] = 0u16;
    _226 = 1u32;
    'loop1: loop {
        if !(_226 <= 16u32) {
            break;
        }
        _277[_226 as (usize)] = 0u16;
        _226 = _226.wrapping_add(1u32);
    }
    _226 = 0u32;
    'loop3: loop {
        if !(_226 as (i32) < _259) {
            break;
        }
        let _rhs = 1;
        let _lhs = &mut _277[*_260.offset(_226 as (isize)) as (usize)];
        *_lhs = (*_lhs as (i32) + _rhs) as (u16);
        _226 = _226.wrapping_add(1u32);
    }
    _288[1usize] = 0u16;
    _226 = 1u32;
    'loop5: loop {
        if !(_226 <= 16u32) {
            break;
        }
        _288[_226.wrapping_add(1u32) as (usize)] = (_288[_226 as (usize)] as (i32)
            + (_277[_226 as (usize)] as (i32) << 16u32.wrapping_sub(_226)))
            as (u16);
        _226 = _226.wrapping_add(1u32);
    }
    if _288[17usize] as (i32) != (1u32 << 16i32) as (u16) as (i32) {
        mStatus = -1i32;
        _243 = 10i16;
    } else {
        _291 = (16i32 - _261) as (u32);
        _226 = 1u32;
        'loop8: loop {
            if !(_226 as (i32) <= _261) {
                break;
            }
            let _rhs = _291;
            let _lhs = &mut _288[_226 as (usize)];
            *_lhs = (*_lhs as (i32) >> _rhs) as (u16);
            _287[_226 as (usize)] = (1u32 << (_261 as (u32)).wrapping_sub(_226)) as (u16);
            _226 = _226.wrapping_add(1u32);
        }
        'loop9: loop {
            if !(_226 <= 16u32) {
                break;
            }
            _287[_226 as (usize)] = (1u32 << 16u32.wrapping_sub(_226)) as (u16);
            _226 = _226.wrapping_add(1u32);
        }
        _226 = (_288[(_261 + 1i32) as (usize)] as (i32) >> _291) as (u32);
        if _226 != (1u32 << 16i32) as (u16) as (u32) {
            _289 = 1u32 << _261;
            'loop12: loop {
                if !(_226 != _289) {
                    break;
                }
                *_262.offset({
                    let _old = _226;
                    _226 = _226.wrapping_add(1u32);
                    _old
                } as (isize)) = 0u16;
            }
        }
        _292 = _259 as (u32);
        _283 = 1u32 << 15i32 - _261;
        _290 = 0u32;
        'loop14: loop {
            if !(_290 as (i32) < _259) {
                _currentBlock = 15;
                break;
            }
            if !({
                _209 = *_260.offset(_290 as (isize)) as (u32);
                _209
            } == 0u32)
            {
                _293 = (_288[_209 as (usize)] as (i32) + _287[_209 as (usize)] as (i32)) as (u32);
                if _209 as (i32) <= _261 {
                    if _293 > _263 as (u32) {
                        _currentBlock = 33;
                        break;
                    }
                    _226 = _288[_209 as (usize)] as (u32);
                    'loop29: loop {
                        if !(_226 < _293) {
                            break;
                        }
                        *_262.offset(_226 as (isize)) = _290 as (u16);
                        _226 = _226.wrapping_add(1u32);
                    }
                } else {
                    _289 = _288[_209 as (usize)] as (u32);
                    _204 = &mut *_262.offset((_289 >> _291) as (isize)) as (*mut u16);
                    _226 = _209.wrapping_sub(_261 as (u32));
                    'loop19: loop {
                        if !(_226 != 0u32) {
                            break;
                        }
                        if *_204 as (i32) == 0i32 {
                            *_190.offset(_292 as (isize)) = {
                                let _rhs = 0i32;
                                let _lhs = &mut *_189.offset(_292 as (isize));
                                *_lhs = _rhs as (u16);
                                *_lhs
                            };
                            *_204 = {
                                let _old = _292;
                                _292 = _292.wrapping_add(1u32);
                                _old
                            } as (u16);
                        }
                        if _289 & _283 != 0 {
                            _204 = &mut *_190.offset(*_204 as (isize)) as (*mut u16);
                        } else {
                            _204 = &mut *_189.offset(*_204 as (isize)) as (*mut u16);
                        }
                        _289 = _289 << 1i32;
                        _226 = _226.wrapping_sub(1u32);
                    }
                    *_204 = _290 as (u16);
                }
                _288[_209 as (usize)] = _293 as (u16);
            }
            _290 = _290.wrapping_add(1u32);
        }
        (if _currentBlock == 15 {
        } else {
            mStatus = -1i32;
            _243 = 10i16;
        })
    }
}
