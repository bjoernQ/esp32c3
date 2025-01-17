#[doc = "Register `INT_CLEAR` writer"]
pub struct W(crate::W<INT_CLEAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_CLEAR_SPEC>;
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
impl From<crate::W<INT_CLEAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_CLEAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INT_CLEAR` writer - Set this bit to clear the AES interrupt."]
pub struct INT_CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> INT_CLEAR_W<'a> {
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
    #[doc = "Bit 0 - Set this bit to clear the AES interrupt."]
    #[inline(always)]
    pub fn int_clear(&mut self) -> INT_CLEAR_W {
        INT_CLEAR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AES Interrupt clear register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_clear](index.html) module"]
pub struct INT_CLEAR_SPEC;
impl crate::RegisterSpec for INT_CLEAR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [int_clear::W](W) writer structure"]
impl crate::Writable for INT_CLEAR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INT_CLEAR to value 0"]
impl crate::Resettable for INT_CLEAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
