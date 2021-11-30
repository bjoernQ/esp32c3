#[doc = "Register `RD_REPEAT_ERR1` reader"]
pub struct R(crate::R<RD_REPEAT_ERR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RD_REPEAT_ERR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RD_REPEAT_ERR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RD_REPEAT_ERR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RPT4_RESERVED2_ERR` reader - Reserved."]
pub struct RPT4_RESERVED2_ERR_R(crate::FieldReader<u16, u16>);
impl RPT4_RESERVED2_ERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        RPT4_RESERVED2_ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RPT4_RESERVED2_ERR_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDT_DELAY_SEL_ERR` reader - If any bit in WDT_DELAY_SEL is 1, then it indicates a programming error."]
pub struct WDT_DELAY_SEL_ERR_R(crate::FieldReader<u8, u8>);
impl WDT_DELAY_SEL_ERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        WDT_DELAY_SEL_ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDT_DELAY_SEL_ERR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI_BOOT_CRYPT_CNT_ERR` reader - If any bit in SPI_BOOT_CRYPT_CNT is 1, then it indicates a programming error."]
pub struct SPI_BOOT_CRYPT_CNT_ERR_R(crate::FieldReader<u8, u8>);
impl SPI_BOOT_CRYPT_CNT_ERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SPI_BOOT_CRYPT_CNT_ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI_BOOT_CRYPT_CNT_ERR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SECURE_BOOT_KEY_REVOKE0_ERR` reader - If SECURE_BOOT_KEY_REVOKE0 is 1, then it indicates a programming error."]
pub struct SECURE_BOOT_KEY_REVOKE0_ERR_R(crate::FieldReader<bool, bool>);
impl SECURE_BOOT_KEY_REVOKE0_ERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SECURE_BOOT_KEY_REVOKE0_ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SECURE_BOOT_KEY_REVOKE0_ERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SECURE_BOOT_KEY_REVOKE1_ERR` reader - If SECURE_BOOT_KEY_REVOKE1 is 1, then it indicates a programming error."]
pub struct SECURE_BOOT_KEY_REVOKE1_ERR_R(crate::FieldReader<bool, bool>);
impl SECURE_BOOT_KEY_REVOKE1_ERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SECURE_BOOT_KEY_REVOKE1_ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SECURE_BOOT_KEY_REVOKE1_ERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SECURE_BOOT_KEY_REVOKE2_ERR` reader - If SECURE_BOOT_KEY_REVOKE2 is 1, then it indicates a programming error."]
pub struct SECURE_BOOT_KEY_REVOKE2_ERR_R(crate::FieldReader<bool, bool>);
impl SECURE_BOOT_KEY_REVOKE2_ERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SECURE_BOOT_KEY_REVOKE2_ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SECURE_BOOT_KEY_REVOKE2_ERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KEY_PURPOSE_0_ERR` reader - If any bit in KEY_PURPOSE_0 is 1, then it indicates a programming error."]
pub struct KEY_PURPOSE_0_ERR_R(crate::FieldReader<u8, u8>);
impl KEY_PURPOSE_0_ERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        KEY_PURPOSE_0_ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KEY_PURPOSE_0_ERR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `KEY_PURPOSE_1_ERR` reader - If any bit in KEY_PURPOSE_1 is 1, then it indicates a programming error."]
pub struct KEY_PURPOSE_1_ERR_R(crate::FieldReader<u8, u8>);
impl KEY_PURPOSE_1_ERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        KEY_PURPOSE_1_ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KEY_PURPOSE_1_ERR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - Reserved."]
    #[inline(always)]
    pub fn rpt4_reserved2_err(&self) -> RPT4_RESERVED2_ERR_R {
        RPT4_RESERVED2_ERR_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:17 - If any bit in WDT_DELAY_SEL is 1, then it indicates a programming error."]
    #[inline(always)]
    pub fn wdt_delay_sel_err(&self) -> WDT_DELAY_SEL_ERR_R {
        WDT_DELAY_SEL_ERR_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:20 - If any bit in SPI_BOOT_CRYPT_CNT is 1, then it indicates a programming error."]
    #[inline(always)]
    pub fn spi_boot_crypt_cnt_err(&self) -> SPI_BOOT_CRYPT_CNT_ERR_R {
        SPI_BOOT_CRYPT_CNT_ERR_R::new(((self.bits >> 18) & 0x07) as u8)
    }
    #[doc = "Bit 21 - If SECURE_BOOT_KEY_REVOKE0 is 1, then it indicates a programming error."]
    #[inline(always)]
    pub fn secure_boot_key_revoke0_err(&self) -> SECURE_BOOT_KEY_REVOKE0_ERR_R {
        SECURE_BOOT_KEY_REVOKE0_ERR_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - If SECURE_BOOT_KEY_REVOKE1 is 1, then it indicates a programming error."]
    #[inline(always)]
    pub fn secure_boot_key_revoke1_err(&self) -> SECURE_BOOT_KEY_REVOKE1_ERR_R {
        SECURE_BOOT_KEY_REVOKE1_ERR_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - If SECURE_BOOT_KEY_REVOKE2 is 1, then it indicates a programming error."]
    #[inline(always)]
    pub fn secure_boot_key_revoke2_err(&self) -> SECURE_BOOT_KEY_REVOKE2_ERR_R {
        SECURE_BOOT_KEY_REVOKE2_ERR_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 24:27 - If any bit in KEY_PURPOSE_0 is 1, then it indicates a programming error."]
    #[inline(always)]
    pub fn key_purpose_0_err(&self) -> KEY_PURPOSE_0_ERR_R {
        KEY_PURPOSE_0_ERR_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - If any bit in KEY_PURPOSE_1 is 1, then it indicates a programming error."]
    #[inline(always)]
    pub fn key_purpose_1_err(&self) -> KEY_PURPOSE_1_ERR_R {
        KEY_PURPOSE_1_ERR_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[doc = "Programming error record register 1 of BLOCK0.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_repeat_err1](index.html) module"]
pub struct RD_REPEAT_ERR1_SPEC;
impl crate::RegisterSpec for RD_REPEAT_ERR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rd_repeat_err1::R](R) reader structure"]
impl crate::Readable for RD_REPEAT_ERR1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RD_REPEAT_ERR1 to value 0"]
impl crate::Resettable for RD_REPEAT_ERR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
