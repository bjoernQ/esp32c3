#[doc = "Register `RD_REPEAT_ERR4` reader"]
pub struct R(crate::R<RD_REPEAT_ERR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RD_REPEAT_ERR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RD_REPEAT_ERR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RD_REPEAT_ERR4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RPT4_RESERVED4_ERR` reader - Reserved."]
pub struct RPT4_RESERVED4_ERR_R(crate::FieldReader<u32, u32>);
impl RPT4_RESERVED4_ERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        RPT4_RESERVED4_ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RPT4_RESERVED4_ERR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:23 - Reserved."]
    #[inline(always)]
    pub fn rpt4_reserved4_err(&self) -> RPT4_RESERVED4_ERR_R {
        RPT4_RESERVED4_ERR_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
#[doc = "Programming error record register 4 of BLOCK0.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_repeat_err4](index.html) module"]
pub struct RD_REPEAT_ERR4_SPEC;
impl crate::RegisterSpec for RD_REPEAT_ERR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rd_repeat_err4::R](R) reader structure"]
impl crate::Readable for RD_REPEAT_ERR4_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RD_REPEAT_ERR4 to value 0"]
impl crate::Resettable for RD_REPEAT_ERR4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
