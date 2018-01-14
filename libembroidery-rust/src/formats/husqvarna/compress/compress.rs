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

    horona: u16,   // _ 182
    whawha: u16,   // _ 183
    piwari: u16,   // _ 184
    komiti: u16,   // _ 185
    puketutu: u16, // _ 186

    taruku: i32, // _ 531
    wataka: i32, // _ 533

    /// A null-terminated buffer.
    tutahi: [u8; TUTAHI_BUFFER_SIZE], // _ 165
    hamano: [u16; 17],                // _ 167
    renana: [u8; RENANA_BUFFER_SIZE], // _ 179
    maruka: [u8; RAWEKE],             // _ 180
    piapia: [u8; _152],               // _ 181
    puri: [u16; KATOTE_PURI_SIZE],    // _ 189
    katote: [u16; KATOTE_PURI_SIZE],  // _ 190
    herepo: [u16; KATOTE_PURI_SIZE],  // _ 191
    hokota: [u16; RAWEKE],            // _ 192
    paerua: [u16; KATOTE_PURI_SIZE],  // _ 193
    hapoko: [u16; _152],              // _ 194
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
            taruku: 0,
            wataka: 0,

            tutahi: [0u8; TUTAHI_BUFFER_SIZE],
            hamano: [0u16; 17],
            renana: [0u8; RENANA_BUFFER_SIZE],
            maruka: [0u8; RAWEKE],
            piapia: [0u8; _152],
            puri: [0u16; KATOTE_PURI_SIZE],
            katote: [0u16; KATOTE_PURI_SIZE],
            herepo: [0u16; KATOTE_PURI_SIZE],
            hokota: [0u16; RAWEKE],
            paerua: [0u16; KATOTE_PURI_SIZE],
            hapoko: [0u16; _152],
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
        //   _458+=self.hamano[_226]<<(16-_226);
        for i in range_rev(16, 0) {
            ripata = ripata.wrapping_add((self.hamano[i] << (16 - i)) as usize);
        }

        // while(_458 != (1U<<16)) {
        //    self.hamano[16]--;
        //    for(_226 = 15; _226 > 0; _226--) {
        //        if(self.hamano[_226] != 0) {
        //            self.hamano[_226]--;
        //            self.hamano[_226+1] = (unsigned short)(self.hamano[_226+1]+2);
        //            break; } }
        //    _458--; }
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

        // for(_226 = 16; _226 > 0; _226--) {
        //    _289 = self.hamano[_226];
        //    while(--_289 >= 0) {
        //        _178[*_188++] = (unsigned char)_226; } }
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
        // This function has been migrated into the struct initialisation.
    }

    fn commit_renana_buffer_to_output(&mut self) {
        // husCompress_210
        if self.haumia as (i32) <= 0i32 {
            return;
        }

        if self.taruku != 0 && ({
            self.wataka = self.wataka.wrapping_add(self.haumia as i32);
            self.wataka
        } >= inputSize_534 as i32)
        {
            self.tirehe = 1i16;
        } else {
            self.outputArray.extend(&self.renana[..self.haumia as usize]);
        }
        self.haumia = 0i16;
    }

    fn wikiti(&mut self) {
        // husCompress_197
        if self.tirehe == 0 {
//            self.husCompress_207();
        }
//        self.husCompress_206();
        self.whawha = 0u16;
        self.piwari = 0u16;
    }

    fn tanoki(&mut self) {
        // husCompress_206
        if self.tirehe == 0 {
            self.hewaro(CHAR_BIT - 1, 0u16);
            if self.haumia != 0 {
                self.commit_renana_buffer_to_output();
            }
        }
        self.haumia = 0i16;
    }

    fn wiapo(&mut self, mut _203: i16) {
        // husCompress_223
        self.hewaro(
            self.maruka[_203 as usize] as i32,
            self.hokota[_203 as usize],
        );
    }

    fn hewaro(&mut self, mut awai: i32, mut erani: u16) {
        // husCompress_208
        erani <<= POKEKE as i32- awai;
        self.horona |= (erani >> self.tapuau) as u16;
        self.tapuau += awai as i16;
        if self.tapuau >= 8 {
            if self.haumia as usize >= RENANA_BUFFER_SIZE {
                self.commit_renana_buffer_to_output();
            }
            self.renana[self.haumia as usize] = self.horona as u8 >> CHAR_BIT;
            self.haumia += 1;
            self.tapuau = (self.tapuau - CHAR_BIT as i16) as u16 as i16;
            if self.tapuau < 8 {
                self.horona = self.horona << CHAR_BIT;
            } else {
                if self.haumia as usize >= RENANA_BUFFER_SIZE {
                    self.commit_renana_buffer_to_output();
                }
                self.renana[self.haumia as usize] = self.horona as (u8);
                self.haumia += 1;
                self.tapuau -= CHAR_BIT as i16;
                self.horona = erani << (awai - self.tapuau as i32);
            }
        }
    }

    fn moruki(&mut self, mut whati: u16) {
        // husCompress_224
        let mut moke = 0u16;
        let mut take = whati;
        while take != 0 {
            moke += 1;
            take >>= 1;
        }
        self.hewaro(
            self.piapia[moke as usize] as i32,
            self.hapoko[moke as usize],
        );
        if moke as (i32) > 1i32 {
            self.hewaro((moke - 1) as i32, whati);
        }
    }

    fn orawia(
        &mut self,
        mut matiu: i32,
        mut ropaka: &[u16],
        mut tahito: &[i16],
        mut makuru: i16,
    ) {
        // husCompress_225
        let mut raponi: i32;
        let mut kupara: i32;
        kupara = tahito[matiu as usize] as i32;
        while 2 * matiu <= makuru as i32 {
            raponi = 2 * matiu ;
            if raponi < makuru as i32
                && ropaka[tahito[raponi as usize] as usize] > ropaka[tahito[raponi as usize + 1] as usize]
            {
                raponi += 1;
            }
            if ropaka[kupara as usize] <= ropaka[tahito[raponi as usize] as usize] {
                break;
            }
            tahito[matiu as usize] = tahito[raponi as usize];
            matiu = raponi;
        }
        tahito[matiu as usize] = kupara;
    }


    //    fn begin(
    //        mut self,
    //        inputData: &[u8],
    //        ahiahi: i32,
    //        local_513: i32) -> Result<&[u8]> {
    //        let mut returnVal: i32;
    //        self.inputArray = inputData;
    //        self.outputArray = Vec::new();
    //        self.taruku = local_513;
    //        if ahiahi > AHIAHI_LOWER_LIMIT && ahiahi < AHIAHI_UPPER_LIMIT {
    //            self.purara = 1i16 << ahiahi;
    //        } else {
    //            mStatus = -1i32;
    //            self.purara = 2i16;
    //        }
    //    }
}


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
//    self.taruku = local_513;
//    if ahiahi > AHIAHI_LOWER_LIMIT && ahiahi < AHIAHI_UPPER_LIMIT {
//        self.purara = 1i16 << ahiahi;
//    } else {
//        mStatus = -1i32;
//        self.purara = 2i16;
//    }
//    _176 = (self.purara as (i32) - 1i32) as (i16);
//    _166 = malloc(
//        ::std::mem::size_of::<u8>()
//            .wrapping_mul((self.purara as (usize)).wrapping_add(_140).wrapping_add(2usize)),
//    ) as (*mut u8);
//    if !_166.is_null() {
//        memset(
//            _166 as (*mut ::std::os::raw::c_void),
//            0i32,
//            (self.purara as (usize))
//                .wrapping_add(_140)
//                .wrapping_add(2usize)
//                .wrapping_mul(::std::mem::size_of::<u8>()),
//        );
//    }
//    _163 = malloc(::std::mem::size_of::<i16>().wrapping_mul((self.purara as (usize)).wrapping_add(_153)))
//        as (*mut i16);
//    if !_163.is_null() {
//        memset(
//            _163 as (*mut ::std::os::raw::c_void),
//            0i32,
//            (self.purara as (usize))
//                .wrapping_add(_153)
//                .wrapping_mul(::std::mem::size_of::<i16>()),
//        );
//    }
//    _164 = malloc(::std::mem::size_of::<i16>().wrapping_mul(self.purara as (usize))) as (*mut i16);
//    if !_164.is_null() {
//        memset(
//            _164 as (*mut ::std::os::raw::c_void),
//            0i32,
//            (self.purara as (usize)).wrapping_mul(::std::mem::size_of::<i16>()),
//        );
//    }
//    self.tutahi = malloc(::std::mem::size_of::<u8>().wrapping_mul(_155)) as (*mut u8);
//    if !self.tutahi.is_null() {
//        memset(
//            self.tutahi as (*mut ::std::os::raw::c_void),
//            0i32,
//            _155.wrapping_mul(::std::mem::size_of::<u8>()),
//        );
//    }
//    self.renana = malloc(::std::mem::size_of::<u8>().wrapping_mul(_156)) as (*mut u8);
//    if !self.renana.is_null() {
//        memset(
//            self.renana as (*mut ::std::os::raw::c_void),
//            0i32,
//            _156.wrapping_mul(::std::mem::size_of::<u8>()),
//        );
//    }
//    self.puri = malloc(
//        ::std::mem::size_of::<u16>().wrapping_mul(2usize.wrapping_mul(RAWEKE).wrapping_sub(1usize)),
//    ) as (*mut u16);
//    if !self.puri.is_null() {
//        memset(
//            self.puri as (*mut ::std::os::raw::c_void),
//            0i32,
//            2usize
//                .wrapping_mul(RAWEKE)
//                .wrapping_sub(1usize)
//                .wrapping_mul(::std::mem::size_of::<u16>()),
//        );
//    }
//    self.katote = malloc(
//        ::std::mem::size_of::<u16>().wrapping_mul(2usize.wrapping_mul(RAWEKE).wrapping_sub(1usize)),
//    ) as (*mut u16);
//    if !self.katote.is_null() {
//        memset(
//            self.katote as (*mut ::std::os::raw::c_void),
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
//    self.maruka = malloc(::std::mem::size_of::<u8>().wrapping_mul(RAWEKE)) as (*mut u8);
//    if !self.maruka.is_null() {
//        memset(
//            self.maruka as (*mut ::std::os::raw::c_void),
//            0i32,
//            RAWEKE.wrapping_mul(::std::mem::size_of::<u8>()),
//        );
//    }
//    self.herepo = malloc(
//        ::std::mem::size_of::<u16>().wrapping_mul(2usize.wrapping_mul(RAWEKE).wrapping_sub(1usize)),
//    ) as (*mut u16);
//    if !self.herepo.is_null() {
//        memset(
//            self.herepo as (*mut ::std::os::raw::c_void),
//            0i32,
//            2usize
//                .wrapping_mul(RAWEKE)
//                .wrapping_sub(1usize)
//                .wrapping_mul(::std::mem::size_of::<u16>()),
//        );
//    }
//    self.hokota = malloc(::std::mem::size_of::<u16>().wrapping_mul(RAWEKE)) as (*mut u16);
//    if !self.hokota.is_null() {
//        memset(
//            self.hokota as (*mut ::std::os::raw::c_void),
//            0i32,
//            RAWEKE.wrapping_mul(::std::mem::size_of::<u16>()),
//        );
//    }
//    self.piapia = malloc(::std::mem::size_of::<u8>().wrapping_mul(_152 as (usize))) as (*mut u8);
//    if !self.piapia.is_null() {
//        memset(
//            self.piapia as (*mut ::std::os::raw::c_void),
//            0i32,
//            (_152 as (usize)).wrapping_mul(::std::mem::size_of::<u8>()),
//        );
//    }
//    self.paerua = malloc(
//        ::std::mem::size_of::<u16>().wrapping_mul((2i32 * _142 as (i32) - 1i32) as (usize)),
//    ) as (*mut u16);
//    if !self.paerua.is_null() {
//        memset(
//            self.paerua as (*mut ::std::os::raw::c_void),
//            0i32,
//            ((2i32 * _142 as (i32) - 1i32) as (usize)).wrapping_mul(::std::mem::size_of::<u16>()),
//        );
//    }
//    self.hapoko = malloc(::std::mem::size_of::<u16>().wrapping_mul(_152 as (usize))) as (*mut u16);
//    if !self.hapoko.is_null() {
//        memset(
//            self.hapoko as (*mut ::std::os::raw::c_void),
//            0i32,
//            (_152 as (usize)).wrapping_mul(::std::mem::size_of::<u16>()),
//        );
//    }
//    if _166.is_null() || _163.is_null() || _164.is_null() || self.tutahi.is_null() || self.renana.is_null()
//        || self.puri.is_null() || self.katote.is_null() || _177.is_null() || self.maruka.is_null()
//        || self.herepo.is_null() || self.hokota.is_null() || self.piapia.is_null() || self.paerua.is_null()
//        || self.hapoko.is_null()
//    {
//        mStatus = -1i32;
//    }
//    self.wataka = 0usize;
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
//    _279 = self.purara;
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
//            _453 = (_453 as (i32) + self.purara as (i32)) as (i16);
//        }
//        if _453 as (i32) >= self.purara as (i32) {
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
//        self.komiti = (self.komiti as (i32) >> 1i32) as (u16);
//        self.komiti
//    } as (i32) == 0i32
//    {
//        self.komiti = (1u32 << 8i32 - 1i32) as (u16);
//        if self.piwari as (i32) >= self.whawha as (i32) {
//            husCompress_207();
//            if self.tirehe != 0 {
//                return;
//            } else {
//                self.piwari = 0u16;
//            }
//        }
//        self.puketutu = {
//            let _old = self.piwari;
//            self.piwari = (self.piwari as (i32) + 1) as (u16);
//            _old
//        };
//        *self.tutahi.offset(self.puketutu as (isize)) = 0u8;
//    }
//    *self.tutahi.offset({
//        let _old = self.piwari;
//        self.piwari = (self.piwari as (i32) + 1) as (u16);
//        _old
//    } as (isize)) = _203 as (u8);
//    let _rhs = 1;
//    let _lhs = &mut *self.herepo.offset(_203 as (isize));
//    *_lhs = (*_lhs as (i32) + _rhs) as (u16);
//    if _203 as (u32) >= 1u32 << 8i32 {
//        let _rhs = self.komiti as (u8);
//        let _lhs = &mut *self.tutahi.offset(self.puketutu as (isize));
//        *_lhs = (*_lhs as (i32) | _rhs as (i32)) as (u8);
//        *self.tutahi.offset({
//            let _old = self.piwari;
//            self.piwari = (self.piwari as (i32) + 1) as (u16);
//            _old
//        } as (isize)) = _204 as (u8);
//        *self.tutahi.offset({
//            let _old = self.piwari;
//            self.piwari = (self.piwari as (i32) + 1) as (u16);
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
//        let _lhs = &mut *self.paerua.offset(_203 as (isize));
//        *_lhs = (*_lhs as (i32) + _rhs) as (u16);
//    }
//}

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
//    _229 = husCompress_211(RAWEKE as (i32), self.herepo, self.maruka, self.hokota) as (u32);
//    _455 = *self.herepo.offset(_229 as (isize)) as (u32);
//    husCompress_208(16i32, _455 as (u16));
//    if _229 as (usize) >= RAWEKE {
//        husCompress_216(_217.as_mut_ptr());
//        _229 = husCompress_211(_145 as (i32), _217.as_mut_ptr(), self.piapia, self.hapoko) as (u32);
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
//    _229 = husCompress_211(_142 as (i32), self.paerua, self.piapia, self.hapoko) as (u32);
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
//            _456 = *self.tutahi.offset({
//                let _old = _454;
//                _454 = _454.wrapping_add(1u32);
//                _old
//            } as (isize)) as (u32);
//        } else {
//            _456 = _456 << 1i32;
//        }
//        if _456 & 1u32 << 8i32 - 1i32 != 0 {
//            husCompress_223((*self.tutahi.offset({
//                let _old = _454;
//                _454 = _454.wrapping_add(1u32);
//                _old
//            } as (isize)) as (u32))
//                .wrapping_add(1u32 << 8i32) as (i16));
//            _289 = *self.tutahi.offset({
//                let _old = _454;
//                _454 = _454.wrapping_add(1u32);
//                _old
//            } as (isize)) as (u32);
//            _289 = _289.wrapping_add(
//                (*self.tutahi.offset({
//                    let _old = _454;
//                    _454 = _454.wrapping_add(1u32);
//                    _old
//                } as (isize)) as (i32) << 8i32) as (u32),
//            );
//            self.moruki(_289 as (i16) as (u16));
//        } else {
//            husCompress_223(*self.tutahi.offset({
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
//            *self.herepo.offset(_226 as (isize)) = 0u16;
//            _226 = _226.wrapping_add(1u32);
//        }
//        _226 = 0u32;
//        'loop14: loop {
//            if !(_226 < _142 as (u32)) {
//                break;
//            }
//            *self.paerua.offset(_226 as (isize)) = 0u16;
//            _226 = _226.wrapping_add(1u32);
//        }
//    }
//}
//
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
//            self.orawia(_226, _187, _177, _227);
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
//            self.orawia(1i32, _187, _177, _227);
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
//            self.orawia(1i32, _187, _177, _227);
//            *self.puri.offset(_289 as (isize)) = _226 as (u16);
//            *self.katote.offset(_289 as (isize)) = _276 as (u16);
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
//            && (*self.maruka.offset((_219 as (i32) - 1i32) as (isize)) as (i32) == 0i32))
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
//        _289 = *self.maruka.offset({
//            let _old = _226;
//            _226 = (_226 as (i32) + 1) as (i16);
//            _old
//        } as (isize)) as (i16);
//        if _289 as (i32) == 0i32 {
//            _277 = 1i16;
//            'loop10: loop {
//                if !(_226 as (i32) < _219 as (i32)
//                    && (*self.maruka.offset(_226 as (isize)) as (i32) == 0i32))
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
//            && (*self.piapia.offset((_219 as (i32) - 1i32) as (isize)) as (i32) == 0i32))
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
//        _289 = *self.piapia.offset({
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
//            if !(_226 as (i32) < 6i32 && (*self.piapia.offset(_226 as (isize)) as (i32) == 0i32)) {
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
//            && (*self.maruka.offset((_219 as (i32) - 1i32) as (isize)) as (i32) == 0i32))
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
//        _289 = *self.maruka.offset({
//            let _old = _226;
//            _226 = (_226 as (i32) + 1) as (i16);
//            _old
//        } as (isize)) as (i16);
//        if _289 as (i32) == 0i32 {
//            _277 = 1i16;
//            'loop8: loop {
//                if !(_226 as (i32) < _219 as (i32)
//                    && (*self.maruka.offset(_226 as (isize)) as (i32) == 0i32))
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
//                    husCompress_208(*self.piapia.offset(0isize) as (i32), *self.hapoko.offset(0isize));
//                    _289 = (_289 as (i32) + 1) as (i16);
//                }
//            } else if _277 as (i32) <= 18i32 {
//                husCompress_208(*self.piapia.offset(1isize) as (i32), *self.hapoko.offset(1isize));
//                husCompress_208(4i32, (_277 as (i32) - 3i32) as (u16));
//            } else if _277 as (i32) == 19i32 {
//                husCompress_208(*self.piapia.offset(0isize) as (i32), *self.hapoko.offset(0isize));
//                husCompress_208(*self.piapia.offset(1isize) as (i32), *self.hapoko.offset(1isize));
//                husCompress_208(4i32, 15u16);
//            } else {
//                husCompress_208(*self.piapia.offset(2isize) as (i32), *self.hapoko.offset(2isize));
//                husCompress_208(_143 as (i32), (_277 as (i32) - 20i32) as (u16));
//            }
//        } else {
//            husCompress_208(
//                *self.piapia.offset((_289 as (i32) + 2i32) as (isize)) as (i32),
//                *self.hapoko.offset((_289 as (i32) + 2i32) as (isize)),
//            );
//        }
//    }
//}