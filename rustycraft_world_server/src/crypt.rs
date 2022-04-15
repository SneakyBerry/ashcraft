use crate::constants::SERVER_PRIVATE_KEY;
use boring_sys::{
    EVP_CIPHER_CTX_ctrl, EVP_CIPHER_CTX_init, EVP_CIPHER_CTX_new, EVP_CipherFinal_ex,
    EVP_CipherInit_ex, EVP_CipherUpdate, EVP_aes_128_gcm, EVP_CTRL_GCM_GET_TAG,
    EVP_CTRL_GCM_SET_TAG,
};
use bytes::Bytes;
use std::ffi::c_void;
use std::os::raw::{c_int, c_uint};

pub struct RSA {
    c_rsa: *mut boring_sys::RSA,
}

lazy_static! {
    pub static ref INITIALIZED_RSA: RSA = RSA::new();
}

impl RSA {
    fn new() -> RSA {
        unsafe {
            let mut rsa = boring_sys::RSA_new();
            let rsa_pk: *const c_void = std::mem::transmute(SERVER_PRIVATE_KEY);
            let pk_bio = boring_sys::BIO_new_mem_buf(rsa_pk, SERVER_PRIVATE_KEY.len() as i32);
            boring_sys::PEM_read_bio_RSAPrivateKey(
                pk_bio,
                &mut rsa as *mut *mut boring_sys::RSA,
                None,
                std::ptr::null_mut(),
            );
            RSA { c_rsa: rsa }
        }
    }

    pub fn sign(&self, data: &[u8]) -> Vec<u8> {
        let mut signature = vec![0; 256];
        let mut siglen = 0 as c_uint;
        unsafe {
            boring_sys::RSA_sign(
                boring_sys::NID_sha256,
                data.as_ptr(),
                data.len() as c_uint,
                signature.as_mut_ptr(),
                &mut siglen as *mut c_uint,
                self.c_rsa,
            );
        };
        signature.reverse();
        signature
    }
}
unsafe impl Send for RSA {}
unsafe impl Sync for RSA {}

#[derive(Debug)]
pub struct EncryptionResult {
    pub cipher_text: Bytes,
    pub aes_tag: Vec<u8>,
}

#[derive(Debug)]
pub struct AES128 {
    ctx: *mut boring_sys::EVP_CIPHER_CTX,
    magic: u32,
    counter: u64,
    initialized: bool,
    encrypting: bool,
}

impl AES128 {
    fn new(encrypting: bool, magic: u32) -> AES128 {
        unsafe {
            let ctx = EVP_CIPHER_CTX_new();
            EVP_CIPHER_CTX_init(ctx);
            let res = EVP_CipherInit_ex(
                ctx,
                EVP_aes_128_gcm(),
                std::ptr::null_mut(),
                std::ptr::null(),
                std::ptr::null(),
                {
                    if encrypting {
                        1
                    } else {
                        0
                    }
                },
            );
            assert_ne!(res, 0 as c_int);
            AES128 {
                ctx,
                magic,
                counter: 0,
                initialized: false,
                encrypting,
            }
        }
    }

    fn get_iv(&self) -> Vec<u8> {
        let mut iv = vec![];
        iv.extend(self.counter.to_le_bytes());
        iv.extend(self.magic.to_le_bytes());
        iv
    }

    fn init(&mut self, key: &[u8]) -> anyhow::Result<()> {
        unsafe {
            let res = EVP_CipherInit_ex(
                self.ctx,
                std::ptr::null(),
                std::ptr::null_mut(),
                key.as_ptr(),
                std::ptr::null(),
                -1,
            );
            assert_ne!(res, 0);
        };
        self.initialized = true;
        Ok(())
    }

    fn decrypt(&mut self, data: &[u8], tag: &[u8]) -> anyhow::Result<Vec<u8>> {
        assert_eq!(self.encrypting, false);
        let out = if self.initialized {
            let mut tag = tag.to_vec();
            let len = data.len() as i32;
            let mut out = data.clone().to_vec();
            let mut _out_len = 0;
            let iv = self.get_iv();
            unsafe {
                let res = EVP_CipherInit_ex(
                    self.ctx,
                    std::ptr::null(),
                    std::ptr::null_mut(),
                    std::ptr::null(),
                    iv.as_ptr(),
                    -1,
                );
                assert_ne!(res, 0);
                let res = EVP_CipherUpdate(
                    self.ctx,
                    out.as_mut_ptr(),
                    &mut _out_len as *mut c_int,
                    data.as_ptr(),
                    len as c_int,
                );
                assert_ne!(res, 0);
                let res = EVP_CIPHER_CTX_ctrl(
                    self.ctx,
                    EVP_CTRL_GCM_SET_TAG,
                    tag.len() as c_int,
                    tag.as_mut_ptr() as *mut c_void,
                );
                assert_ne!(res, 0);

                let res = EVP_CipherFinal_ex(
                    self.ctx,
                    out.as_mut_ptr().offset(_out_len as isize),
                    &mut _out_len as *mut c_int,
                );
                assert_ne!(res, 0);
            };
            out
        } else {
            data.to_vec()
        };
        self.counter += 1;
        Ok(out)
    }

    fn encrypt(&mut self, data: &[u8]) -> anyhow::Result<EncryptionResult> {
        assert_eq!(self.encrypting, true);
        let mut tag = vec![0; 12];
        let out = if self.initialized {
            let len = data.len() as i32;
            let mut out = Vec::with_capacity(data.len());
            out.extend((0..data.len()).map(|_| 0));
            let mut _out_len = 0;
            let iv = self.get_iv();
            unsafe {
                let res = EVP_CipherInit_ex(
                    self.ctx,
                    std::ptr::null(),
                    std::ptr::null_mut(),
                    std::ptr::null(),
                    iv.as_ptr(),
                    -1,
                );
                assert_ne!(res, 0);
                let res = EVP_CipherUpdate(
                    self.ctx,
                    out.as_mut_ptr(),
                    &mut _out_len as *mut c_int,
                    data.as_ptr(),
                    len as c_int,
                );
                assert_ne!(res, 0);

                let res = EVP_CipherFinal_ex(
                    self.ctx,
                    out.as_mut_ptr().offset(_out_len as isize),
                    &mut _out_len as *mut c_int,
                );
                assert_ne!(res, 0);

                let res = EVP_CIPHER_CTX_ctrl(
                    self.ctx,
                    EVP_CTRL_GCM_GET_TAG,
                    tag.len() as c_int,
                    tag.as_mut_ptr() as *mut c_void,
                );
                assert_ne!(res, 0);
                out
            }
        } else {
            data.to_vec()
        };

        self.counter += 1;
        Ok(EncryptionResult {
            cipher_text: out.into(),
            aes_tag: tag,
        })
    }
}

pub struct AES128Companion {
    server_encrypt: AES128,
    client_decrypt: AES128,
    initialized: bool,
}

impl AES128Companion {
    pub fn new() -> AES128Companion {
        AES128Companion {
            server_encrypt: AES128::new(true, 0x52565253),
            client_decrypt: AES128::new(false, 0x544E4C43),
            initialized: false,
        }
    }

    pub fn init(&mut self, key: &[u8]) -> anyhow::Result<()> {
        assert_eq!(key.len(), 16);
        self.server_encrypt.init(key)?;
        self.client_decrypt.init(key)?;
        self.initialized = true;
        Ok(())
    }

    pub fn decrypt(&mut self, data: &[u8], tag: &[u8]) -> anyhow::Result<Vec<u8>> {
        self.client_decrypt.decrypt(data, tag)
    }

    pub fn encrypt(&mut self, data: &[u8]) -> anyhow::Result<EncryptionResult> {
        self.server_encrypt.encrypt(data)
    }
}

unsafe impl Send for AES128 {}
unsafe impl Sync for AES128 {}
