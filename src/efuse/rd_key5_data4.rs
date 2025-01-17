#[doc = "Register `RD_KEY5_DATA4` reader"]
pub struct R(crate::R<RD_KEY5_DATA4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RD_KEY5_DATA4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RD_KEY5_DATA4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RD_KEY5_DATA4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `KEY5_DATA4` reader - Stores the fourth 32 bits of KEY5."]
pub struct KEY5_DATA4_R(crate::FieldReader<u32, u32>);
impl KEY5_DATA4_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        KEY5_DATA4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for KEY5_DATA4_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Stores the fourth 32 bits of KEY5."]
    #[inline(always)]
    pub fn key5_data4(&self) -> KEY5_DATA4_R {
        KEY5_DATA4_R::new(self.bits as u32)
    }
}
#[doc = "Register 4 of BLOCK9 (KEY5).\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_key5_data4](index.html) module"]
pub struct RD_KEY5_DATA4_SPEC;
impl crate::RegisterSpec for RD_KEY5_DATA4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rd_key5_data4::R](R) reader structure"]
impl crate::Readable for RD_KEY5_DATA4_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RD_KEY5_DATA4 to value 0"]
impl crate::Resettable for RD_KEY5_DATA4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
