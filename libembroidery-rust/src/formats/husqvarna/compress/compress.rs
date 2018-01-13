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
use std::vec::Vec;


use formats::husqvarna::compress::constants::*;
use formats::husqvarna::compress::data::*;
use formats::husqvarna::compress::errors::Result;


/// An iterator over the range [start, stop) by `step`. It handles overflow by stopping.
#[derive(Clone)]
pub struct ReverseRange {
    state: usize,
    stop: usize,
}
/// Return an iterator over the range [start, stop) by `step`. It handles overflow by stopping.
#[inline]
pub fn range_rev(start: usize, stop: usize) -> ReverseRange {
    ReverseRange { state: start, stop }
}

impl Iterator for ReverseRange {
    type Item = usize;

    #[inline]
    fn next(&mut self) -> Option<usize> {
        if self.state > self.stop {
            let result = self.state.clone();
            match self.state.checked_sub(1) {
                Some(x) => self.state = x,
                None => self.state = self.stop.clone(),
            }
            Some(result)
        } else {
            None
        }
    }
}

//
//#[no_mangle]
//pub unsafe extern "C" fn husCompress(
//    mut _inputArray: *mut u8,
//    mut _inputSize: usize,
//    mut _outputArray: *mut u8,
//    mut _269: i32,
//    mut _235: i32,
//) -> i32 {
//    CompressData()
//}
//
//pub fn compress(
//    inputData: &[u8],
//    ahiahi: i32,
//    local_513: i32,
//) -> Result<&[u8]> {
//    return CompressData{}.begin(
//        inputData: &[u8],
//        ahiahi: i32,
//        local_513: i32
//    )
//}
const KATOTE_PURI_SIZE: usize = (2 * RAWEKE) - 1;

#[derive(Clone)]
struct CompressData {
    outputArray: Vec<u8>, //  unsigned char.

    tirehe: i16,    // _ 170
    haumia: i16,    // _ 171
    tapuau: i16,    // _ 172
    ketere: i16,    // _ 173
    whakareri: i16, // _ 174
    purara: i16,    // _ 175

    horona: u16,   //_182
    whawha: u16,   //_183
    piwari: u16,   //_184
    komiti: u16,   //_185
    puketutu: u16, //_186

    taruku: i32, // _531
    wataka: i32, // _533

    /// A null-terminated buffer.
    tutahi: [u8; TUTAHI_BUFFER_SIZE], // _165
    hamano: [u16; 17],                // _167
    renana: [u8; RENANA_BUFFER_SIZE], // _179
    puri: [u16; KATOTE_PURI_SIZE],    // _189
    katote: [u16; KATOTE_PURI_SIZE],  // _190
    herepo: [u16; KATOTE_PURI_SIZE],  // _191
    paerua: [u16; KATOTE_PURI_SIZE],  // _193
                                      //    inputArray: &'a[u8],
                                      //    outputArray: Vec<u8>,
}

//impl <'a> CompressData<'a> {
impl<'a> CompressData {
    fn new() -> Self {
        CompressData {
            // TODO: can the capacity be calculated ahead of time?
            outputArray: Vec::with_capacity(512),

            haumia: 0i16,
            tirehe: 0i16,
            tapuau: 0,
            ketere: 0,
            whakareri: 0,
            purara: 0,
            horona: 0,
            whawha: (TUTAHI_BUFFER_SIZE - ((3 * 8) + 6)) as u16,
            piwari: 0u16,
            komiti: 1u16,
            puketutu: 0,

            tutahi: [0u8; TUTAHI_BUFFER_SIZE],
            hamano: [0u16; 17],
            puri: [0u16; KATOTE_PURI_SIZE],
            katote: [0u16; KATOTE_PURI_SIZE],
            herepo: [0u16; KATOTE_PURI_SIZE],
            paerua: [0u16; KATOTE_PURI_SIZE],
        }
    }

    fn generate_hamano_lookup_table(&mut self, mutu: usize) {
        // husCompress_232
        if mutu < self.whakareri as usize {
            let lhs_index = self.ketere.min(16) as (usize);
            self.hamano[lhs_index] += 1;
        } else {
            self.ketere += 1;
            let compress_mutu_1 = self.puri[mutu] as usize;
            let compress_mutu_2 = self.katote[mutu] as usize;
            self.generate_hamano_lookup_table(compress_mutu_1);
            self.generate_hamano_lookup_table(compress_mutu_2);
            self.ketere -= 1;
        }
    }

    fn kainui(&mut self, iawaka: i32, ngaiwi: &[u8], hauhu: &mut [u16]) {
        // husCompress_230
        //      void husCompress_230(int _219, unsigned char* _209, unsigned short* _231)
        //      {
        //        int _226;
        //        unsigned short _288[18];
        //        _288[1] = 0;
        //        for(_226 = 1; _226 <= 16; _226++)
        //        _288[_226+1] = (unsigned short)((_288[_226]+_167[_226])<<1);
        //        for(_226 = 0; _226 < _219; _226++)
        //        _231[_226] = _288[_209[_226]]++;
        //      }
        let mut ratara: [u16; 18] = [0u16; 18]; // _288
        for i in 1..16usize {
            ratara[i + 1] = (ratara[i] + self.hamano[i]) << 1;
        }
        for i in 0..(iawaka as usize) {
            let ratara_index = ngaiwi[i] as usize;
            hauhu[i] = ratara[ratara_index];
            ratara[ratara_index] += 1;
        }
    }

    fn uehoka(&mut self, mut makoia: i32) {
        // husCompress_228
        // TODO: Check type of makoia, should it be usize or u16?
        let mut putoti: i32;
        let mut ripata: usize = 0;

        self.hamano = [0; 17];
        self.generate_hamano_lookup_table(makoia as usize);

        // for(_226 = 16; _226 > 0; _226--)
        //   _458+=_167[_226]<<(16-_226);
        for i in range_rev(16, 0) {
            ripata = ripata.wrapping_add((self.hamano[i] << (16 - i)) as usize);
        }

        //        while(_458 != (1U<<16)) {
        //            _167[16]--;
        //            for(_226 = 15; _226 > 0; _226--) {
        //                if(_167[_226] != 0) {
        //                    _167[_226]--;
        //                    _167[_226+1] = (unsigned short)(_167[_226+1]+2);
        //                    break; } }
        //            _458--; }
        while ripata != (1u32 << 16i32) as usize {
            self.hamano[16] -= 1;
            for i in range_rev(15, 0) {
                if self.hamano[i] != 0 {
                    self.hamano[i] -= 1;
                    self.hamano[i + 1] = self.hamano[i + 1] + 2;
                    break;
                }
            }
            ripata -= 1;
        }

        //        for(_226 = 16; _226 > 0; _226--) {
        //            _289 = _167[_226];
        //            while(--_289 >= 0) {
        //                _178[*_188++] = (unsigned char)_226; } }
        for i in range_rev(16, 0) {
            putoti = self.hamano[i as (usize)] as (i32);
            for _ in 0..putoti {
                // TODO: Fix this.
                unsafe {
                    *hota.offset(*{
                        let _old = ekehi;
                        ekehi = ekehi.offset(1isize);
                        _old
                    } as (isize)) = i as (u8);
                }
            }
        }
    }

    fn kaimoe(&mut self) {
        // husCompress_198
        let mut tawata: *mut i16;
        // _450 = &_163[_175];
        unsafe {
            tawata = &mut *_163.offset(self.purara as (isize)) as (*mut i16);
        }
        //for(i = _153; i > 0; i--)
        //  *_450++ = _157;
        for i in 0.._153 {
            // TODO: Fix this.
            unsafe {
                *tawata.offset(i as isize) = _157;
            }
        }
        // Note: Some parts of this function(using _164) are useless.
    }

    fn watihu(&mut self) {
        // husCompress_196
    }

    fn commit_renana_buffer_to_output(&mut self) {
        // husCompress_210
        if self.haumia as (i32) <= 0i32 {
            return;
        }

        if self.taruku != 0 && ({
            self.wataka = self.wataka.wrapping_add(self.haumia);
            self.wataka
        } >= inputSize_534)
        {
            self.tirehe = 1i16;
        } else {
            self.outputArray.extend(self.renana[..self.haumia]);
        }
        self.haumia = 0i16;
    }

//    fn husCompress_208(&mut self, mut tukapi: i32, mut nganei: u16) {
//        // Tired me to morning me: Have I even converted this right!?
//        nganei = (nganei << _133 - tukapi) as (u16);
//        self.horona = (self.horona | (nganei >> self.tapuau));
//        self.tapuau = (self.tapuau + tukapi);
//        if self.tapuau >= 8i32 {
//            if self.haumia as (usize) >= RENANA_BUFFER_SIZE {
//                // TODO: WARN: BOUNDS!
//                self.commit_renana_buffer_to_output();
//            }
//            *self.renana.offset({
//                let _old = self.haumia;
//                self.haumia = (self.haumia as (i32) + 1) as (i16);
//                _old
//            } as (isize)) = (self.horona as (i32) >> 8i32) as (u8);
//            if {
//                self.tapuau = (self.tapuau as (i32) - 8i32) as (u16) as (i16);
//                self.tapuau
//            } as (i32) < 8i32
//            {
//                self.horona = (self.horona as (i32) << 8i32) as (u16);
//            } else {
//                if self.haumia as (usize) >= RENANA_BUFFER_SIZE {
//                    // TODO: WARN: BOUNDS!
//                    self.commit_renana_buffer_to_output();
//                }
//                *self.renana.offset({
//                    let _old = self.haumia;
//                    self.haumia = (self.haumia as (i32) + 1) as (i16);
//                    _old
//                } as (isize)) = self.horona as (u8);
//                self.tapuau = (self.tapuau as (i32) - 8i32) as (u16) as (i16);
//                self.horona = (nganei as (i32) << tukapi - self.tapuau as (i32)) as (u16);
//            }
//        }
//    }
    //    fn begin(
    //        mut self,
    //        inputData: &[u8],
    //        ahiahi: i32,
    //        local_513: i32) -> Result<&[u8]> {
    //        let mut returnVal: i32;
    //        self.inputArray = inputData;
    //        self.outputArray = Vec::new();
    //        _531 = local_513;
    //        if ahiahi > AHIAHI_LOWER_LIMIT && ahiahi < AHIAHI_UPPER_LIMIT {
    //            _175 = 1i16 << ahiahi;
    //        } else {
    //            mStatus = -1i32;
    //            _175 = 2i16;
    //        }
    //    }
}
//
//
//#[no_mangle]
//pub unsafe extern "C" fn husCompress_(
//    mut _inputArray: *mut u8,
//    mut _inputSize: usize,
//    mut _outputArray: *mut u8,
//    mut ahiahi: i32,
//    mut local_513: i32,
//) -> i32 {
//    let mut returnVal: i32;
//    inputArray = _inputArray;
//    outputArray = _outputArray;
//    _531 = local_513;
//    if ahiahi > AHIAHI_LOWER_LIMIT && ahiahi < AHIAHI_UPPER_LIMIT {
//        _175 = 1i16 << ahiahi;
//    } else {
//        mStatus = -1i32;
//        _175 = 2i16;
//    }
//    _176 = (_175 as (i32) - 1i32) as (i16);
//    _166 = malloc(
//        ::std::mem::size_of::<u8>()
//            .wrapping_mul((_175 as (usize)).wrapping_add(_140).wrapping_add(2usize)),
//    ) as (*mut u8);
//    if !_166.is_null() {
//        memset(
//            _166 as (*mut ::std::os::raw::c_void),
//            0i32,
//            (_175 as (usize))
//                .wrapping_add(_140)
//                .wrapping_add(2usize)
//                .wrapping_mul(::std::mem::size_of::<u8>()),
//        );
//    }
//    _163 = malloc(::std::mem::size_of::<i16>().wrapping_mul((_175 as (usize)).wrapping_add(_153)))
//        as (*mut i16);
//    if !_163.is_null() {
//        memset(
//            _163 as (*mut ::std::os::raw::c_void),
//            0i32,
//            (_175 as (usize))
//                .wrapping_add(_153)
//                .wrapping_mul(::std::mem::size_of::<i16>()),
//        );
//    }
//    _164 = malloc(::std::mem::size_of::<i16>().wrapping_mul(_175 as (usize))) as (*mut i16);
//    if !_164.is_null() {
//        memset(
//            _164 as (*mut ::std::os::raw::c_void),
//            0i32,
//            (_175 as (usize)).wrapping_mul(::std::mem::size_of::<i16>()),
//        );
//    }
//    _165 = malloc(::std::mem::size_of::<u8>().wrapping_mul(_155)) as (*mut u8);
//    if !_165.is_null() {
//        memset(
//            _165 as (*mut ::std::os::raw::c_void),
//            0i32,
//            _155.wrapping_mul(::std::mem::size_of::<u8>()),
//        );
//    }
//    _179 = malloc(::std::mem::size_of::<u8>().wrapping_mul(_156)) as (*mut u8);
//    if !_179.is_null() {
//        memset(
//            _179 as (*mut ::std::os::raw::c_void),
//            0i32,
//            _156.wrapping_mul(::std::mem::size_of::<u8>()),
//        );
//    }
//    _189 = malloc(
//        ::std::mem::size_of::<u16>().wrapping_mul(2usize.wrapping_mul(RAWEKE).wrapping_sub(1usize)),
//    ) as (*mut u16);
//    if !_189.is_null() {
//        memset(
//            _189 as (*mut ::std::os::raw::c_void),
//            0i32,
//            2usize
//                .wrapping_mul(RAWEKE)
//                .wrapping_sub(1usize)
//                .wrapping_mul(::std::mem::size_of::<u16>()),
//        );
//    }
//    _190 = malloc(
//        ::std::mem::size_of::<u16>().wrapping_mul(2usize.wrapping_mul(RAWEKE).wrapping_sub(1usize)),
//    ) as (*mut u16);
//    if !_190.is_null() {
//        memset(
//            _190 as (*mut ::std::os::raw::c_void),
//            0i32,
//            2usize
//                .wrapping_mul(RAWEKE)
//                .wrapping_sub(1usize)
//                .wrapping_mul(::std::mem::size_of::<u16>()),
//        );
//    }
//    _177 =
//        malloc(::std::mem::size_of::<i16>().wrapping_mul(RAWEKE.wrapping_add(1usize))) as (*mut i16);
//    if !_177.is_null() {
//        memset(
//            _177 as (*mut ::std::os::raw::c_void),
//            0i32,
//            RAWEKE.wrapping_add(1usize)
//                .wrapping_mul(::std::mem::size_of::<i16>()),
//        );
//    }
//    _180 = malloc(::std::mem::size_of::<u8>().wrapping_mul(RAWEKE)) as (*mut u8);
//    if !_180.is_null() {
//        memset(
//            _180 as (*mut ::std::os::raw::c_void),
//            0i32,
//            RAWEKE.wrapping_mul(::std::mem::size_of::<u8>()),
//        );
//    }
//    _191 = malloc(
//        ::std::mem::size_of::<u16>().wrapping_mul(2usize.wrapping_mul(RAWEKE).wrapping_sub(1usize)),
//    ) as (*mut u16);
//    if !_191.is_null() {
//        memset(
//            _191 as (*mut ::std::os::raw::c_void),
//            0i32,
//            2usize
//                .wrapping_mul(RAWEKE)
//                .wrapping_sub(1usize)
//                .wrapping_mul(::std::mem::size_of::<u16>()),
//        );
//    }
//    _192 = malloc(::std::mem::size_of::<u16>().wrapping_mul(RAWEKE)) as (*mut u16);
//    if !_192.is_null() {
//        memset(
//            _192 as (*mut ::std::os::raw::c_void),
//            0i32,
//            RAWEKE.wrapping_mul(::std::mem::size_of::<u16>()),
//        );
//    }
//    _181 = malloc(::std::mem::size_of::<u8>().wrapping_mul(_152 as (usize))) as (*mut u8);
//    if !_181.is_null() {
//        memset(
//            _181 as (*mut ::std::os::raw::c_void),
//            0i32,
//            (_152 as (usize)).wrapping_mul(::std::mem::size_of::<u8>()),
//        );
//    }
//    _193 = malloc(
//        ::std::mem::size_of::<u16>().wrapping_mul((2i32 * _142 as (i32) - 1i32) as (usize)),
//    ) as (*mut u16);
//    if !_193.is_null() {
//        memset(
//            _193 as (*mut ::std::os::raw::c_void),
//            0i32,
//            ((2i32 * _142 as (i32) - 1i32) as (usize)).wrapping_mul(::std::mem::size_of::<u16>()),
//        );
//    }
//    _194 = malloc(::std::mem::size_of::<u16>().wrapping_mul(_152 as (usize))) as (*mut u16);
//    if !_194.is_null() {
//        memset(
//            _194 as (*mut ::std::os::raw::c_void),
//            0i32,
//            (_152 as (usize)).wrapping_mul(::std::mem::size_of::<u16>()),
//        );
//    }
//    if _166.is_null() || _163.is_null() || _164.is_null() || _165.is_null() || _179.is_null()
//        || _189.is_null() || _190.is_null() || _177.is_null() || _180.is_null()
//        || _191.is_null() || _192.is_null() || _181.is_null() || _193.is_null()
//        || _194.is_null()
//    {
//        mStatus = -1i32;
//    }
//    _533 = 0usize;
//    inputSize_534 = _inputSize;
//    inputLength = _inputSize as (i32);
//    inputPosition = 0i32;
//    outputPosition = 0i32;
//    returnVal = husCompress_compress();
//    husCompress_cleanup();
//    returnVal
//}
//
//#[no_mangle]
//pub unsafe extern "C" fn husCompress_cleanup() {
//    free(_166 as (*mut ::std::os::raw::c_void));
//    free(_163 as (*mut ::std::os::raw::c_void));
//    free(_164 as (*mut ::std::os::raw::c_void));
//    free(_165 as (*mut ::std::os::raw::c_void));
//    free(_179 as (*mut ::std::os::raw::c_void));
//    free(_189 as (*mut ::std::os::raw::c_void));
//    free(_190 as (*mut ::std::os::raw::c_void));
//    free(_177 as (*mut ::std::os::raw::c_void));
//    free(_180 as (*mut ::std::os::raw::c_void));
//    free(_191 as (*mut ::std::os::raw::c_void));
//    free(_192 as (*mut ::std::os::raw::c_void));
//    free(_181 as (*mut ::std::os::raw::c_void));
//    free(_193 as (*mut ::std::os::raw::c_void));
//    free(_194 as (*mut ::std::os::raw::c_void));
//}
//
//#[no_mangle]
//pub unsafe extern "C" fn husCompress_223(mut _203: i16) {
//    husCompress_208(
//        *_180.offset(_203 as (isize)) as (i32),
//        *_192.offset(_203 as (isize)),
//    );
//}
//
//#[no_mangle]
//pub unsafe extern "C" fn husCompress_compress() -> i32 {
//    let mut _currentBlock;
//    let mut _209: i16;
//    let mut _201: i16;
//    let mut _200: i16;
//    let mut s: i16;
//    let mut _231: i32;
//    let mut _278: *mut u8;
//    let mut _280: i16;
//    let mut _279: i16;
//    _278 = _166;
//    _280 = _176;
//    _279 = _175;
//    _231 = 0i32;
//    husCompress_196();
//    husCompress_198();
//    _200 = 0i16;
//    memcpy(
//        _278 as (*mut ::std::os::raw::c_void),
//        &mut *inputArray.offset(inputPosition as (isize)) as (*mut u8)
//            as (*const ::std::os::raw::c_void),
//        _279 as (usize),
//    );
//    inputPosition = inputPosition + _279 as (i32);
//    if inputPosition > inputLength {
//        _209 = (inputLength - inputPosition) as (i16);
//    } else {
//        _209 = _279;
//    }
//    s = (_209 as (i32) & _280 as (i32)) as (i16);
//    _169 = 0i16;
//    _168 = 0i16;
//    _201 = ((*_278.offset(_200 as (isize)) as (i32) << _154 as (i32)
//        ^ *_278.offset((_200 as (i32) + 1i32) as (isize)) as (i32)) as (usize)
//        & _153.wrapping_sub(1usize)) as (i16);
//    _201 = ((_201 as (i32) << _154 as (i32)
//        ^ *_278.offset((_200 as (i32) + 2i32) as (isize)) as (i32)) as (i16) as (usize)
//        & _153.wrapping_sub(1usize))
//        .wrapping_add(_279 as (usize)) as (i16);
//    'loop4: loop {
//        if !(_209 as (usize) > _140.wrapping_add(4usize) && (self.tirehe == 0)) {
//            break;
//        }
//        husCompress_199(_200, _201);
//        if _168 as (i32) < _135 as (i32) {
//            husCompress_202(*_278.offset(_200 as (isize)) as (u16), 0u16);
//            let mut _204: i16;
//            if {
//                _204 = *_163.offset(_201 as (isize));
//                _204
//            } as (i32) != _157 as (i32)
//            {
//                *_164.offset(_204 as (isize)) = _200;
//            }
//            *_164.offset(_200 as (isize)) = _201;
//            *_163.offset(_200 as (isize)) = _204;
//            *_163.offset(_201 as (isize)) = _200;
//            _200 = (_200 as (i32) + 1) as (i16);
//            _201 = ((_201 as (i32) << _154 as (i32)
//                ^ *_278.offset((_200 as (i32) + 2i32) as (isize)) as (i32))
//                as (i16) as (usize) & _153.wrapping_sub(1usize))
//                .wrapping_add(_279 as (usize)) as (i16);
//            _209 = (_209 as (i32) - 1) as (i16);
//        } else {
//            _209 = (_209 as (i32) - _168 as (i32)) as (i16);
//            husCompress_202(
//                (_168 as (i32) + (0x7fi32 * 2i32 + 1i32 + 1i32 - _135 as (i32))) as (u16),
//                _169 as (u16),
//            );
//            'loop39: loop {
//                if !({
//                    _168 = (_168 as (i32) - 1) as (i16);
//                    _168
//                } as (i32) >= 0i32)
//                {
//                    break;
//                }
//                let mut _204: i16;
//                if {
//                    _204 = *_163.offset(_201 as (isize));
//                    _204
//                } as (i32) != _157 as (i32)
//                {
//                    *_164.offset(_204 as (isize)) = _200;
//                }
//                *_164.offset(_200 as (isize)) = _201;
//                *_163.offset(_200 as (isize)) = _204;
//                *_163.offset(_201 as (isize)) = _200;
//                _200 = (_200 as (i32) + 1) as (i16);
//                _201 = ((_201 as (i32) << _154 as (i32)
//                    ^ *_278.offset((_200 as (i32) + 2i32) as (isize)) as (i32))
//                    as (i16) as (usize) & _153.wrapping_sub(1usize))
//                    .wrapping_add(_279 as (usize)) as (i16);
//            }
//        }
//    }
//    'loop5: loop {
//        if !(_209 as (usize) < _140) {
//            break;
//        }
//        let mut _203: i32;
//        if inputPosition >= inputLength {
//            break;
//        }
//        _203 = *inputArray.offset(inputPosition as (isize)) as (i32);
//        inputPosition = inputPosition + 1i32;
//        *_278.offset(s as (isize)) = _203 as (u8);
//        if s as (usize) < _140.wrapping_sub(1usize) {
//            *_278.offset((s as (i32) + _279 as (i32)) as (isize)) = *_278.offset(s as (isize));
//        }
//        let mut _204: i16;
//        if {
//            _204 = *_164.offset(s as (isize));
//            _204
//        } as (i32) != _157 as (i32)
//        {
//            *_164.offset(s as (isize)) = _157;
//            *_163.offset(_204 as (isize)) = _157;
//        }
//        s = (s as (i32) + 1i32 & _280 as (i32)) as (i16);
//        _209 = (_209 as (i32) + 1) as (i16);
//    }
//    'loop13: loop {
//        if !(_209 as (i32) > 0i32 && (self.tirehe == 0)) {
//            _currentBlock = 14;
//            break;
//        }
//        husCompress_199(_200, _201);
//        if _168 as (i32) > _209 as (i32) {
//            _168 = _209;
//        }
//        if _168 as (i32) < _135 as (i32) {
//            _168 = 1i16;
//            husCompress_202(*_278.offset(_200 as (isize)) as (u16), 0u16);
//        } else {
//            husCompress_202(
//                (_168 as (i32) + (0x7fi32 * 2i32 + 1i32 + 1i32 - _135 as (i32))) as (u16),
//                _169 as (u16),
//            );
//        }
//        'loop22: loop {
//            if !({
//                _168 = (_168 as (i32) - 1) as (i16);
//                _168
//            } as (i32) >= 0i32)
//            {
//                break;
//            }
//            let mut _203: i32;
//            if inputPosition >= inputLength {
//                break;
//            }
//            _203 = *inputArray.offset(inputPosition as (isize)) as (i32);
//            inputPosition = inputPosition + 1i32;
//            *_278.offset(s as (isize)) = _203 as (u8);
//            if s as (usize) < _140.wrapping_sub(1usize) {
//                *_278.offset((s as (i32) + _279 as (i32)) as (isize)) = *_278.offset(s as (isize));
//            }
//            let mut _204: i16;
//            if {
//                _204 = *_164.offset(s as (isize));
//                _204
//            } as (i32) != _157 as (i32)
//            {
//                *_164.offset(s as (isize)) = _157;
//                *_163.offset(_204 as (isize)) = _157;
//            }
//            s = (s as (i32) + 1i32 & _280 as (i32)) as (i16);
//            let mut _204: i16;
//            if {
//                _204 = *_163.offset(_201 as (isize));
//                _204
//            } as (i32) != _157 as (i32)
//            {
//                *_164.offset(_204 as (isize)) = _200;
//            }
//            *_164.offset(_200 as (isize)) = _201;
//            *_163.offset(_200 as (isize)) = _204;
//            *_163.offset(_201 as (isize)) = _200;
//            _200 = (_200 as (i32) + 1i32 & _280 as (i32)) as (i16);
//            _201 = ((_201 as (i32) << _154 as (i32)
//                ^ *_278.offset((_200 as (i32) + 2i32) as (isize)) as (i32))
//                as (i16) as (usize) & _153.wrapping_sub(1usize))
//                .wrapping_add(_279 as (usize)) as (i16);
//        }
//        'loop31: loop {
//            if !({
//                let _old = _168;
//                _168 = (_168 as (i32) - 1) as (i16);
//                _old
//            } as (i32) >= 0i32)
//            {
//                break;
//            }
//            let mut _204: i16;
//            if {
//                _204 = *_163.offset(_201 as (isize));
//                _204
//            } as (i32) != _157 as (i32)
//            {
//                *_164.offset(_204 as (isize)) = _200;
//            }
//            *_164.offset(_200 as (isize)) = _201;
//            *_163.offset(_200 as (isize)) = _204;
//            *_163.offset(_201 as (isize)) = _200;
//            _200 = (_200 as (i32) + 1i32 & _280 as (i32)) as (i16);
//            _201 = ((_201 as (i32) << _154 as (i32)
//                ^ *_278.offset((_200 as (i32) + 2i32) as (isize)) as (i32))
//                as (i16) as (usize) & _153.wrapping_sub(1usize))
//                .wrapping_add(_279 as (usize)) as (i16);
//            _209 = (_209 as (i32) - 1) as (i16);
//        }
//        if mStatus < 0i32 {
//            _currentBlock = 33;
//            break;
//        }
//    }
//    if _currentBlock == 14 {
//        if self.tirehe == 0 {
//            husCompress_202(
//                _144.wrapping_add((0x7fi32 * 2i32 + 1i32 + 1i32 - _135 as (i32)) as (usize))
//                    as (u16),
//                0u16,
//            );
//        }
//        husCompress_197();
//        outputPosition
//    } else {
//        1i32
//    }
//}
//
//
//#[no_mangle]
//pub unsafe extern "C" fn husCompress_197() {
//    if self.tirehe == 0 {
//        husCompress_207();
//    }
//    husCompress_206();
//    _183 = 0u16;
//    _184 = 0u16;
//}
//
//#[no_mangle]
//pub unsafe extern "C" fn husCompress_198() {
//    let mut i: i32;
//    let mut _450: *mut i16;
//    _450 = &mut *_163.offset(_175 as (isize)) as (*mut i16);
//    i = _153 as (i32);
//    'loop1: loop {
//        if !(i > 0i32) {
//            break;
//        }
//        *{
//            let _old = _450;
//            _450 = _450.offset(1isize);
//            _old
//        } = _157;
//        i = i - 1;
//    }
//    _450 = _164;
//    i = _175 as (i32);
//    'loop3: loop {
//        if !(i > 0i32) {
//            break;
//        }
//        *{
//            let _old = _450;
//            _450 = _450.offset(1isize);
//            _old
//        } = _157;
//        i = i - 1;
//    }
//}
//
//#[no_mangle]
//pub unsafe extern "C" fn husCompress_199(mut _200: i16, mut _201: i16) {
//    let mut _451: *mut u8;
//    let mut _278: *mut u8;
//    let mut _226: i16;
//    let mut _452: i16;
//    let mut _204: i16;
//    let mut _453: i16;
//    _452 = _158;
//    _168 = 0i16;
//    _451 = &mut *_166.offset(_200 as (isize)) as (*mut u8);
//    _204 = _201;
//    'loop1: loop {
//        if !({
//            _204 = *_163.offset(_204 as (isize));
//            _204
//        } as (i32) != _157 as (i32))
//        {
//            break;
//        }
//        if {
//            _452 = (_452 as (i32) - 1) as (i16);
//            _452
//        } as (i32) < 0i32
//        {
//            break;
//        }
//        _278 = &mut *_166.offset(_204 as (isize)) as (*mut u8);
//        if *_451.offset(_168 as (isize)) as (i32) != *_278.offset(_168 as (isize)) as (i32) {
//            continue;
//        }
//        if *_451.offset(0isize) as (i32) != *_278.offset(0isize) as (i32) {
//            continue;
//        }
//        if *_451.offset(1isize) as (i32) != *_278.offset(1isize) as (i32) {
//            continue;
//        }
//        if *_451.offset(2isize) as (i32) != *_278.offset(2isize) as (i32) {
//            continue;
//        }
//        _226 = 3i16;
//        'loop8: loop {
//            if !(_226 as (usize) < _140) {
//                break;
//            }
//            if *_451.offset(_226 as (isize)) as (i32) != *_278.offset(_226 as (isize)) as (i32) {
//                break;
//            }
//            _226 = (_226 as (i32) + 1) as (i16);
//        }
//        if !(_226 as (i32) > _168 as (i32)) {
//            continue;
//        }
//        _453 = (_200 as (i32) - _204 as (i32) - 1i32) as (i16);
//        if _453 as (i32) < 0i32 {
//            _453 = (_453 as (i32) + _175 as (i32)) as (i16);
//        }
//        if _453 as (i32) >= _175 as (i32) {
//            break;
//        }
//        _169 = _453;
//        if {
//            _168 = _226;
//            _168
//        } as (usize) >= _140
//        {
//            break;
//        }
//    }
//}
//
//#[no_mangle]
//pub unsafe extern "C" fn husCompress_202(mut _203: u16, mut _204: u16) {
//    if {
//        _185 = (_185 as (i32) >> 1i32) as (u16);
//        _185
//    } as (i32) == 0i32
//    {
//        _185 = (1u32 << 8i32 - 1i32) as (u16);
//        if _184 as (i32) >= _183 as (i32) {
//            husCompress_207();
//            if self.tirehe != 0 {
//                return;
//            } else {
//                _184 = 0u16;
//            }
//        }
//        _186 = {
//            let _old = _184;
//            _184 = (_184 as (i32) + 1) as (u16);
//            _old
//        };
//        *_165.offset(_186 as (isize)) = 0u8;
//    }
//    *_165.offset({
//        let _old = _184;
//        _184 = (_184 as (i32) + 1) as (u16);
//        _old
//    } as (isize)) = _203 as (u8);
//    let _rhs = 1;
//    let _lhs = &mut *_191.offset(_203 as (isize));
//    *_lhs = (*_lhs as (i32) + _rhs) as (u16);
//    if _203 as (u32) >= 1u32 << 8i32 {
//        let _rhs = _185 as (u8);
//        let _lhs = &mut *_165.offset(_186 as (isize));
//        *_lhs = (*_lhs as (i32) | _rhs as (i32)) as (u8);
//        *_165.offset({
//            let _old = _184;
//            _184 = (_184 as (i32) + 1) as (u16);
//            _old
//        } as (isize)) = _204 as (u8);
//        *_165.offset({
//            let _old = _184;
//            _184 = (_184 as (i32) + 1) as (u16);
//            _old
//        } as (isize)) = (_204 as (i32) >> 8i32) as (u8);
//        _203 = 0u16;
//        'loop7: loop {
//            if _204 == 0 {
//                break;
//            }
//            _203 = (_203 as (i32) + 1) as (u16);
//            _204 = (_204 as (i32) >> 1i32) as (u16);
//        }
//        let _rhs = 1;
//        let _lhs = &mut *_193.offset(_203 as (isize));
//        *_lhs = (*_lhs as (i32) + _rhs) as (u16);
//    }
//}

//#[no_mangle]
//pub unsafe extern "C" fn husCompress_206() {
//    if self.tirehe == 0 {
//        husCompress_208(8i32 - 1i32, 0u16);
//        if _171 != 0 {
//            husCompress_210();
//        }
//    }
//    _171 = 0i16;
//}
//
//#[no_mangle]
//pub unsafe extern "C" fn husCompress_207() {
//    let mut _currentBlock;
//    let mut _226: u32;
//    let mut _289: u32;
//    let mut _229: u32;
//    let mut _454: u32;
//    let mut _455: u32;
//    let mut _456: u32 = 0u32;
//    let mut _217: [u16; 3405691582];
//    _229 = husCompress_211(RAWEKE as (i32), _191, _180, _192) as (u32);
//    _455 = *_191.offset(_229 as (isize)) as (u32);
//    husCompress_208(16i32, _455 as (u16));
//    if _229 as (usize) >= RAWEKE {
//        husCompress_216(_217.as_mut_ptr());
//        _229 = husCompress_211(_145 as (i32), _217.as_mut_ptr(), _181, _194) as (u32);
//        if _229 >= _145 as (u32) {
//            husCompress_218(_145, _147, 3i16);
//        } else {
//            husCompress_208(_147 as (i32), 0u16);
//            husCompress_208(_147 as (i32), _229 as (u16));
//        }
//        husCompress_222();
//    } else {
//        husCompress_208(_147 as (i32), 0u16);
//        husCompress_208(_147 as (i32), 0u16);
//        husCompress_208(_143 as (i32), 0u16);
//        husCompress_208(_143 as (i32), _229 as (u16));
//    }
//    _229 = husCompress_211(_142 as (i32), _193, _181, _194) as (u32);
//    if _229 >= _142 as (u32) {
//        husCompress_218(_142, _540, -1i16);
//    } else {
//        husCompress_208(_540 as (i32), 0u16);
//        husCompress_208(_540 as (i32), _229 as (u16));
//    }
//    _454 = 0u32;
//    _226 = 0u32;
//    'loop10: loop {
//        if !(_226 < _455) {
//            _currentBlock = 11;
//            break;
//        }
//        if _226.wrapping_rem(8u32) == 0u32 {
//            _456 = *_165.offset({
//                let _old = _454;
//                _454 = _454.wrapping_add(1u32);
//                _old
//            } as (isize)) as (u32);
//        } else {
//            _456 = _456 << 1i32;
//        }
//        if _456 & 1u32 << 8i32 - 1i32 != 0 {
//            husCompress_223((*_165.offset({
//                let _old = _454;
//                _454 = _454.wrapping_add(1u32);
//                _old
//            } as (isize)) as (u32))
//                .wrapping_add(1u32 << 8i32) as (i16));
//            _289 = *_165.offset({
//                let _old = _454;
//                _454 = _454.wrapping_add(1u32);
//                _old
//            } as (isize)) as (u32);
//            _289 = _289.wrapping_add(
//                (*_165.offset({
//                    let _old = _454;
//                    _454 = _454.wrapping_add(1u32);
//                    _old
//                } as (isize)) as (i32) << 8i32) as (u32),
//            );
//            husCompress_224(_289 as (i16) as (u16));
//        } else {
//            husCompress_223(*_165.offset({
//                let _old = _454;
//                _454 = _454.wrapping_add(1u32);
//                _old
//            } as (isize)) as (i16));
//        }
//        if self.tirehe != 0 {
//            _currentBlock = 28;
//            break;
//        }
//        _226 = _226.wrapping_add(1u32);
//    }
//    if _currentBlock == 11 {
//        _226 = 0u32;
//        'loop12: loop {
//            if !(_226 as (usize) < RAWEKE) {
//                break;
//            }
//            *_191.offset(_226 as (isize)) = 0u16;
//            _226 = _226.wrapping_add(1u32);
//        }
//        _226 = 0u32;
//        'loop14: loop {
//            if !(_226 < _142 as (u32)) {
//                break;
//            }
//            *_193.offset(_226 as (isize)) = 0u16;
//            _226 = _226.wrapping_add(1u32);
//        }
//    }
//}
//
//#[no_mangle]
//pub unsafe extern "C" fn husCompress_208(mut _209: i32, mut _203: u16) {
//    _203 = (_203 as (i32) << _133 as (i32) - _209) as (u16);
//    _182 = (_182 as (i32) | (_203 as (i32) >> _172 as (i32)) as (u16) as (i32)) as (u16);
//    if {
//        _172 = (_172 as (i32) + _209 as (i16) as (i32)) as (i16);
//        _172
//    } as (i32) >= 8i32
//    {
//        if _171 as (usize) >= _156 {
//            husCompress_210();
//        }
//        *_179.offset({
//            let _old = _171;
//            _171 = (_171 as (i32) + 1) as (i16);
//            _old
//        } as (isize)) = (_182 as (i32) >> 8i32) as (u8);
//        if {
//            _172 = (_172 as (i32) - 8i32) as (u16) as (i16);
//            _172
//        } as (i32) < 8i32
//        {
//            _182 = (_182 as (i32) << 8i32) as (u16);
//        } else {
//            if _171 as (usize) >= _156 {
//                husCompress_210();
//            }
//            *_179.offset({
//                let _old = _171;
//                _171 = (_171 as (i32) + 1) as (i16);
//                _old
//            } as (isize)) = _182 as (u8);
//            _172 = (_172 as (i32) - 8i32) as (u16) as (i16);
//            _182 = (_203 as (i32) << _209 - _172 as (i32)) as (u16);
//        }
//    }
//}
//
//#[no_mangle]
//pub unsafe extern "C" fn husCompress_211(
//    mut _212: i32,
//    mut _213: *mut u16,
//    mut _214: *mut u8,
//    mut _215: *mut u16,
//) -> i32 {
//    let mut _226: i32;
//    let mut _276: i32;
//    let mut _289: i32;
//    let mut _292: i32;
//    let mut _227: i16;
//    whakareri = _212 as (i16);
//    _187 = _213;
//    _178 = _214;
//    _292 = whakareri as (i32);
//    _227 = 0i16;
//    *_177.offset(1isize) = 0i16;
//    _226 = 0i32;
//    'loop1: loop {
//        if !(_226 < whakareri as (i32)) {
//            break;
//        }
//        *_178.offset(_226 as (isize)) = 0u8;
//        if *_187.offset(_226 as (isize)) != 0 {
//            *_177.offset({
//                _227 = (_227 as (i32) + 1) as (i16);
//                _227
//            } as (isize)) = _226 as (i16);
//        }
//        _226 = _226 + 1;
//    }
//    if _227 as (i32) < 2i32 {
//        *_215.offset(*_177.offset(1isize) as (isize)) = 0u16;
//        *_177.offset(1isize) as (i32)
//    } else {
//        _226 = _227 as (i32) / 2i32;
//        'loop4: loop {
//            if !(_226 >= 1i32) {
//                break;
//            }
//            husCompress_225(_226, _187, _177, _227);
//            _226 = _226 - 1;
//        }
//        _188 = _215;
//        'loop6: loop {
//            _226 = *_177.offset(1isize) as (i32);
//            if _226 < whakareri as (i32) {
//                *{
//                    let _old = _188;
//                    _188 = _188.offset(1isize);
//                    _old
//                } = _226 as (u16);
//            }
//            *_177.offset(1isize) = *_177.offset({
//                let _old = _227;
//                _227 = (_227 as (i32) - 1) as (i16);
//                _old
//            } as (isize));
//            husCompress_225(1i32, _187, _177, _227);
//            _276 = *_177.offset(1isize) as (i32);
//            if _276 < whakareri as (i32) {
//                *{
//                    let _old = _188;
//                    _188 = _188.offset(1isize);
//                    _old
//                } = _276 as (u16);
//            }
//            _289 = {
//                let _old = _292;
//                _292 = _292 + 1;
//                _old
//            };
//            *_187.offset(_289 as (isize)) = (*_187.offset(_226 as (isize)) as (i32)
//                + *_187.offset(_276 as (isize)) as (i32))
//                as (u16);
//            *_177.offset(1isize) = _289 as (i16);
//            husCompress_225(1i32, _187, _177, _227);
//            *_189.offset(_289 as (isize)) = _226 as (u16);
//            *_190.offset(_289 as (isize)) = _276 as (u16);
//            if !(_227 as (i32) > 1i32) {
//                break;
//            }
//        }
//        _188 = _215;
//        husCompress_228(_289);
//        husCompress_230(_212, _214, _215);
//        _289
//    }
//}
//
//#[no_mangle]
//pub unsafe extern "C" fn husCompress_216(mut _217: *mut u16) {
//    let mut _226: i16;
//    let mut _289: i16;
//    let mut _219: i16;
//    let mut _277: i16;
//    _226 = 0i16;
//    'loop1: loop {
//        if !(_226 as (i32) < _145 as (i32)) {
//            break;
//        }
//        *_217.offset(_226 as (isize)) = 0u16;
//        _226 = (_226 as (i32) + 1) as (i16);
//    }
//    _219 = RAWEKE as (i16);
//    'loop3: loop {
//        if !(_219 as (i32) > 0i32
//            && (*_180.offset((_219 as (i32) - 1i32) as (isize)) as (i32) == 0i32))
//        {
//            break;
//        }
//        _219 = (_219 as (i32) - 1) as (i16);
//    }
//    _226 = 0i16;
//    'loop5: loop {
//        if !(_226 as (i32) < _219 as (i32)) {
//            break;
//        }
//        _289 = *_180.offset({
//            let _old = _226;
//            _226 = (_226 as (i32) + 1) as (i16);
//            _old
//        } as (isize)) as (i16);
//        if _289 as (i32) == 0i32 {
//            _277 = 1i16;
//            'loop10: loop {
//                if !(_226 as (i32) < _219 as (i32)
//                    && (*_180.offset(_226 as (isize)) as (i32) == 0i32))
//                {
//                    break;
//                }
//                _226 = (_226 as (i32) + 1) as (i16);
//                _277 = (_277 as (i32) + 1) as (i16);
//            }
//            if _277 as (i32) <= 2i32 {
//                let _rhs = _277;
//                let _lhs = &mut *_217.offset(0isize);
//                *_lhs = (*_lhs as (i32) + _rhs as (i32)) as (u16);
//            } else if _277 as (i32) <= 18i32 {
//                let _rhs = 1;
//                let _lhs = &mut *_217.offset(1isize);
//                *_lhs = (*_lhs as (i32) + _rhs) as (u16);
//            } else if _277 as (i32) == 19i32 {
//                let _rhs = 1;
//                let _lhs = &mut *_217.offset(0isize);
//                *_lhs = (*_lhs as (i32) + _rhs) as (u16);
//                let _rhs = 1;
//                let _lhs = &mut *_217.offset(1isize);
//                *_lhs = (*_lhs as (i32) + _rhs) as (u16);
//            } else {
//                let _rhs = 1;
//                let _lhs = &mut *_217.offset(2isize);
//                *_lhs = (*_lhs as (i32) + _rhs) as (u16);
//            }
//        } else {
//            let _rhs = 1;
//            let _lhs = &mut *_217.offset((_289 as (i32) + 2i32) as (isize));
//            *_lhs = (*_lhs as (i32) + _rhs) as (u16);
//        }
//    }
//}
//
//#[no_mangle]
//pub unsafe extern "C" fn husCompress_218(mut _219: i16, mut _220: i16, mut _221: i16) {
//    let mut _226: i16;
//    let mut _289: i16;
//    'loop1: loop {
//        if !(_219 as (i32) > 0i32
//            && (*_181.offset((_219 as (i32) - 1i32) as (isize)) as (i32) == 0i32))
//        {
//            break;
//        }
//        _219 = (_219 as (i32) - 1) as (i16);
//    }
//    husCompress_208(_220 as (i32), _219 as (u16));
//    _226 = 0i16;
//    'loop3: loop {
//        if !(_226 as (i32) < _219 as (i32)) {
//            break;
//        }
//        _289 = *_181.offset({
//            let _old = _226;
//            _226 = (_226 as (i32) + 1) as (i16);
//            _old
//        } as (isize)) as (i16);
//        if _289 as (i32) <= 6i32 {
//            husCompress_208(3i32, _289 as (u16));
//        } else {
//            husCompress_208(
//                _289 as (i32) - 3i32,
//                (0x7fffi32 * 2i32 + 1i32 << 1i32) as (u16),
//            );
//        }
//        if !(_226 as (i32) == _221 as (i32)) {
//            continue;
//        }
//        'loop9: loop {
//            if !(_226 as (i32) < 6i32 && (*_181.offset(_226 as (isize)) as (i32) == 0i32)) {
//                break;
//            }
//            _226 = (_226 as (i32) + 1) as (i16);
//        }
//        husCompress_208(2i32, (_226 as (i32) - 3i32) as (u16));
//    }
//}
//
//#[no_mangle]
//pub unsafe extern "C" fn husCompress_222() {
//    let mut _226: i16;
//    let mut _289: i16;
//    let mut _219: i16;
//    let mut _277: i16;
//    _219 = RAWEKE as (i16);
//    'loop1: loop {
//        if !(_219 as (i32) > 0i32
//            && (*_180.offset((_219 as (i32) - 1i32) as (isize)) as (i32) == 0i32))
//        {
//            break;
//        }
//        _219 = (_219 as (i32) - 1) as (i16);
//    }
//    husCompress_208(_143 as (i32), _219 as (u16));
//    _226 = 0i16;
//    'loop3: loop {
//        if !(_226 as (i32) < _219 as (i32)) {
//            break;
//        }
//        _289 = *_180.offset({
//            let _old = _226;
//            _226 = (_226 as (i32) + 1) as (i16);
//            _old
//        } as (isize)) as (i16);
//        if _289 as (i32) == 0i32 {
//            _277 = 1i16;
//            'loop8: loop {
//                if !(_226 as (i32) < _219 as (i32)
//                    && (*_180.offset(_226 as (isize)) as (i32) == 0i32))
//                {
//                    break;
//                }
//                _226 = (_226 as (i32) + 1) as (i16);
//                _277 = (_277 as (i32) + 1) as (i16);
//            }
//            if _277 as (i32) <= 2i32 {
//                _289 = 0i16;
//                'loop16: loop {
//                    if !(_289 as (i32) < _277 as (i32)) {
//                        break;
//                    }
//                    husCompress_208(*_181.offset(0isize) as (i32), *_194.offset(0isize));
//                    _289 = (_289 as (i32) + 1) as (i16);
//                }
//            } else if _277 as (i32) <= 18i32 {
//                husCompress_208(*_181.offset(1isize) as (i32), *_194.offset(1isize));
//                husCompress_208(4i32, (_277 as (i32) - 3i32) as (u16));
//            } else if _277 as (i32) == 19i32 {
//                husCompress_208(*_181.offset(0isize) as (i32), *_194.offset(0isize));
//                husCompress_208(*_181.offset(1isize) as (i32), *_194.offset(1isize));
//                husCompress_208(4i32, 15u16);
//            } else {
//                husCompress_208(*_181.offset(2isize) as (i32), *_194.offset(2isize));
//                husCompress_208(_143 as (i32), (_277 as (i32) - 20i32) as (u16));
//            }
//        } else {
//            husCompress_208(
//                *_181.offset((_289 as (i32) + 2i32) as (isize)) as (i32),
//                *_194.offset((_289 as (i32) + 2i32) as (isize)),
//            );
//        }
//    }
//}
//
//#[no_mangle]
//pub unsafe extern "C" fn husCompress_224(mut _204: u16) {
//    let mut _203: u16;
//    let mut _457: u16;
//    _203 = 0u16;
//    _457 = _204;
//    'loop1: loop {
//        if _457 == 0 {
//            break;
//        }
//        _203 = (_203 as (i32) + 1) as (u16);
//        _457 = (_457 as (i32) >> 1i32) as (u16);
//    }
//    husCompress_208(
//        *_181.offset(_203 as (isize)) as (i32),
//        *_194.offset(_203 as (isize)),
//    );
//    if _203 as (i32) > 1i32 {
//        husCompress_208(_203 as (i32) - 1i32, _204);
//    }
//}
//
//#[no_mangle]
//pub unsafe extern "C" fn husCompress_225(
//    mut _226: i32,
//    mut local_187: *mut u16,
//    mut local_177: *mut i16,
//    mut _227: i16,
//) {
//    let mut _276: i32;
//    let mut _289: i32;
//    _289 = *local_177.offset(_226 as (isize)) as (i32);
//    'loop1: loop {
//        if !({
//            _276 = 2i32 * _226;
//            _276
//        } <= _227 as (i32))
//        {
//            break;
//        }
//        if _276 < _227 as (i32)
//            && (*local_187.offset(*local_177.offset(_276 as (isize)) as (isize)) as (i32)
//                > *local_187.offset(*local_177.offset((_276 + 1i32) as (isize)) as (isize)) as (i32))
//        {
//            _276 = _276 + 1;
//        }
//        if *local_187.offset(_289 as (isize)) as (i32)
//            <= *local_187.offset(*local_177.offset(_276 as (isize)) as (isize)) as (i32)
//        {
//            break;
//        }
//        *local_177.offset(_226 as (isize)) = *local_177.offset(_276 as (isize));
//        _226 = _276;
//    }
//    *local_177.offset(_226 as (isize)) = _289 as (u16) as (i16);
//}
//
