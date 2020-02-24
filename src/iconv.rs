//! 修改自 https://github.com/andelf/rust-iconv/blob/master/src/lib.rs

extern crate libc;

use libc::{E2BIG, EILSEQ, EINVAL};
use std::{
    ffi::CString,
    io,
    io::{Read, Result, Write},
    iter, mem, ptr, str,
};

#[allow(dead_code)]
const DEFAULT_BUF_SIZE: usize = 64 * 1024;

use libc::{c_char, c_int, c_void, size_t};

#[allow(non_camel_case_types)]
type iconv_t = *mut c_void;

/// The representation of a iconv converter
pub(crate) struct Converter {
    cd: iconv_t,
}

impl Converter {
    /// Creates a new Converter from ``from`` encoding and ``to`` encoding.
    pub fn new(from: &str, to: &str) -> Converter {
        let from_encoding = CString::new(from).unwrap();
        let to_encoding = CString::new(to).unwrap();

        let handle = unsafe {
            libloading::Library::new("libiconv.dll").unwrap().get::<extern "C" fn(__tocode: *const c_char, __fromcode: *const c_char) -> iconv_t>("libiconv_open".as_bytes()).unwrap()(to_encoding.as_ptr(), from_encoding.as_ptr())
        };
        if handle as isize == -1 {
            panic!(
                "Error creating conversion descriptor from {:} to {:}",
                from, to
            );
        }
        Converter { cd: handle }
    }

    /// Convert from input into output.
    /// Returns (bytes_read, bytes_written, errno).
    pub fn convert(&self, input: &[u8], output: &mut [u8]) -> (usize, usize, c_int) {
        let input_left = input.len() as size_t;
        let output_left = output.len() as size_t;
        let lib = libloading::Library::new("libiconv.dll").unwrap();
        let iconv = unsafe {
            lib.get::<extern "C" fn(
                __cd: iconv_t,
                __inbuf: *mut *mut c_char,
                __inbytesleft: *mut size_t,
                __outbuf: *mut *mut c_char,
                __outbytesleft: *mut size_t,
            ) -> size_t>("libiconv".as_bytes())
                .unwrap()
        };

        if input_left > 0 && output_left > 0 {
            let input_ptr = input.as_ptr();
            let output_ptr = output.as_ptr();

            let ret = unsafe {
                iconv(
                    self.cd,
                    mem::transmute(&input_ptr),
                    mem::transmute(&input_left),
                    mem::transmute(&output_ptr),
                    mem::transmute(&output_left),
                )
            };
            let bytes_read = input.len() - input_left as usize;
            let bytes_written = output.len() - output_left as usize;

            return (
                bytes_read,
                bytes_written,
                if ret as isize == -1 {
                    io::Error::last_os_error().raw_os_error().unwrap() as c_int
                } else {
                    0
                },
            );
        } else if input_left == 0 && output_left > 0 {
            let output_ptr = output.as_ptr();

            let ret = unsafe {
                iconv(
                    self.cd,
                    ptr::null_mut::<*mut c_char>(),
                    mem::transmute(&input_left),
                    mem::transmute(&output_ptr),
                    mem::transmute(&output_left),
                )
            };

            let bytes_written = output.len() - output_left as usize;

            return (
                0,
                bytes_written,
                if ret as isize == -1 {
                    io::Error::last_os_error().raw_os_error().unwrap() as c_int
                } else {
                    0
                },
            );
        } else {
            let ret = unsafe {
                iconv(
                    self.cd,
                    ptr::null_mut::<*mut c_char>(),
                    mem::transmute(&input_left),
                    ptr::null_mut::<*mut c_char>(),
                    mem::transmute(&output_left),
                )
            };

            return (0, 0, ret as i32);
        }
    }
}

impl Drop for Converter {
    fn drop(&mut self) {
        unsafe {
            libloading::Library::new("libiconv.dll")
                .unwrap()
                .get::<extern "C" fn(__cd: iconv_t) -> c_int>("libiconv_close".as_bytes())
                .unwrap()(self.cd)
        };
    }
}

/// A ``Reader`` which does iconv convert from another Reader.
pub struct IconvReader<R> {
    inner: R,
    conv: Converter,
    buf: Vec<u8>,
    read_pos: usize,
    write_pos: usize,
    err: Option<io::Error>,
    tempbuf: Vec<u8>, // used when outbut is too small and can't make a single convertion
}

impl<R: Read> IconvReader<R> {
    #[allow(dead_code)]
    pub fn new(r: R, from: &str, to: &str) -> IconvReader<R> {
        let conv = Converter::new(from, to);
        let mut buf = Vec::with_capacity(DEFAULT_BUF_SIZE);
        buf.extend(iter::repeat(0).take(DEFAULT_BUF_SIZE));
        IconvReader {
            inner: r,
            conv: conv,
            buf: buf,
            read_pos: 0,
            write_pos: 0,
            err: None,
            tempbuf: Vec::new(), // small buf allocate dynamicly
        }
    }

    fn fill_buf(&mut self) {
        if self.read_pos > 0 {
            unsafe {
                ptr::copy::<u8>(
                    self.buf.as_mut_ptr(),
                    mem::transmute(&self.buf[self.read_pos]),
                    self.write_pos - self.read_pos,
                );
            }

            self.write_pos -= self.read_pos;
            self.read_pos = 0;
        }
        match self.inner.read(&mut self.buf[self.write_pos..]) {
            Ok(nread) => {
                self.write_pos += nread;
            },
            Err(e) => {
                self.err = Some(e);
            },
        }
    }
}

impl<R: Read> Read for IconvReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        if self.tempbuf.len() != 0 {
            let mut nwrite = 0;
            for slot in self.tempbuf.iter() {
                buf[nwrite] = *slot;
                nwrite += 1;
            }
            //let nwrite = buf.clone_from_slice(self.tempbuf.as_ref());
            if nwrite < self.tempbuf.len() {
                self.tempbuf = self.tempbuf[nwrite..].to_vec();
            } else {
                self.tempbuf = Vec::new();
            }
            return Ok(nwrite);
        }

        while self.write_pos == 0 || self.read_pos == self.write_pos {
            match self.err {
                Some(ref e) => {
                    return Err(io::Error::new(e.kind(), ""));
                },
                None => self.fill_buf(),
            }
        }

        let (nread, nwrite, err) = self
            .conv
            .convert(&self.buf[self.read_pos..self.write_pos], buf);

        self.read_pos += nread;

        match err {
            EILSEQ => {
                //debug!("An invalid multibyte sequence has been encountered in the input.");
                return Err(io::Error::new(io::ErrorKind::InvalidInput, ""));
            },
            EINVAL => {
                //debug!("An incomplete multibyte sequence has been encountered in the input.");
                // FIXME fill_buf() here is ugly
                self.fill_buf();
                return Ok(nwrite);
            },
            E2BIG => {
                //debug!("There is not sufficient room at *outbuf.");
                // FIXED: if outbuf buffer has size 1? Can't hold a
                if nread == 0 && nwrite == 0 && buf.len() > 0 {
                    // outbuf too small and can't conv 1 rune
                    let mut tempbuf = Vec::with_capacity(8);
                    tempbuf.extend(iter::repeat(0u8).take(8));
                    assert!(self.tempbuf.is_empty());
                    let (nread, temp_nwrite, err) = self
                        .conv
                        .convert(&self.buf[self.read_pos..self.write_pos], &mut tempbuf[..]);
                    self.read_pos += nread;
                    // here we will write 1 or 2 bytes as most.
                    // try avoiding return Ok(0)
                    let mut nwrite = 0;
                    for slot in tempbuf.iter() {
                        buf[nwrite] = *slot;
                        nwrite += 1
                    }
                    //buf.clone_from_slice(tempbuf.as_slice());
                    self.tempbuf = tempbuf[nwrite..temp_nwrite].to_vec();
                    match err {
                        EILSEQ => return Err(io::Error::new(io::ErrorKind::InvalidInput, "")),
                        _ => return Ok(nwrite),
                    }
                }
                return Ok(nwrite);
            },
            0 => {
                return Ok(nwrite);
            },
            _ => unreachable!(),
        }
    }
}

/// A ``Writer`` which does iconv convert into another Writer. not implemented yet.
#[allow(dead_code)]
pub struct IconvWriter<W> {
    inner: W,
    conv: Converter,
    buf: Vec<u8>,
    read_pos: usize,
    write_pos: usize,
    err: Option<io::Error>,
}

impl<W: Write> IconvWriter<W> {
    #[allow(dead_code)]
    pub fn new(r: W, from: &str, to: &str) -> IconvWriter<W> {
        let conv = Converter::new(from, to);
        let mut buf = Vec::with_capacity(DEFAULT_BUF_SIZE);
        buf.extend(iter::repeat(0u8).take(DEFAULT_BUF_SIZE));
        IconvWriter {
            inner: r,
            conv: conv,
            buf: buf,
            read_pos: 0,
            write_pos: 0,
            err: None,
        }
    }
}

impl<W: Write> Write for IconvWriter<W> {
    fn write(&mut self, _buf: &[u8]) -> Result<usize> {
        unimplemented!()
    }

    fn flush(&mut self) -> Result<()> {
        unimplemented!()
    }
}

// TODO: use Result<> instead of Option<> to indicate Error
pub fn convert_bytes(inbuf: &[u8], from: &str, to: &str) -> Option<Vec<u8>> {
    let converter = Converter::new(from, to);
    let mut outbuf_size = inbuf.len() * 2;
    let mut total_nread = 0;
    let mut total_nwrite = 0;

    let mut outbuf = Vec::with_capacity(outbuf_size);
    unsafe { outbuf.set_len(outbuf_size) };

    while total_nread < inbuf.len() {
        let (nread, nwrite, err) =
            converter.convert(&inbuf[total_nread..], &mut outbuf[total_nwrite..]);

        total_nread += nread;
        total_nwrite += nwrite;

        match err {
            EINVAL | EILSEQ => return None,
            E2BIG => {
                outbuf_size += inbuf.len();
                outbuf.reserve(outbuf_size);
                unsafe { outbuf.set_len(outbuf_size) };
            },
            _ => (),
        }
    }

    unsafe { outbuf.set_len(total_nwrite) };
    outbuf.shrink_to_fit();

    return Some(outbuf);
}

/// Can be encoded to bytes via iconv
pub trait IconvEncodable {
    /// Encode to bytes with encoding
    fn encode_with_encoding(&self, encoding: &str) -> Option<Vec<u8>>;
}

impl<'a> IconvEncodable for &'a [u8] {
    fn encode_with_encoding(&self, encoding: &str) -> Option<Vec<u8>> {
        convert_bytes(*self, "UTF-8", encoding)
    }
}

impl<'a> IconvEncodable for Vec<u8> {
    fn encode_with_encoding(&self, encoding: &str) -> Option<Vec<u8>> {
        convert_bytes(&self[..], "UTF-8", encoding)
    }
}

impl<'a> IconvEncodable for &'a str {
    fn encode_with_encoding(&self, encoding: &str) -> Option<Vec<u8>> {
        return self.as_bytes().encode_with_encoding(encoding);
    }
}

impl<'a> IconvEncodable for String {
    fn encode_with_encoding(&self, encoding: &str) -> Option<Vec<u8>> {
        return self.as_bytes().encode_with_encoding(encoding);
    }
}

/// Can be decoded to str via iconv
pub trait IconvDecodable {
    /// Decode to str with encoding
    fn decode_with_encoding(&self, encoding: &str) -> Option<String>;
}

impl<'a> IconvDecodable for &'a [u8] {
    fn decode_with_encoding(&self, encoding: &str) -> Option<String> {
        convert_bytes(*self, encoding, "UTF-8")
            .and_then(|bs| str::from_utf8(&bs[..]).map(|s| s.to_string()).ok())
    }
}

impl<'a> IconvDecodable for Vec<u8> {
    fn decode_with_encoding(&self, encoding: &str) -> Option<String> {
        convert_bytes(&self[..], encoding, "UTF-8")
            .and_then(|bs| str::from_utf8(&bs[..]).map(|s| s.to_string()).ok())
    }
}
