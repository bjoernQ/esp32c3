#[doc = "Register `TIMER_INT2_MAP` reader"]
pub struct R(crate::R<TIMER_INT2_MAP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMER_INT2_MAP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMER_INT2_MAP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMER_INT2_MAP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMER_INT2_MAP` writer"]
pub struct W(crate::W<TIMER_INT2_MAP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMER_INT2_MAP_SPEC>;
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
impl From<crate::W<TIMER_INT2_MAP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMER_INT2_MAP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIMER_INT2_MAP` reader - reg_core0_timer_int2_map"]
pub struct TIMER_INT2_MAP_R(crate::FieldReader<u8, u8>);
impl TIMER_INT2_MAP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TIMER_INT2_MAP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER_INT2_MAP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMER_INT2_MAP` writer - reg_core0_timer_int2_map"]
pub struct TIMER_INT2_MAP_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER_INT2_MAP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - reg_core0_timer_int2_map"]
    #[inline(always)]
    pub fn timer_int2_map(&self) -> TIMER_INT2_MAP_R {
        TIMER_INT2_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - reg_core0_timer_int2_map"]
    #[inline(always)]
    pub fn timer_int2_map(&mut self) -> TIMER_INT2_MAP_W {
        TIMER_INT2_MAP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "timer2 intr map register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer_int2_map](index.html) module"]
pub struct TIMER_INT2_MAP_SPEC;
impl crate::RegisterSpec for TIMER_INT2_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timer_int2_map::R](R) reader structure"]
impl crate::Readable for TIMER_INT2_MAP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timer_int2_map::W](W) writer structure"]
impl crate::Writable for TIMER_INT2_MAP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIMER_INT2_MAP to value 0"]
impl crate::Resettable for TIMER_INT2_MAP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
