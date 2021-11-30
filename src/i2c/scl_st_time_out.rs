#[doc = "Register `SCL_ST_TIME_OUT` reader"]
pub struct R(crate::R<SCL_ST_TIME_OUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCL_ST_TIME_OUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCL_ST_TIME_OUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCL_ST_TIME_OUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCL_ST_TIME_OUT` writer"]
pub struct W(crate::W<SCL_ST_TIME_OUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCL_ST_TIME_OUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<SCL_ST_TIME_OUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCL_ST_TIME_OUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCL_ST_TO_I2C` reader - reg_scl_st_to_regno more than 23"]
pub struct SCL_ST_TO_I2C_R(crate::FieldReader<u8, u8>);
impl SCL_ST_TO_I2C_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SCL_ST_TO_I2C_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCL_ST_TO_I2C_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCL_ST_TO_I2C` writer - reg_scl_st_to_regno more than 23"]
pub struct SCL_ST_TO_I2C_W<'a> {
    w: &'a mut W,
}
impl<'a> SCL_ST_TO_I2C_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - reg_scl_st_to_regno more than 23"]
    #[inline(always)]
    pub fn scl_st_to_i2c(&self) -> SCL_ST_TO_I2C_R {
        SCL_ST_TO_I2C_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - reg_scl_st_to_regno more than 23"]
    #[inline(always)]
    pub fn scl_st_to_i2c(&mut self) -> SCL_ST_TO_I2C_W {
        SCL_ST_TO_I2C_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C_SCL_ST_TIME_OUT_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scl_st_time_out](index.html) module"]
pub struct SCL_ST_TIME_OUT_SPEC;
impl crate::RegisterSpec for SCL_ST_TIME_OUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scl_st_time_out::R](R) reader structure"]
impl crate::Readable for SCL_ST_TIME_OUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scl_st_time_out::W](W) writer structure"]
impl crate::Writable for SCL_ST_TIME_OUT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SCL_ST_TIME_OUT to value 0x10"]
impl crate::Resettable for SCL_ST_TIME_OUT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x10
    }
}