#[doc = "Register `RD_USR_DATA2` reader"]
pub struct R(crate::R<RD_USR_DATA2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RD_USR_DATA2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RD_USR_DATA2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RD_USR_DATA2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `USR_DATA2` reader - Stores the second 32 bits of BLOCK3 (user)."]
pub struct USR_DATA2_R(crate::FieldReader<u32, u32>);
impl USR_DATA2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        USR_DATA2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USR_DATA2_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Stores the second 32 bits of BLOCK3 (user)."]
    #[inline(always)]
    pub fn usr_data2(&self) -> USR_DATA2_R {
        USR_DATA2_R::new(self.bits as u32)
    }
}
#[doc = "Register 2 of BLOCK3 (user).\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_usr_data2](index.html) module"]
pub struct RD_USR_DATA2_SPEC;
impl crate::RegisterSpec for RD_USR_DATA2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rd_usr_data2::R](R) reader structure"]
impl crate::Readable for RD_USR_DATA2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RD_USR_DATA2 to value 0"]
impl crate::Resettable for RD_USR_DATA2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
