#[doc = "Register `LSCH5_CONF1` reader"]
pub struct R(crate::R<LSCH5_CONF1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LSCH5_CONF1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LSCH5_CONF1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LSCH5_CONF1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LSCH5_CONF1` writer"]
pub struct W(crate::W<LSCH5_CONF1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LSCH5_CONF1_SPEC>;
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
impl From<crate::W<LSCH5_CONF1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LSCH5_CONF1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DUTY_SCALE_LSCH5` reader - reg_duty_scale_lsch5."]
pub struct DUTY_SCALE_LSCH5_R(crate::FieldReader<u16, u16>);
impl DUTY_SCALE_LSCH5_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        DUTY_SCALE_LSCH5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DUTY_SCALE_LSCH5_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DUTY_SCALE_LSCH5` writer - reg_duty_scale_lsch5."]
pub struct DUTY_SCALE_LSCH5_W<'a> {
    w: &'a mut W,
}
impl<'a> DUTY_SCALE_LSCH5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | (value as u32 & 0x03ff);
        self.w
    }
}
#[doc = "Field `DUTY_CYCLE_LSCH5` reader - reg_duty_cycle_lsch5."]
pub struct DUTY_CYCLE_LSCH5_R(crate::FieldReader<u16, u16>);
impl DUTY_CYCLE_LSCH5_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        DUTY_CYCLE_LSCH5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DUTY_CYCLE_LSCH5_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DUTY_CYCLE_LSCH5` writer - reg_duty_cycle_lsch5."]
pub struct DUTY_CYCLE_LSCH5_W<'a> {
    w: &'a mut W,
}
impl<'a> DUTY_CYCLE_LSCH5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 10)) | ((value as u32 & 0x03ff) << 10);
        self.w
    }
}
#[doc = "Field `DUTY_NUM_LSCH5` reader - reg_duty_num_lsch5."]
pub struct DUTY_NUM_LSCH5_R(crate::FieldReader<u16, u16>);
impl DUTY_NUM_LSCH5_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        DUTY_NUM_LSCH5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DUTY_NUM_LSCH5_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DUTY_NUM_LSCH5` writer - reg_duty_num_lsch5."]
pub struct DUTY_NUM_LSCH5_W<'a> {
    w: &'a mut W,
}
impl<'a> DUTY_NUM_LSCH5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 20)) | ((value as u32 & 0x03ff) << 20);
        self.w
    }
}
#[doc = "Field `DUTY_INC_LSCH5` reader - reg_duty_inc_lsch5."]
pub struct DUTY_INC_LSCH5_R(crate::FieldReader<bool, bool>);
impl DUTY_INC_LSCH5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DUTY_INC_LSCH5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DUTY_INC_LSCH5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DUTY_INC_LSCH5` writer - reg_duty_inc_lsch5."]
pub struct DUTY_INC_LSCH5_W<'a> {
    w: &'a mut W,
}
impl<'a> DUTY_INC_LSCH5_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `DUTY_START_LSCH5` reader - reg_duty_start_lsch5."]
pub struct DUTY_START_LSCH5_R(crate::FieldReader<bool, bool>);
impl DUTY_START_LSCH5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DUTY_START_LSCH5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DUTY_START_LSCH5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DUTY_START_LSCH5` writer - reg_duty_start_lsch5."]
pub struct DUTY_START_LSCH5_W<'a> {
    w: &'a mut W,
}
impl<'a> DUTY_START_LSCH5_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - reg_duty_scale_lsch5."]
    #[inline(always)]
    pub fn duty_scale_lsch5(&self) -> DUTY_SCALE_LSCH5_R {
        DUTY_SCALE_LSCH5_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:19 - reg_duty_cycle_lsch5."]
    #[inline(always)]
    pub fn duty_cycle_lsch5(&self) -> DUTY_CYCLE_LSCH5_R {
        DUTY_CYCLE_LSCH5_R::new(((self.bits >> 10) & 0x03ff) as u16)
    }
    #[doc = "Bits 20:29 - reg_duty_num_lsch5."]
    #[inline(always)]
    pub fn duty_num_lsch5(&self) -> DUTY_NUM_LSCH5_R {
        DUTY_NUM_LSCH5_R::new(((self.bits >> 20) & 0x03ff) as u16)
    }
    #[doc = "Bit 30 - reg_duty_inc_lsch5."]
    #[inline(always)]
    pub fn duty_inc_lsch5(&self) -> DUTY_INC_LSCH5_R {
        DUTY_INC_LSCH5_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - reg_duty_start_lsch5."]
    #[inline(always)]
    pub fn duty_start_lsch5(&self) -> DUTY_START_LSCH5_R {
        DUTY_START_LSCH5_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - reg_duty_scale_lsch5."]
    #[inline(always)]
    pub fn duty_scale_lsch5(&mut self) -> DUTY_SCALE_LSCH5_W {
        DUTY_SCALE_LSCH5_W { w: self }
    }
    #[doc = "Bits 10:19 - reg_duty_cycle_lsch5."]
    #[inline(always)]
    pub fn duty_cycle_lsch5(&mut self) -> DUTY_CYCLE_LSCH5_W {
        DUTY_CYCLE_LSCH5_W { w: self }
    }
    #[doc = "Bits 20:29 - reg_duty_num_lsch5."]
    #[inline(always)]
    pub fn duty_num_lsch5(&mut self) -> DUTY_NUM_LSCH5_W {
        DUTY_NUM_LSCH5_W { w: self }
    }
    #[doc = "Bit 30 - reg_duty_inc_lsch5."]
    #[inline(always)]
    pub fn duty_inc_lsch5(&mut self) -> DUTY_INC_LSCH5_W {
        DUTY_INC_LSCH5_W { w: self }
    }
    #[doc = "Bit 31 - reg_duty_start_lsch5."]
    #[inline(always)]
    pub fn duty_start_lsch5(&mut self) -> DUTY_START_LSCH5_W {
        DUTY_START_LSCH5_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LEDC_LSCH5_CONF1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lsch5_conf1](index.html) module"]
pub struct LSCH5_CONF1_SPEC;
impl crate::RegisterSpec for LSCH5_CONF1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lsch5_conf1::R](R) reader structure"]
impl crate::Readable for LSCH5_CONF1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lsch5_conf1::W](W) writer structure"]
impl crate::Writable for LSCH5_CONF1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LSCH5_CONF1 to value 0x4000_0000"]
impl crate::Resettable for LSCH5_CONF1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x4000_0000
    }
}
