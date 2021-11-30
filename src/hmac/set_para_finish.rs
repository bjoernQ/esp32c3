#[doc = "Register `SET_PARA_FINISH` writer"]
pub struct W(crate::W<SET_PARA_FINISH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SET_PARA_FINISH_SPEC>;
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
impl From<crate::W<SET_PARA_FINISH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SET_PARA_FINISH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SET_PARA_END` writer - Finish hmac configuration."]
pub struct SET_PARA_END_W<'a> {
    w: &'a mut W,
}
impl<'a> SET_PARA_END_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - Finish hmac configuration."]
    #[inline(always)]
    pub fn set_para_end(&mut self) -> SET_PARA_END_W {
        SET_PARA_END_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Finish initial configuration.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [set_para_finish](index.html) module"]
pub struct SET_PARA_FINISH_SPEC;
impl crate::RegisterSpec for SET_PARA_FINISH_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [set_para_finish::W](W) writer structure"]
impl crate::Writable for SET_PARA_FINISH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SET_PARA_FINISH to value 0"]
impl crate::Resettable for SET_PARA_FINISH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
