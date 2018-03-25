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

use formats::husqvarna::compress::constants;
use formats::husqvarna::compress::constants::{CONST_135, CONST_140, CONST_142, CONST_143,
                                              CONST_144, CONST_145, CONST_147, CONST_148,
                                              CONST_149, CONST_152, CONST_540, AHIAHI_LOWER_LIMIT,
                                              AHIAHI_UPPER_LIMIT, BUFFER_SIZE, BYTE_MAX};

pub const _141: usize = constants::RAWEKE;


struct ExpandData {
    tieaka: *mut i16,
    mamare: *mut i16,
    puaroa: *mut u8,
    ahakoa: *mut u8,
    maouru: [u16; 17],
    tarahe: i16,
    kowaha: i16,
    puhore: i16,
    uruhua: i16,
    mahoro: i16,
    omania: i16,
    paramu: i16,
    kohinu: i16,
    minita: i16,
    maheke: *mut i16,
    manuea: *mut u8,
    tomata: *mut u8,
    taewai: *mut u8,
    hoihoi: *mut u8,
    itinga: u16,
    papatu: u16,
    hinohi: u16,
    pokiwa: u16,
    tutohi: u16,
    puhura: *mut u16,
    moetia: *mut u16,
    kapura: *mut u16,
    ruatea: *mut u16,
    pararo: *mut u16,
    pekeri: *mut u16,
    koroau: *mut u16,
    pahara: *mut u16,
    kotoko: *mut u16,
    topiki: *mut u16,
    pokana: i16,
    nepata: u16,
    hanene: u8,
    keneti: i16,
    patoka: i32,
    hiropi: usize,
    m_status: i32,
    current_index: i32,
    remaining_bytes: isize,
    input_size: usize,
    input_buffer_size: usize,
    input_length: usize,
    output_array: *mut u8,
    input_array: *mut u8,
    input_buffer: *mut u8,
    current_position: usize,
    input_position: usize,
    output_position: usize,
}

impl ExpandData {
    fn new(factor: i32) -> ExpandData {
        let mut m_status: i32 = 0;
        let mut kohinu: i16;
        if factor > AHIAHI_UPPER_LIMIT as (i32) || factor < AHIAHI_LOWER_LIMIT as (i32) {
            m_status = -1;
            kohinu = 2;
        } else {
            kohinu = (1 << factor);
        }

        ExpandData {
            tieaka: 0 as *mut i16,
            mamare: 0 as *mut i16,
            puaroa: 0 as *mut u8,
            ahakoa: 0 as *mut u8,
            maouru: [0; 17],
            tarahe: 0,
            kowaha: 0,
            puhore: 0,
            uruhua: 0,
            mahoro: 0,
            omania: 0,
            paramu: 0,
            kohinu,
            minita: 0,
            maheke: 0 as *mut i16,
            manuea: 0 as *mut u8,
            tomata: 0 as *mut u8,
            taewai: 0 as *mut u8,
            hoihoi: 0 as *mut u8,
            itinga: 0,
            papatu: 0,
            hinohi: 0,
            pokiwa: 0,
            tutohi: 0,
            puhura: 0 as *mut u16,
            moetia: 0 as *mut u16,
            kapura: 0 as *mut u16,
            ruatea: 0 as *mut u16,
            pararo: 0 as *mut u16,
            pekeri: 0 as *mut u16,
            koroau: 0 as *mut u16,
            pahara: 0 as *mut u16,
            kotoko: 0 as *mut u16,
            topiki: 0 as *mut u16,
            pokana: 0,
            nepata: 0,
            hanene: 0,
            keneti: 0,
            patoka: 0,
            hiropi: 0,
            m_status,
            current_index: 0,
            remaining_bytes: 0,
            input_size: 0,
            input_buffer_size: BUFFER_SIZE,
            input_length: 0,
            output_array: 0 as *mut u8,
            input_array: 0 as *mut u8,
            input_buffer: 0 as *mut u8,
            current_position: 0,
            input_position: 0,
            output_position: 0,
        }
    }

    unsafe fn husExpand(
        &mut self,
        mut input: *mut u8,
        mut output: *mut u8,
        mut compressed_size: i32,
        mut factor: i32,
    ) {
        self.output_array = output;
        self.input_array = input;
        self.input_size = BUFFER_SIZE;
        self.remaining_bytes = compressed_size as (isize);
        self.minita = (self.kohinu as (i32) - 1) as (i16);
        self.ahakoa = malloc(
            ::std::mem::size_of::<u8>().wrapping_mul((self.kohinu as (i32) + 2) as (usize)),
        ) as (*mut u8);
        if !self.ahakoa.is_null() {
            memset(
                self.ahakoa as (*mut ::std::os::raw::c_void),
                0,
                ((self.kohinu as (i32) + 2) as (usize)).wrapping_mul(::std::mem::size_of::<u8>()),
            );
        }
        self.kotoko = malloc(::std::mem::size_of::<u16>().wrapping_mul(CONST_148)) as (*mut u16);
        if !self.kotoko.is_null() {
            memset(
                self.kotoko as (*mut ::std::os::raw::c_void),
                0,
                CONST_148.wrapping_mul(::std::mem::size_of::<u16>()),
            );
        }
        self.topiki = malloc(::std::mem::size_of::<u16>().wrapping_mul(CONST_149)) as (*mut u16);
        if !self.topiki.is_null() {
            memset(
                self.topiki as (*mut ::std::os::raw::c_void),
                0,
                CONST_149.wrapping_mul(::std::mem::size_of::<u16>()),
            );
        }
        self.kapura = malloc(
            ::std::mem::size_of::<u16>().wrapping_mul(2usize.wrapping_mul(_141).wrapping_sub(1)),
        ) as (*mut u16);
        if !self.kapura.is_null() {
            memset(
                self.kapura as (*mut ::std::os::raw::c_void),
                0,
                2usize
                    .wrapping_mul(_141)
                    .wrapping_sub(1)
                    .wrapping_mul(::std::mem::size_of::<u16>()),
            );
        }
        self.ruatea = malloc(
            ::std::mem::size_of::<u16>().wrapping_mul(2usize.wrapping_mul(_141).wrapping_sub(1)),
        ) as (*mut u16);
        if !self.ruatea.is_null() {
            memset(
                self.ruatea as (*mut ::std::os::raw::c_void),
                0,
                2usize
                    .wrapping_mul(_141)
                    .wrapping_sub(1)
                    .wrapping_mul(::std::mem::size_of::<u16>()),
            );
        }
        self.taewai = malloc(::std::mem::size_of::<u8>().wrapping_mul(_141)) as (*mut u8);
        if !self.taewai.is_null() {
            memset(
                self.taewai as (*mut ::std::os::raw::c_void),
                0,
                _141.wrapping_mul(::std::mem::size_of::<u8>()),
            );
        }
        self.hoihoi =
            malloc(::std::mem::size_of::<u8>().wrapping_mul(CONST_152 as (usize))) as (*mut u8);
        if !self.hoihoi.is_null() {
            memset(
                self.hoihoi as (*mut ::std::os::raw::c_void),
                0,
                (CONST_152 as (usize)).wrapping_mul(::std::mem::size_of::<u8>()),
            );
        }
        if self.ahakoa == 0 as (*mut ::std::os::raw::c_void) as (*mut u8)
            || self.kapura == 0 as (*mut ::std::os::raw::c_void) as (*mut u16)
            || self.ruatea == 0 as (*mut ::std::os::raw::c_void) as (*mut u16)
            || self.taewai == 0 as (*mut ::std::os::raw::c_void) as (*mut u8)
            || self.hoihoi == 0 as (*mut ::std::os::raw::c_void) as (*mut u8)
            || self.kotoko == 0 as (*mut ::std::os::raw::c_void) as (*mut u16)
            || self.topiki == 0 as (*mut ::std::os::raw::c_void) as (*mut u16)
            || self.input_buffer == 0 as (*mut ::std::os::raw::c_void) as (*mut u8)
        {
            self.m_status = -1;
        }
        self.husExpand_expand();
        self.husExpand_cleanup();
    }

    unsafe fn husExpand_cleanup(&mut self) {
        free(self.ahakoa as (*mut ::std::os::raw::c_void));
        free(self.kapura as (*mut ::std::os::raw::c_void));
        free(self.ruatea as (*mut ::std::os::raw::c_void));
        free(self.taewai as (*mut ::std::os::raw::c_void));
        free(self.hoihoi as (*mut ::std::os::raw::c_void));
        free(self.kotoko as (*mut ::std::os::raw::c_void));
        free(self.topiki as (*mut ::std::os::raw::c_void));
    }

    unsafe fn husExpand_253(&mut self, mut _254: i16, mut _220: i16, mut _221: i16) {
        let mut _226: i16;
        let mut _203: i16;
        let mut _219: i16;
        let mut _283: u16;
        _219 = self.husExpand_252(_220 as (i32)) as (i16);
        if _219 as (i32) == 0 {
            _203 = self.husExpand_252(_220 as (i32)) as (i16);
            _226 = 0;
            'loop15: loop {
                if !(_226 as (i32) < _254 as (i32)) {
                    break;
                }
                *self.hoihoi.offset(_226 as (isize)) = 0;
                _226 = (_226 as (i32) + 1) as (i16);
            }
            _226 = 0;
            'loop17: loop {
                if !(_226 as (i32) < 256) {
                    break;
                }
                *self.topiki.offset(_226 as (isize)) = _203 as (u16);
                _226 = (_226 as (i32) + 1) as (i16);
            }
        } else {
            _226 = 0;
            'loop2: loop {
                if !(_226 as (i32) < _219 as (i32)) {
                    break;
                }
                _203 = (self.itinga as (i32) >> 13) as (i16);
                if _203 as (i32) == 7 {
                    _283 = (1 << 12) as (u16);
                    'loop8: loop {
                        if _283 as (i32) & self.itinga as (i32) == 0 {
                            break;
                        }
                        _283 = (_283 as (i32) >> 1) as (u16);
                        _203 = (_203 as (i32) + 1) as (i16);
                    }
                }
                self.husExpand_256(if _203 as (i32) < 7 {
                    3
                } else {
                    _203 as (i32) - 3
                });
                *self.hoihoi.offset({
                    let _old = _226;
                    _226 = (_226 as (i32) + 1) as (i16);
                    _old
                } as (isize)) = _203 as (u8);
                if !(_226 as (i32) == _221 as (i32)) {
                    continue;
                }
                _203 = self.husExpand_252(2) as (i16);
                'loop11: loop {
                    if !({
                        _203 = (_203 as (i32) - 1) as (i16);
                        _203
                    } as (i32) >= 0)
                    {
                        break;
                    }
                    *self.hoihoi.offset({
                        let _old = _226;
                        _226 = (_226 as (i32) + 1) as (i16);
                        _old
                    } as (isize)) = 0;
                }
            }
            'loop3: loop {
                if !(_226 as (i32) < _254 as (i32)) {
                    break;
                }
                *self.hoihoi.offset({
                    let _old = _226;
                    _226 = (_226 as (i32) + 1) as (i16);
                    _old
                } as (isize)) = 0;
            }
            let b = self.topiki;
            let a = self.hoihoi;
            self.husExpand_258(_254 as (i32), a, 8, b, CONST_149 as (u16));
        }
    }

    unsafe fn husExpand_expand(&mut self) -> i32 {
        let mut _currentBlock;
        let mut _200: i16 = 0;
        let mut _278: *mut u8 = self.ahakoa;
        let mut _279: i16 = self.kohinu;
        let mut _280: i16 = self.minita;
        self.pokana = 0;
        self.husExpand_251();
        'loop1: loop {
            if !(self.pokana as (i32) < 5) {
                break;
            }
            let mut _203: i16;
            if {
                _203 = self.husExpand_249() as (i16);
                _203
            } as (usize) <= BYTE_MAX
            {
                *_278.offset(_200 as (isize)) = _203 as (u8);
                if !({
                    _200 = (_200 as (i32) + 1) as (i16);
                    _200
                } as (i32) >= _279 as (i32))
                {
                    continue;
                }
                _200 = 0;
                memcpy(
                    &mut *self.output_array.offset(self.output_position as (isize)) as (*mut u8)
                        as (*mut ::std::os::raw::c_void),
                    _278 as (*const ::std::os::raw::c_void),
                    _279 as (usize),
                );
                self.output_position = self.output_position + _279 as (usize);
            } else {
                let mut _226: i16;
                let mut _276: i16 = (_203 as (usize))
                    .wrapping_sub(BYTE_MAX.wrapping_add(1).wrapping_sub(CONST_135 as (usize)))
                    as (i16);
                if _276 as (usize) == CONST_144 {
                    break;
                }
                _226 = (_200 as (i32) - self.husExpand_250() as (i32) - 1 & _280 as (i32)) as (i16);
                if _226 as (usize) < (_279 as (usize)).wrapping_sub(CONST_140).wrapping_sub(1)
                    && (_200 as (usize) < (_279 as (usize)).wrapping_sub(CONST_140).wrapping_sub(1))
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
                        } as (i32) >= 0)
                        {
                            break;
                        }
                        *_278.offset(_200 as (isize)) = *_278.offset(_226 as (isize));
                        if {
                            _200 = (_200 as (i32) + 1) as (i16);
                            _200
                        } as (i32) >= _279 as (i32)
                        {
                            _200 = 0;
                            memcpy(
                                &mut *self.output_array.offset(self.output_position as (isize))
                                    as (*mut u8)
                                    as (*mut ::std::os::raw::c_void),
                                _278 as (*const ::std::os::raw::c_void),
                                _279 as (usize),
                            );
                            self.output_position = self.output_position + _279 as (usize);
                        }
                        _226 = (_226 as (i32) + 1 & _280 as (i32)) as (i16);
                        _currentBlock = 5;
                    } else {
                        if !({
                            _276 = (_276 as (i32) - 1) as (i16);
                            _276
                        } as (i32) >= 0)
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
        if _200 as (i32) != 0 {
            memcpy(
                &mut *self.output_array.offset(self.output_position as (isize)) as (*mut u8)
                    as (*mut ::std::os::raw::c_void),
                _278 as (*const ::std::os::raw::c_void),
                _200 as (usize),
            );
            self.output_position = self.output_position + _200 as (usize);
        }
        0
    }

    unsafe fn husExpand_249(&mut self) -> u16 {
        let mut _276: u16;
        let mut _283: u16;
        if self.nepata as (i32) == 0 {
            self.nepata = self.husExpand_252(16);
            self.husExpand_253(CONST_145, CONST_147, 3);
            self.husExpand_255();
            self.husExpand_253(CONST_142, CONST_540, -1);
            if self.m_status < 0 {
                return 0;
            }
        }
        self.nepata = (self.nepata as (i32) - 1) as (u16);
        _276 = *self.kotoko.offset((self.itinga as (i32) >> 4) as (isize));
        if _276 as (usize) >= _141 {
            _283 = (1 << 3) as (u16);
            'loop4: loop {
                if self.itinga as (i32) & _283 as (i32) != 0 {
                    _276 = *self.ruatea.offset(_276 as (isize));
                } else {
                    _276 = *self.kapura.offset(_276 as (isize));
                }
                _283 = (_283 as (i32) >> 1) as (u16);
                if !(_276 as (usize) >= _141) {
                    break;
                }
            }
        }
        let a = *self.taewai.offset(_276 as (isize)) as (i32);
        self.husExpand_256(a);
        _276
    }

    unsafe fn husExpand_250(&mut self) -> u16 {
        let mut _276: u16;
        let mut _283: u16;
        _276 = *self.topiki.offset((self.itinga as (i32) >> 8) as (isize));
        if _276 as (i32) >= CONST_142 as (i32) {
            _283 = (1 << 7) as (u16);
            'loop2: loop {
                if self.itinga as (i32) & _283 as (i32) != 0 {
                    _276 = *self.ruatea.offset(_276 as (isize));
                } else {
                    _276 = *self.kapura.offset(_276 as (isize));
                }
                _283 = (_283 as (i32) >> 1) as (u16);
                if !(_276 as (i32) >= CONST_142 as (i32)) {
                    break;
                }
            }
        }
        let a = *self.hoihoi.offset(_276 as (isize)) as (i32);
        self.husExpand_256(a);
        if _276 as (i32) != 0 {
            _276 = (_276 as (i32) - 1) as (u16);
            _276 = (1u32 << _276 as (i32)).wrapping_add(self.husExpand_252(_276 as (i32)) as (u32))
                as (i16) as (u16);
        }
        _276
    }

    unsafe fn husExpand_251(&mut self) {
        self.nepata = 0;
        self.husExpand_257();
    }

    unsafe fn husExpand_252(&mut self, _219: i32) -> u16 {
        let mut _284: u16 = (self.itinga as (i32) >> 16 - _219) as (u16);
        self.husExpand_256(_219);
        _284
    }

    unsafe fn husExpand_255(&mut self) {
        let mut _226: i16;
        let mut _203: i16;
        let mut _219: i16 = self.husExpand_252(CONST_143 as (i32)) as (i16);
        if _219 as (i32) == 0 {
            _203 = self.husExpand_252(CONST_143 as (i32)) as (i16);
            _226 = 0;
            'loop23: loop {
                if !(_226 as (usize) < _141) {
                    break;
                }
                *self.taewai.offset(_226 as (isize)) = 0;
                _226 = (_226 as (i32) + 1) as (i16);
            }
            _226 = 0;
            'loop25: loop {
                if !(_226 as (usize) < CONST_148) {
                    break;
                }
                *self.kotoko.offset(_226 as (isize)) = _203 as (u16);
                _226 = (_226 as (i32) + 1) as (i16);
            }
        } else {
            _226 = 0;
            'loop2: loop {
                if !(_226 as (i32) < _219 as (i32)) {
                    break;
                }
                _203 = *self.topiki.offset((self.itinga as (i32) >> 8) as (isize)) as (i16);
                if _203 as (i32) >= CONST_145 as (i32) {
                    let mut _283: u16 = (1 << 7) as (u16);
                    'loop8: loop {
                        if self.itinga as (i32) & _283 as (i32) != 0 {
                            _203 = *self.ruatea.offset(_203 as (isize)) as (i16);
                        } else {
                            _203 = *self.kapura.offset(_203 as (isize)) as (i16);
                        }
                        _283 = (_283 as (i32) >> 1) as (u16);
                        if !(_203 as (i32) >= CONST_145 as (i32)) {
                            break;
                        }
                    }
                }
                let a = *self.hoihoi.offset(_203 as (isize)) as (i32);
                self.husExpand_256(a);
                if _203 as (i32) <= 2 {
                    if _203 as (i32) == 0 {
                        _203 = 1;
                    } else if _203 as (i32) == 1 {
                        _203 = (self.husExpand_252(4) as (i32) + 3) as (i16);
                    } else {
                        _203 = (self.husExpand_252(CONST_143 as (i32)) as (i32) + 20) as (i16);
                    }
                    'loop20: loop {
                        if !({
                            _203 = (_203 as (i32) - 1) as (i16);
                            _203
                        } as (i32) >= 0)
                        {
                            break;
                        }
                        *self.taewai.offset({
                            let _old = _226;
                            _226 = (_226 as (i32) + 1) as (i16);
                            _old
                        } as (isize)) = 0;
                    }
                } else {
                    *self.taewai.offset({
                        let _old = _226;
                        _226 = (_226 as (i32) + 1) as (i16);
                        _old
                    } as (isize)) = (_203 as (i32) - 2) as (u8);
                }
            }
            'loop3: loop {
                if !(_226 as (usize) < _141) {
                    break;
                }
                *self.taewai.offset({
                    let _old = _226;
                    _226 = (_226 as (i32) + 1) as (i16);
                    _old
                } as (isize)) = 0;
            }
            let a = self.taewai;
            let b = self.kotoko;
            self.husExpand_258(_141 as (i32), a, 12, b, CONST_148 as (u16));
        }
    }

    unsafe fn husExpand_256(&mut self, mut _219: i32) {
        'loop0: loop {
            if !(_219 > self.mahoro as (i32)) {
                break;
            }
            _219 = _219 - self.mahoro as (i32);
            self.itinga = ((self.itinga as (i32) << self.mahoro as (i32))
                + (self.hanene as (i32) >> 8 - self.mahoro as (i32)))
                as (u16);
            if self.keneti as (i32) <= 0 {
                self.current_index = 0;
                if self.remaining_bytes >= 0 && (self.remaining_bytes as (usize) < BUFFER_SIZE) {
                    self.input_buffer = &mut *self.input_array
                        .offset(self.current_position as (isize))
                        as (*mut u8);
                    self.current_position =
                        (self.current_position as (isize) + self.remaining_bytes) as (usize);
                    self.keneti = self.remaining_bytes as (i16);
                    self.remaining_bytes = self.remaining_bytes - self.keneti as (isize);
                    self.input_buffer_size = self.keneti as (usize);
                } else {
                    self.input_buffer = &mut *self.input_array
                        .offset(self.current_position as (isize))
                        as (*mut u8);
                    self.current_position =
                        (self.current_position as (usize)).wrapping_add(BUFFER_SIZE) as (usize);
                    self.keneti = BUFFER_SIZE as (i16);
                    self.input_buffer_size = self.keneti as (usize);
                }
                if self.keneti as (i32) <= 0 {
                    self.pokana = (self.pokana as (i32) + 1) as (i16);
                }
            }
            self.hanene = *self.input_buffer.offset({
                let _old = self.current_index;
                self.current_index = self.current_index + 1;
                _old
            } as (isize));
            self.keneti = (self.keneti as (i32) - 1) as (i16);
            self.mahoro = 8;
        }
        self.mahoro = (self.mahoro as (i32) - _219) as (i16);
        self.itinga =
            ((self.itinga as (i32) << _219) + (self.hanene as (i32) >> 8 - _219)) as (u16);
        self.hanene = (self.hanene as (i32) << _219) as (u8);
    }

    unsafe fn husExpand_257(&mut self) {
        self.itinga = 0;
        self.hanene = 0;
        self.mahoro = 0;
        self.keneti = 0;
        self.husExpand_256(16);
    }

    unsafe fn husExpand_258(
        &mut self,
        mut _259: i32,
        mut _260: *mut u8,
        mut _261: i32,
        mut _262: *mut u16,
        mut _263: u16,
    ) {
        let mut _currentBlock;
        let mut _277: [u16; 17] = [0; 17];
        let mut _287: [u16; 17] = [0; 17];
        let mut _288: [u16; 18] = [0; 18];
        let mut _204: *mut u16;
        let mut _226: u32;
        let mut _289: u32;
        let mut _209: u32;
        let mut _290: u32;
        let mut _291: u32;
        let mut _292: u32;
        let mut _293: u32;
        let mut _283: u32;
        _288[0] = 0;
        _277[0] = 0;
        _287[0] = 0;
        _226 = 1;
        'loop1: loop {
            if !(_226 <= 16) {
                break;
            }
            _277[_226 as (usize)] = 0;
            _226 = _226.wrapping_add(1);
        }
        _226 = 0;
        'loop3: loop {
            if !(_226 as (i32) < _259) {
                break;
            }
            let _rhs = 1;
            let _lhs = &mut _277[*_260.offset(_226 as (isize)) as (usize)];
            *_lhs = (*_lhs as (i32) + _rhs) as (u16);
            _226 = _226.wrapping_add(1);
        }
        _288[1] = 0;
        _226 = 1;
        'loop5: loop {
            if !(_226 <= 16) {
                break;
            }
            _288[_226.wrapping_add(1) as (usize)] = (_288[_226 as (usize)] as (i32)
                + (_277[_226 as (usize)] as (i32) << 16u32.wrapping_sub(_226)))
                as (u16);
            _226 = _226.wrapping_add(1);
        }
        if _288[17] as (i32) != (1 << 16) as (u16) as (i32) {
            self.m_status = -1;
            self.pokana = 10;
        } else {
            _291 = (16 - _261) as (u32);
            _226 = 1;
            'loop8: loop {
                if !(_226 as (i32) <= _261) {
                    break;
                }
                let _rhs = _291;
                let _lhs = &mut _288[_226 as (usize)];
                *_lhs = (*_lhs as (i32) >> _rhs) as (u16);
                _287[_226 as (usize)] = (1 << (_261 as (u32)).wrapping_sub(_226)) as (u16);
                _226 = _226.wrapping_add(1);
            }
            'loop9: loop {
                if !(_226 <= 16) {
                    break;
                }
                _287[_226 as (usize)] = (1 << 16u32.wrapping_sub(_226)) as (u16);
                _226 = _226.wrapping_add(1);
            }
            _226 = (_288[(_261 + 1) as (usize)] as (i32) >> _291) as (u32);
            if _226 != (1 << 16) as (u16) as (u32) {
                _289 = 1 << _261;
                'loop12: loop {
                    if !(_226 != _289) {
                        break;
                    }
                    *_262.offset({
                        let _old = _226;
                        _226 = _226.wrapping_add(1);
                        _old
                    } as (isize)) = 0;
                }
            }
            _292 = _259 as (u32);
            _283 = 1 << 15 - _261;
            _290 = 0;
            'loop14: loop {
                if !(_290 as (i32) < _259) {
                    _currentBlock = 15;
                    break;
                }
                if !({
                    _209 = *_260.offset(_290 as (isize)) as (u32);
                    _209
                } == 0)
                {
                    _293 =
                        (_288[_209 as (usize)] as (i32) + _287[_209 as (usize)] as (i32)) as (u32);
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
                            _226 = _226.wrapping_add(1);
                        }
                    } else {
                        _289 = _288[_209 as (usize)] as (u32);
                        _204 = &mut *_262.offset((_289 >> _291) as (isize)) as (*mut u16);
                        _226 = _209.wrapping_sub(_261 as (u32));
                        'loop19: loop {
                            if !(_226 != 0) {
                                break;
                            }
                            if *_204 as (i32) == 0 {
                                *self.ruatea.offset(_292 as (isize)) = {
                                    let _rhs = 0;
                                    let _lhs = &mut *self.kapura.offset(_292 as (isize));
                                    *_lhs = _rhs as (u16);
                                    *_lhs
                                };
                                *_204 = {
                                    let _old = _292;
                                    _292 = _292.wrapping_add(1);
                                    _old
                                } as (u16);
                            }
                            if _289 & _283 != 0 {
                                _204 = &mut *self.ruatea.offset(*_204 as (isize)) as (*mut u16);
                            } else {
                                _204 = &mut *self.kapura.offset(*_204 as (isize)) as (*mut u16);
                            }
                            _289 = _289 << 1;
                            _226 = _226.wrapping_sub(1);
                        }
                        *_204 = _290 as (u16);
                    }
                    _288[_209 as (usize)] = _293 as (u16);
                }
                _290 = _290.wrapping_add(1);
            }
            (if _currentBlock == 15 {
            } else {
                self.m_status = -1;
                self.pokana = 10;
            })
        }
    }
}
