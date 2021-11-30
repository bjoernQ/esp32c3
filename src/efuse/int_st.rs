#[doc = "Register `INT_ST` reader"]
pub struct R(crate::R<INT_ST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_ST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_ST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_ST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `READ_DONE_INT_ST` reader - The status signal for read_done interrupt."]
pub struct READ_DONE_INT_ST_R(crate::FieldReader<bool, bool>);
impl READ_DONE_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        READ_DONE_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for READ_DONE_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PGM_DONE_INT_ST` reader - The status signal for pgm_done interrupt."]
pub struct PGM_DONE_INT_ST_R(crate::FieldReader<bool, bool>);
impl PGM_DONE_INT_ST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PGM_DONE_INT_ST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PGM_DONE_INT_ST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - The status signal for read_done interrupt."]
    #[inline(always)]
    pub fn read_done_int_st(&self) -> READ_DONE_INT_ST_R {
        READ_DONE_INT_ST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - The status signal for pgm_done interrupt."]
    #[inline(always)]
    pub fn pgm_done_int_st(&self) -> PGM_DONE_INT_ST_R {
        PGM_DONE_INT_ST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
#[doc = "eFuse interrupt status register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_st](index.html) module"]
pub struct INT_ST_SPEC;
impl crate::RegisterSpec for INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_st::R](R) reader structure"]
impl crate::Readable for INT_ST_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INT_ST to value 0"]
impl crate::Resettable for INT_ST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}